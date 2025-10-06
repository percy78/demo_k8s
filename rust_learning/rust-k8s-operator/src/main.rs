mod controller;
mod crd;

use anyhow::Result;
use kube::Client;
use tracing::{info, Level};
use tracing_subscriber::{EnvFilter, FmtSubscriber};

#[tokio::main]
async fn main() -> Result<()> {
    // Initialize tracing
    let filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new("info"));
    
    let subscriber = FmtSubscriber::builder()
        .with_env_filter(filter)
        .with_target(false)
        .with_level(true)
        .with_ansi(true)
        .finish();
    
    tracing::subscriber::set_global_default(subscriber)?;

    info!("Starting Kubernetes Operator");

    // Create Kubernetes client
    let client = Client::try_default().await?;
    
    info!("Connected to Kubernetes cluster");

    // Run the controller
    controller::run_controller(client).await?;

    Ok(())
}
