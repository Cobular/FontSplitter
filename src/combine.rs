use image::{io::Reader, imageops::overlay};
use std::{
    fs::{self, read_to_string},
    io,
    path::PathBuf,
};

use font_splitter::{data_format::FntData, validate_path};

pub struct Combine {
    sprites_folder: PathBuf,
    dest_folder: PathBuf,
    orig_folder: PathBuf,
    file_name: String,
}

impl Combine {
    pub fn new(
        orig_folder: PathBuf,
        dest_folder: PathBuf,
        sprites_folder: PathBuf,
    ) -> Result<Combine, io::Error> {
        // Validates folders exist
        validate_path::validate_path(&orig_folder, true)?;
        validate_path::validate_path(&sprites_folder, true)?;
        validate_path::validate_path(&dest_folder, true)?;

        // Validates required contents of original folder exist
        // Requires only fnt file.
        let dir = fs::read_dir(&orig_folder)?;
        let mut file_name: Option<String> = None;
        for entry in dir {
            let entry = entry?;
            let path = entry.path();

            match path.extension() {
                Some(extension) => {
                    if extension == "fnt" {
                        file_name = Some(path
                            .file_stem()
                            .expect("Failed to parse fnt to string")
                            .to_str()
                            .ok_or(io::Error::new(io::ErrorKind::Other, ""))?
                            .to_string());
                        break;
                    }
                }
                None => continue,
            }
        }
        if file_name.is_none() {
            return Err(io::Error::new(
                io::ErrorKind::NotFound,
                "Could not find fnt file in orig directory",
            ));
        }

        return Ok(Combine {
            orig_folder,
            dest_folder,
            sprites_folder,
            file_name: file_name.unwrap(),
        });
    }

    pub fn combine(&self) -> Result<(), io::Error> {
        let data = self.parse_fnt()?;
        let mut imgbuf = image::RgbaImage::new(data.common.scaleW, data.common.scaleH);

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

            let mut char_img_path = self.sprites_folder.clone();
            char_img_path.push(filename);
            char_img_path.set_extension("png");

            let char_image = match Reader::open(char_img_path)?.decode() {
                Ok(it) => it,
                Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
            };

            overlay(&mut imgbuf, &char_image, char.x, char.y);
        }

        let mut buf_save_path = self.dest_folder.clone();
        buf_save_path.push(&self.file_name);
        buf_save_path.set_extension("png");

        match imgbuf.save(buf_save_path) {
            Ok(_) => return Ok(()),
            Err(err) => return Err(io::Error::new(io::ErrorKind::Other, err)),
        };
    }

    pub fn parse_fnt(&self) -> Result<FntData, io::Error> {
        let mut font_data_file_path = self.orig_folder.clone();
        font_data_file_path.push(&self.file_name);
        font_data_file_path.set_extension("fnt");

        let fnt_file = read_to_string(font_data_file_path)?;
        return Ok(FntData::from(fnt_file.as_str()));
    }
}
