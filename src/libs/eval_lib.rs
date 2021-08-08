// Copyright (C) 2017 Stephane Raux. Distributed under the MIT license.
// https://github.com/stephaneyfx/everust

extern crate tempdir;

use std::error::Error;
use std::fmt::{self, Display};
use std::fs::File;
use std::io::{self, Write};
use std::path::Path;
use std::process::Command;
use tempdir::TempDir;

/// Type of errors that can occur when calling `eval`.
#[derive(Debug)]
pub enum EvalError {
    /// The string contains the build messages.
    Build(String),
    /// Other type of error.
    Other(OtherFailure),
    /// The string contains what was written by the program to stderr.
    ProgReturnedError(String),
}

impl Error for EvalError {
    fn description(&self) -> &str {
        match *self {
            EvalError::Build(_) => "Build failed",
            EvalError::Other(_) => "Other error",
            EvalError::ProgReturnedError(_) => "Program returned an error",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            EvalError::Other(ref e) => Some(&e.0),
            _ => None,
        }
    }
}

impl Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        #[allow(deprecated)]
        write!(f, "{}", self.description())?;
        let s = match *self {
            EvalError::Build(ref s) => s,
            EvalError::ProgReturnedError(ref s) => s,
            _ => return Ok(()),
        };
        write!(f, "\n{}", s)
    }
}

/// Other type of errors that can occur when evaluating rust code.
#[derive(Debug)]
pub struct OtherFailure(OtherError);

#[derive(Debug)]
enum OtherError {
    CreateTempDir(io::Error),
    SpawnProg(io::Error),
    SpawnRustc(io::Error),
    WriteSrcFile(io::Error),
}

impl Error for OtherError {
    fn description(&self) -> &str {
        match *self {
            OtherError::CreateTempDir(_) => {
                "Failed to create temporary \
                directory"
            }
            OtherError::SpawnProg(_) => "Failed to spawn program",
            OtherError::SpawnRustc(_) => "Failed to spawn rustc",
            OtherError::WriteSrcFile(_) => "Failed to write source file",
        }
    }

    fn cause(&self) -> Option<&dyn Error> {
        match *self {
            OtherError::CreateTempDir(ref e) => Some(e),
            OtherError::SpawnProg(ref e) => Some(e),
            OtherError::SpawnRustc(ref e) => Some(e),
            OtherError::WriteSrcFile(ref e) => Some(e),
        }
    }
}

impl Display for OtherError {
    #[allow(deprecated)]
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.description())
    }
}

impl From<OtherError> for EvalError {
    fn from(e: OtherError) -> EvalError {
        EvalError::Other(OtherFailure(e))
    }
}
pub fn eval(code: &str, with_main: bool) -> Result<String, EvalError> {
    let temp = TempDir::new("").map_err(OtherError::CreateTempDir)?;
    let code_path = temp.path().join("main.rs");
    if with_main {
        write_source_file(&code_path, code).map_err(OtherError::WriteSrcFile)?;
    } else {
        write_source_file_without_main(&code_path, code).map_err(OtherError::WriteSrcFile)?;
    }
    let out_path = temp.path().join("main");
    let out = Command::new("rustc")
        .arg("-o")
        .arg(&out_path)
        .arg(&code_path)
        .output()
        .map_err(OtherError::SpawnRustc)?;
    if !out.status.success() {
        return Err(EvalError::Build(
            String::from_utf8_lossy(&out.stderr).into_owned(),
        ));
    }
    let out = Command::new(&out_path)
        .output()
        .map_err(OtherError::SpawnProg)?;
    if out.status.success() {
        Ok(String::from_utf8_lossy(&out.stdout).into_owned())
    } else {
        Err(EvalError::ProgReturnedError(
            String::from_utf8_lossy(&out.stderr).into_owned(),
        ))
    }
}

fn write_source_file(path: &Path, code: &str) -> io::Result<()> {
    let mut f = File::create(path)?;
    write!(
        &mut f,
        r##"
fn main() {{

    {};

}}
    "##,
        code
    )?;
    f.sync_all()
}

fn write_source_file_without_main(path: &Path, code: &str) -> io::Result<()> {
    let mut f = File::create(path)?;
    write!(
        &mut f,
        r##"
{}
    "##,
        code
    )?;
    f.sync_all()
}
