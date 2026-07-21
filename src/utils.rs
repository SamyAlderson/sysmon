// Utility functions for sysmon

use std::collections::HashMap;
use std::convert::TryInto;
use std::fmt;
use std::io::{BufReader, BufWriter, Read, Write};
use std::path::{Path, PathBuf};

use clap::{Parser, ValueEnum};
use futures::future::BoxFuture;
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};
use serde_json::{Error as JsonError, Result as JsonResult};

// Type alias for a boxed future
type BoxedFuture<T> = Box<dyn Future<Output = T> + Send>;

// Helper function to parse a value from a string
fn parse_value<T: FromStr>(s: &str) -> Result<T, T::Err> {
    s.parse().or_else(|_| {
        // If parsing fails, try to convert to a float
        match s.parse::<f64>() {
            Ok(f) => Ok(f as T),
            Err(_) => Err(T::Err),
        }
    })
}

// Helper function to convert a value to a string
fn value_to_string<T: fmt::Display>(value: &T) -> String {
    value.to_string()
}

// Helper function to read a file into a string
async fn read_file<P>(path: P) -> Result<String, std::io::Error>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::open(path)?;
    let reader = BufReader::new(file);
    let mut contents = String::new();
    reader.read_to_string(&mut contents).await?;
    Ok(contents)
}

// Helper function to write a string to a file
async fn write_file<P>(path: P, contents: &str) -> Result<(), std::io::Error>
where
    P: AsRef<Path>,
{
    let file = std::fs::File::create(path)?;
    let writer = BufWriter::new(file);
    writer.write_all(contents.as_bytes()).await?;
    Ok(())
}

// Custom error type for sysmon
#[derive(Debug, Serialize, Deserialize)]
enum SysmonError {
    Io(std::io::Error),
    Json(JsonError),
    // Add more error types as needed
}

impl From<std::io::Error> for SysmonError {
    fn from(e: std::io::Error) -> Self {
        SysmonError::Io(e)
    }
}

impl From<JsonError> for SysmonError {
    fn from(e: JsonError) -> Self {
        SysmonError::Json(e)
    }
}

// Custom error implementation for sysmon
impl fmt::Display for SysmonError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            SysmonError::Io(e) => write!(f, "I/O error: {}", e),
            SysmonError::Json(e) => write!(f, "JSON error: {}", e),
        }
    }
}

impl std::error::Error for SysmonError {}