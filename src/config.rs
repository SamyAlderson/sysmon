// Configuration management
pub mod config {
    use serde::{Deserialize, Serialize};

    /// Configuration options for the system monitor CLI tool.
    #[derive(Deserialize, Serialize, Debug, PartialEq)]
    pub struct SysmonConfig {
        /// Logging level (debug, info, warn, error).
        pub log_level: String,
        /// Alerting mechanism (none, email, slack).
        pub alert: Option<String>,
        /// Monitoring interval in seconds.
        pub interval: u64,
        /// Metric collection interval in seconds.
        pub metric_interval: u64,
        /// System metrics to collect (cpu, memory, disk, network).
        pub metrics: Vec<String>,
    }

    impl SysmonConfig {
        /// Load configuration from a JSON file.
        ///
        /// This function uses the `serde` crate to deserialize the JSON file
        /// into a `SysmonConfig` instance. If the file does not exist or cannot
        /// be read, an error is returned.
        pub fn load(file_path: &str) -> Result<Self, std::io::Error> {
            let config_str = std::fs::read_to_string(file_path)?;
            let config: Self = serde_json::from_str(&config_str)?;
            Ok(config)
        }

        /// Validate configuration options.
        ///
        /// This function checks that the configuration options are valid and
        /// return an error if they are not.
        pub fn validate(&self) -> Result<(), String> {
            if self.log_level != "debug" && self.log_level != "info" && self.log_level != "warn" && self.log_level != "error" {
                return Err("Invalid log level".to_string());
            }
            if self.interval == 0 {
                return Err("Monitoring interval must be greater than 0".to_string());
            }
            if self.metric_interval == 0 {
                return Err("Metric collection interval must be greater than 0".to_string());
            }
            if self.metrics.is_empty() {
                return Err("Must specify at least one system metric".to_string());
            }
            Ok(())
        }
    }

    #[cfg(test)]
    mod tests {
        use super::*;

        #[test]
        fn test_load_config() {
            let config = SysmonConfig::load("src/config.json").unwrap();
            assert_eq!(config.log_level, "debug");
            assert_eq!(config.interval, 10);
            assert_eq!(config.metric_interval, 5);
            assert_eq!(config.metrics.len(), 4);
        }

        #[test]
        fn test_validate_config() {
            let mut config = SysmonConfig::default();
            config.log_level = "invalid".to_string();
            let err = config.validate().unwrap_err();
            assert_eq!(err, "Invalid log level");
            config.log_level = "error".to_string();
            config.interval = 0;
            let err = config.validate().unwrap_err();
            assert_eq!(err, "Monitoring interval must be greater than 0");
            config.interval = 10;
            config.metric_interval = 0;
            let err = config.validate().unwrap_err();
            assert_eq!(err, "Metric collection interval must be greater than 0");
            config.metric_interval = 5;
            config.metrics.clear();
            let err = config.validate().unwrap_err();
            assert_eq!(err, "Must specify at least one system metric");
        }
    }
}