use std::untrusted::fs::File;
use std::io::{Read, Result};
use std::path::Path;
use std::string::String;
/// Read a file at `path` into a String
pub fn read_file(path: &Path) -> Result<String> {
    info!("Loading file {:?}", path);
    let mut f = File::open(&path)?;
    let mut buffer = String::new();
    f.read_to_string(&mut buffer)?;

    Ok(buffer)
}
