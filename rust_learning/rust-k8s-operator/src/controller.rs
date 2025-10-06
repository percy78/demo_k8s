use crate::crd::{AppManager, AppManagerStatus};
use anyhow::Result;
use k8s_openapi::api::apps::v1::{Deployment, DeploymentSpec};
use k8s_openapi::api::core::v1::{
    Container, ContainerPort, EnvVar, PodSpec, PodTemplateSpec, ResourceRequirements as K8sResourceRequirements,
};
use k8s_openapi::apimachinery::pkg::apis::meta::v1::{LabelSelector, ObjectMeta};
use kube::{
    api::{Api, Patch, PatchParams, PostParams},
    client::Client,
    runtime::{controller::Action, watcher::Config, Controller},
    Resource, ResourceExt,
};
use serde_json::json;
use std::{collections::BTreeMap, sync::Arc, time::Duration};
use thiserror::Error;
use tracing::{error, info, instrument, warn};

#[derive(Error, Debug)]
pub enum ControllerError {
    #[error("Kubernetes API error: {0}")]
    KubeError(#[from] kube::Error),
    #[error("JSON serialization error: {0}")]
    JsonError(#[from] serde_json::Error),
    #[error("Missing object key: {0}")]
    MissingObjectKey(&'static str),
}

pub type ControllerResult<T> = std::result::Result<T, ControllerError>;

/// Context for the controller
#[derive(Clone)]
pub struct Context {
    pub client: Client,
}

/// Main reconciliation function
#[instrument(skip(app_manager, ctx), fields(name = %app_manager.name_any()))]
pub async fn reconcile(app_manager: Arc<AppManager>, ctx: Arc<Context>) -> ControllerResult<Action> {
    let client = &ctx.client;
    let name = app_manager.name_any();
    let namespace = app_manager.namespace().unwrap_or_else(|| "default".to_string());
    
    info!("Reconciling AppManager {}/{}", namespace, name);

    // Create or update deployment
    match create_or_update_deployment(&app_manager, client, &namespace).await {
        Ok(_) => {
            info!("Successfully reconciled deployment for {}/{}", namespace, name);
            
            // Update status
            update_status(&app_manager, client, &namespace, "Running", "Deployment updated successfully").await?;
            
            // Requeue after 5 minutes for status updates
            Ok(Action::requeue(Duration::from_secs(300)))
        }
        Err(e) => {
            error!("Failed to reconcile deployment for {}/{}: {}", namespace, name, e);
            
            // Update status with error
            update_status(&app_manager, client, &namespace, "Error", &format!("Failed to update deployment: {}", e)).await?;
            
            // Requeue after 1 minute on error
            Ok(Action::requeue(Duration::from_secs(60)))
        }
    }
}

/// Error handler for the controller
pub fn error_policy(app_manager: Arc<AppManager>, error: &ControllerError, _ctx: Arc<Context>) -> Action {
    warn!("reconcile failed for AppManager {}: {:?}", app_manager.name_any(), error);
    Action::requeue(Duration::from_secs(60))
}

/// Create or update the deployment for the AppManager
async fn create_or_update_deployment(
    app_manager: &AppManager,
    client: &Client,
    namespace: &str,
) -> ControllerResult<()> {
    let deployment_api: Api<Deployment> = Api::namespaced(client.clone(), namespace);
    let name = &app_manager.spec.app_name;

    // Build deployment spec
    let deployment = build_deployment(app_manager, namespace)?;

    // Try to create or update the deployment
    match deployment_api.get(name).await {
        Ok(_) => {
            // Deployment exists, update it
            info!("Updating existing deployment: {}", name);
            deployment_api
                .patch(
                    name,
                    &PatchParams::apply("k8s-operator"),
                    &Patch::Apply(&deployment),
                )
                .await?;
        }
        Err(kube::Error::Api(err)) if err.code == 404 => {
            // Deployment doesn't exist, create it
            info!("Creating new deployment: {}", name);
            deployment_api.create(&PostParams::default(), &deployment).await?;
        }
        Err(e) => return Err(ControllerError::KubeError(e)),
    }

    Ok(())
}

/// Build a Kubernetes Deployment from AppManager spec
fn build_deployment(app_manager: &AppManager, namespace: &str) -> ControllerResult<Deployment> {
    let name = &app_manager.spec.app_name;
    let mut labels = BTreeMap::new();
    labels.insert("app".to_string(), name.clone());
    labels.insert("managed-by".to_string(), "k8s-operator".to_string());

    // Convert environment variables
    let env_vars: Vec<EnvVar> = app_manager
        .spec
        .env_vars
        .iter()
        .map(|(key, value)| EnvVar {
            name: key.clone(),
            value: Some(value.clone()),
            value_from: None,
        })
        .collect();

    // Resource requirements
    let mut requests = BTreeMap::new();
    requests.insert("cpu".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity(app_manager.spec.resources.cpu.clone()));
    requests.insert("memory".to_string(), k8s_openapi::apimachinery::pkg::api::resource::Quantity(app_manager.spec.resources.memory.clone()));

    let container = Container {
        name: name.clone(),
        image: Some(app_manager.spec.image.clone()),
        ports: Some(vec![ContainerPort {
            container_port: 8080,
            ..Default::default()
        }]),
        env: Some(env_vars),
        resources: Some(K8sResourceRequirements {
            requests: Some(requests),
            ..Default::default()
        }),
        ..Default::default()
    };

    let deployment = Deployment {
        metadata: ObjectMeta {
            name: Some(name.clone()),
            namespace: Some(namespace.to_string()),
            labels: Some(labels.clone()),
            owner_references: Some(vec![app_manager.controller_owner_ref(&()).unwrap()]),
            ..Default::default()
        },
        spec: Some(DeploymentSpec {
            replicas: Some(app_manager.spec.replicas),
            selector: LabelSelector {
                match_labels: Some(labels.clone()),
                ..Default::default()
            },
            template: PodTemplateSpec {
                metadata: Some(ObjectMeta {
                    labels: Some(labels),
                    ..Default::default()
                }),
                spec: Some(PodSpec {
                    containers: vec![container],
                    ..Default::default()
                }),
            },
            ..Default::default()
        }),
        ..Default::default()
    };

    Ok(deployment)
}

/// Update the status of the AppManager resource
async fn update_status(
    app_manager: &AppManager,
    client: &Client,
    namespace: &str,
    phase: &str,
    message: &str,
) -> ControllerResult<()> {
    let api: Api<AppManager> = Api::namespaced(client.clone(), namespace);
    let name = app_manager.name_any();

    let status = AppManagerStatus {
        phase: Some(phase.to_string()),
        message: Some(message.to_string()),
        last_updated: Some(chrono::Utc::now()),
        ready_replicas: Some(0), // TODO: Get actual ready replicas from deployment
    };

    let patch = json!({
        "status": status
    });

    api.patch_status(
        &name,
        &PatchParams::default(),
        &Patch::Merge(&patch),
    ).await?;

    Ok(())
}

/// Run the controller
pub async fn run_controller(client: Client) -> Result<()> {
    let app_managers: Api<AppManager> = Api::all(client.clone());
    
    info!("Starting AppManager controller");
    
    Controller::new(app_managers, Config::default())
        .run(reconcile, error_policy, Arc::new(Context { client }))
        .for_each(|res| async move {
            match res {
                Ok(_) => {},
                Err(e) => error!("Controller error: {:?}", e),
            }
        })
        .await;

    Ok(())
}