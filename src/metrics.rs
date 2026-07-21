//! System metric collection module

use std::fs;
use std::io;
use std::path::PathBuf;

use log::info;
use serde_json::{json, Value};
use thiserror::Error;

use crate::config::Config;
use crate::monitor::Monitor;

/// Collects system metrics from various sources
pub struct Metrics {
    config: Config,
    monitor: Monitor,
}

impl Metrics {
    /// Creates a new `Metrics` instance
    pub fn new(config: Config, monitor: Monitor) -> Self {
        Self { config, monitor }
    }

    /// Collects metrics from the system
    ///
    /// This function fetches metrics from various sources, including CPU, memory, disk, and network.
    /// It returns a `Value` containing the collected metrics.
    ///
    /// # Errors
    ///
    /// Returns a `MetricsError` if there's an issue collecting metrics.
    pub async fn collect_metrics(&self) -> Result<Value, MetricsError> {
        let mut metrics = Value::Object(Default::default());
        let mut errors = Vec::new();

        // CPU metrics
        info!("Collecting CPU metrics");
        let cpu_metrics = self.monitor.cpu_metrics().await?;
        metrics.insert("cpu".to_string(), cpu_metrics);

        // Memory metrics
        info!("Collecting memory metrics");
        let mem_metrics = self.monitor.memory_metrics().await?;
        metrics.insert("memory".to_string(), mem_metrics);

        // Disk metrics
        info!("Collecting disk metrics");
        let disk_metrics = self.monitor.disk_metrics().await?;
        metrics.insert("disk".to_string(), disk_metrics);

        // Network metrics
        info!("Collecting network metrics");
        let net_metrics = self.monitor.net_metrics().await?;
        metrics.insert("network".to_string(), net_metrics);

        Ok(metrics)
    }
}

/// Error type for metrics collection
#[derive(Debug, Error)]
pub enum MetricsError {
    #[error("Failed to collect metrics: {0}")]
    CollectionError(String),
    #[error("Failed to parse metrics: {0}")]
    ParseError(serde_json::Error),
}

impl Metrics {
    /// Reads the metrics configuration from the `metrics.json` file
    pub fn read_config(path: &PathBuf) -> Result<Config, io::Error> {
        let contents = fs::read_to_string(path)?;
        serde_json::from_str(&contents).map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))
    }
}