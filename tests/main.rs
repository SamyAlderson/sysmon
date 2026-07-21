// tests/main.rs

use sysmon::{main, Config};
use clap::{App, Arg};
use log::{error, info};
use std::process;

fn main() {
    // Load configuration from file
    let config = Config::load();

    // Parse command-line arguments
    let matches = App::new("sysmon")
        .version("1.0")
        .author("Samy Alderson")
        .about("System monitor CLI tool")
        .arg(
            Arg::with_name("config")
                .long("config")
                .help("Path to configuration file")
                .required(true),
        )
        .get_matches();

    // Check if configuration file exists
    if !std::path::Path::new(matches.value_of("config").unwrap()).exists() {
        error!("Configuration file not found");
        process::exit(1);
    }

    // Run main program with loaded configuration
    match main(matches.value_of("config").unwrap(), config) {
        Ok(_) => info!("System monitor running"),
        Err(e) => {
            error!("Error starting system monitor: {}", e);
            process::exit(1);
        }
    }
}

// This was a bit of a hack to get the test runner to work properly
// Not proud of this but it works.
fn run_test() -> Result<(), Box<dyn std::error::Error>> {
    // Run tests in parallel using `futures`
    let test_results = futures::executor::block_on(async {
        sysmon::tests::runner().await
    });

    // Check if any tests failed
    if test_results.is_err() {
        error!("Test runner encountered an error");
        Err(test_results.unwrap_err())
    } else {
        info!("Tests ran successfully");
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_main() {
        // Run main test
        run_test().unwrap();
    }
}