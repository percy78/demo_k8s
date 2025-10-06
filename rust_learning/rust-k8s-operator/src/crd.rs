use kube::CustomResource;
use schemars::JsonSchema;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;

/// Our custom resource for managing applications
#[derive(CustomResource, Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[kube(
    group = "operator.telia.com",
    version = "v1",
    kind = "AppManager",
    namespaced,
    status = "AppManagerStatus",
    derive = "Default"
)]
#[serde(rename_all = "camelCase")]
pub struct AppManagerSpec {
    /// Name of the application to manage
    pub app_name: String,
    
    /// Number of replicas
    #[serde(default = "default_replicas")]
    pub replicas: i32,
    
    /// Docker image to deploy
    pub image: String,
    
    /// Environment variables
    #[serde(default)]
    pub env_vars: BTreeMap<String, String>,
    
    /// Resource requirements
    #[serde(default)]
    pub resources: ResourceRequirements,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema, Default)]
#[serde(rename_all = "camelCase")]
pub struct ResourceRequirements {
    /// CPU request (e.g., "100m")
    #[serde(default = "default_cpu")]
    pub cpu: String,
    
    /// Memory request (e.g., "128Mi")
    #[serde(default = "default_memory")]
    pub memory: String,
}

#[derive(Deserialize, Serialize, Clone, Debug, JsonSchema)]
#[serde(rename_all = "camelCase")]
pub struct AppManagerStatus {
    /// Current number of ready replicas
    pub ready_replicas: Option<i32>,
    
    /// Current phase of the application
    pub phase: Option<String>,
    
    /// Last update timestamp
    pub last_updated: Option<chrono::DateTime<chrono::Utc>>,
    
    /// Status message
    pub message: Option<String>,
}

impl Default for AppManagerStatus {
    fn default() -> Self {
        Self {
            ready_replicas: Some(0),
            phase: Some("Pending".to_string()),
            last_updated: Some(chrono::Utc::now()),
            message: None,
        }
    }
}

fn default_replicas() -> i32 {
    1
}

fn default_cpu() -> String {
    "100m".to_string()
}

fn default_memory() -> String {
    "128Mi".to_string()
}