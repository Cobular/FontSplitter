use std::{fs::metadata, io, path::PathBuf};

pub fn validate_path(possible_path: &PathBuf, folder: bool) -> Result<(), io::Error> {
    let md = metadata(possible_path)?;
    let dir = md.is_dir();
    let file = md.is_file();
    if dir && folder {
        return Ok(());
    }
    if file && !folder {
        return Ok(());
    }
    Err(io::Error::new(
        io::ErrorKind::Other,
        "Dir is not a file or a folder.",
    ))
}
