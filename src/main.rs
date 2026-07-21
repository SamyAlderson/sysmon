// src/main.rs

// Import necessary dependencies
use clap::{App, Arg};
use log::{info, error};
use serde_json::json;
use sysmon::config::Config;
use sysmon::metrics::Metrics;
use sysmon::alert::Alert;
use sysmon::monitor::Monitor;

// Define the main entry point
fn main() {
    // Initialize the logger
    env_logger::init();

    // Define the CLI app
    let matches = App::new("sysmon")
        .version("1.0")
        .author("Samy Alderson")
        .about("System monitor CLI tool that provides real-time system metrics and alerts")
        .arg(Arg::with_name("config")
            .long("config")
            .short("c")
            .help("Path to the configuration file")
            .takes_value(true))
        .arg(Arg::with_name("alert_threshold")
            .long("alert_threshold")
            .short("t")
            .help("Alert threshold value")
            .takes_value(true))
        .arg(Arg::with_name("metric_interval")
            .long("metric_interval")
            .short("i")
            .help("Metric collection interval in seconds")
            .takes_value(true))
        .get_matches();

    // Get the configuration file path
    let config_file_path = matches.value_of("config").unwrap_or("config.json");

    // Get the alert threshold value
    let alert_threshold: f64 = matches.value_of("alert_threshold").unwrap_or("50.0").parse().unwrap();

    // Get the metric collection interval in seconds
    let metric_interval: u64 = matches.value_of("metric_interval").unwrap_or("1").parse().unwrap();

    // Load the configuration
    let config = Config::load(config_file_path).unwrap_or_else(|e| {
        error!("Error loading configuration: {}", e);
        std::process::exit(1);
    });

    // Initialize the metrics collection
    let metrics = Metrics::new(config);

    // Initialize the monitoring
    let monitor = Monitor::new(config);

    // Initialize the alerting
    let alert = Alert::new(config, alert_threshold);

    // Start the metric collection loop
    info!("Starting metric collection loop");
    loop {
        // Collect metrics
        let metrics = metrics.collect();

        // Check for alerts
        let alerts = alert.check_metrics(metrics);

        // Print metrics and alerts
        println!("Metrics: {:?}", metrics);
        println!("Alerts: {:?}", alerts);

        // Sleep for the metric collection interval
        std::thread::sleep(std::time::Duration::from_secs(metric_interval));
    }
}