// Alerting mechanism
// ===============

use std::time::Instant;
use serde_json::json;
use log::{error, info};
use crate::config::{Config, Alert};

pub struct Alerting {
    config: Config,
}

impl Alerting {
    pub fn new(config: Config) -> Self {
        Self { config }
    }

    // Trigger an alert based on the given condition
    // This was a tricky part, we need to handle the async nature of the condition
    // and the potential errors when logging the alert
    pub async fn trigger_alert(
        &self,
        condition: bool,
        message: &str,
    ) -> Result<(), String> {
        if condition {
            // Get the current time to include in the log message
            let now = Instant::now().elapsed().as_secs();
            // Construct the log message
            let log_message = format!("ALERT: {} ({})", message, now);
            // Log the alert
            info!("{}", log_message);
            // If we're in debug mode, log the complete alert JSON
            if self.config.debug {
                let alert_json = json!({
                    "message": message,
                    "timestamp": now,
                });
                error!("ALERT JSON: {:?}", alert_json);
            }
            Ok(())
        } else {
            // If the condition is false, we don't log anything
            Ok(())
        }
    }
}

impl From<Config> for Alerting {
    fn from(config: Config) -> Self {
        Self::new(config)
    }
}

// Alerting trait for the Config struct
pub trait AlertingTrait {
    fn alerting(&self) -> &Alerting;
}

impl AlertingTrait for Config {
    fn alerting(&self) -> &Alerting {
        &self.alerting
    }
}