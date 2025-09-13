// File: src/conversion.rs

//! Provides file-based conversion utilities between GFA and ODGI formats.
//!
//! The functions in this module shell out to the `odgi` command-line executable
//! that is compiled as part of this crate's build process. This provides a stable
//! and robust way to perform complex file conversions.

use super::graph::Error;
use std::process::Command;

/// Converts a GFA file to an ODGI file by calling `odgi build`.
///
/// # Arguments
///
/// * `gfa_path` - The path to the input GFA file.
/// * `odgi_path` - The path where the output ODGI file will be saved.
///
/// # Errors
///
/// Returns an `Error` if the `odgi build` command fails, for example if the
/// input file does not exist or the GFA is malformed.
pub fn gfa_to_odgi(gfa_path: &str, odgi_path: &str) -> Result<(), Error> {
    let odgi_exe = env!("ODGI_EXE");
    let output = Command::new(odgi_exe)
        .arg("build")
        .arg("-g")
        .arg(gfa_path)
        .arg("-o")
        .arg(odgi_path)
        .output()
        .map_err(|e| Error(format!("Failed to execute odgi command: {}", e)))?;

    if output.status.success() {
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error(format!(
            "odgi build command failed for '{}': {}",
            gfa_path, stderr
        )))
    }
}

/// Converts an ODGI file to a GFA file by calling `odgi view`.
///
/// # Arguments
///
/// * `odgi_path` - The path to the input ODGI file.
/// * `gfa_path` - The path where the output GFA file will be saved.
///
/// # Errors
///
/// Returns an `Error` if the `odgi view` command fails or if the resulting
/// GFA content cannot be written to the output file.
pub fn odgi_to_gfa(odgi_path: &str, gfa_path: &str) -> Result<(), Error> {
    let odgi_exe = env!("ODGI_EXE");
    let output = Command::new(odgi_exe)
        .arg("view")
        .arg("-i")
        .arg(odgi_path)
        .arg("-g") // Output in GFA format
        .output()
        .map_err(|e| Error(format!("Failed to execute odgi command: {}", e)))?;

    if output.status.success() {
        std::fs::write(gfa_path, output.stdout)
            .map_err(|e| Error(format!("Failed to write GFA output to file: {}", e)))?;
        Ok(())
    } else {
        let stderr = String::from_utf8_lossy(&output.stderr);
        Err(Error(format!(
            "odgi view command failed for '{}': {}",
            odgi_path, stderr
        )))
    }
}