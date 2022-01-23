use std::{fs, io, path::PathBuf};

#[path = "utils/validate_path.rs"]
mod validate_path;
pub struct Split {
    orig_folder: PathBuf,
    sprites_folder: PathBuf,
}

impl Split {
    pub fn new(orig_folder: PathBuf, sprites_folder: PathBuf) -> Result<Split, io::Error> {
        // Validates folders exist
        validate_path::validate_path(&orig_folder, true)?;
        validate_path::validate_path(&sprites_folder, true)?;

        // Validates required contents of original folder exist
        // Must be 2 files with same name, one must have extension .fnt
        //  and the other must be a PNG
        let mut dir = fs::read_dir(&orig_folder)?;

        match (dir.nth(0), dir.nth(0)) {
            (None, None) | (None, Some(_)) | (Some(_), None) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Could not find one of the desired files",
                ))
            }
            (Some(first_file), Some(second_file)) => {
                if first_file?.path().file_stem() != second_file?.path().file_stem() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Filenames do not match",
                    ));
                }
            }
        }

        return Ok(Split {
            orig_folder,
            sprites_folder,
        });
    }

    // fn parse_fnt() -> Result<(), ()> {

    // }
}
