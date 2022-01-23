use image::{io::Reader, GenericImageView};
use std::{
    fs::{self, read_to_string},
    io,
    path::PathBuf,
};

use font_splitter::{data_format::FntData, validate_path};

pub struct Split {
    orig_folder: PathBuf,
    sprites_folder: PathBuf,
    file_name: String,
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
        let file_name: String = match (dir.nth(0), dir.nth(0)) {
            (None, None) | (None, Some(_)) | (Some(_), None) => {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    "Could not find one of the desired files",
                ))
            }
            (Some(first_file), Some(second_file)) => {
                let first_file = first_file?;
                if first_file.path().file_stem() != second_file?.path().file_stem() {
                    return Err(io::Error::new(
                        io::ErrorKind::Other,
                        "Filenames do not match",
                    ));
                }
                let path = first_file.path();
                let string = path
                    .file_stem()
                    .expect("msg")
                    .to_str()
                    .ok_or(io::Error::new(io::ErrorKind::Other, ""))?;
                string.to_owned()
            }
        };

        return Ok(Split {
            orig_folder,
            sprites_folder,
            file_name,
        });
    }

    pub fn parse_fnt(&self) -> Result<(), io::Error> {
        let mut font_data_file_path = self.orig_folder.clone();
        font_data_file_path.push(&self.file_name);
        font_data_file_path.set_extension("fnt");

        let fnt_file = read_to_string(font_data_file_path)?;
        let data = FntData::from(fnt_file.as_str());

        let mut font_sprites_file_path = self.orig_folder.clone();
        font_sprites_file_path.push(&data.page.file);
        let image = match Reader::open(font_sprites_file_path)?.decode() {
            Ok(it) => it,
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };

        for char in data.page.chars {
            // Zero width not supported in images
            if char.width == 0 || char.height == 0 {
                continue;
            }

            let filename = match char.letter.as_str() {
                "<" => "lt",
                ">" => "gt",
                ":" => "colon",
                "\"" => "dq",
                "/" => "fs",
                "\\" => "bs",
                "|" => "pipe",
                "?" => "qm",
                "*" => "star",
                _ => char.letter.as_str()
            };

            let char_image = image.view(char.x, char.y, char.width, char.height);
            let mut char_save_path = self.sprites_folder.clone();
            char_save_path.push(filename);
            char_save_path.set_extension("png");
            match char_image.to_image().save(char_save_path) {
                Ok(it) => it,
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
            };
        }
        return Ok(());
    }
}
