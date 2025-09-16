// File: src/conversion.rs

//! Provides utilities to convert between GFA and ODGI file formats.
//!
//! The functions in this module shell out to the `odgi` command-line executable
//! that is compiled as part of this crate's build process. This provides a stable
//! and robust way to perform complex file conversions without linking the entire
//! `odgi build` and `odgi view` logic into the library binary.
use super::graph::Error;
use std::io::Write; // Needed for the updated examples
use std::process::Command;
use tempfile::NamedTempFile; // Needed for the updated examples

/// Converts a GFA file to an ODGI file by calling `odgi build`.
///
/// This function is useful for preparing an ODGI graph from the more common
/// GFA format, making it ready to be loaded by [`super::Graph::load`].
///
/// # Arguments
///
/// * `gfa_path` - Path to the input GFA file.
/// * `odgi_path` - Path for the output ODGI file.
///
/// # Errors
///
/// Returns an [`Error`] if the `odgi build` command fails. This can happen if the
/// input file does not exist, the GFA is malformed, or the output path is
/// not writable.
///
/// # Examples
///
/// ```rust,no_run
/// use odgi_ffi::gfa_to_odgi;
/// use std::io::Write;
/// use tempfile::NamedTempFile;
///
/// // 1. Create a temporary GFA file.
/// let mut gfa_file = NamedTempFile::new().unwrap();
/// writeln!(gfa_file, "S\t1\tGATTACA").unwrap();
///
/// // 2. Prepare the path for the ODGI output file.
/// let odgi_file = NamedTempFile::new().unwrap();
/// let odgi_path = odgi_file.path();
///
/// // 3. Convert the GFA to ODGI format.
/// gfa_to_odgi(gfa_file.path().to_str().unwrap(), odgi_path.to_str().unwrap())
///     .expect("Conversion failed");
///
/// assert!(odgi_path.exists());
/// ```
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
/// This is the reverse operation of [`gfa_to_odgi`].
///
/// # Arguments
///
/// * `odgi_path` - Path to the input ODGI file.
/// * `gfa_path` - Path for the output GFA file.
///
/// # Errors
///
/// Returns an [`Error`] if the `odgi view` command fails or if the resulting
/// GFA content cannot be written to the output file.
///
/// # Examples
///
/// ```rust,no_run
/// use odgi_ffi::{gfa_to_odgi, odgi_to_gfa};
/// use std::io::Write;
/// use tempfile::NamedTempFile;
///
/// // 1. First, create a dummy ODGI file to use as input.
/// let mut gfa_file = NamedTempFile::new().unwrap();
/// writeln!(gfa_file, "S\t1\tGATTACA").unwrap();
/// let odgi_file = NamedTempFile::new().unwrap();
/// gfa_to_odgi(gfa_file.path().to_str().unwrap(), odgi_file.path().to_str().unwrap()).unwrap();
///
/// // 2. Prepare the path for the GFA output file.
/// let gfa_out_file = NamedTempFile::new().unwrap();
/// let gfa_out_path = gfa_out_file.path();
///
/// // 3. Convert it back to GFA format.
/// odgi_to_gfa(odgi_file.path().to_str().unwrap(), gfa_out_path.to_str().unwrap())
///     .expect("Conversion failed");
///
/// assert!(gfa_out_path.exists());
/// ```
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