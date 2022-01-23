use std::path::PathBuf;

use clap::{Parser, Subcommand};

mod split;

/// A tool to split and recombobulate geometry dash font sprite sheets
#[derive(Parser, Debug)]
#[clap(version, about, long_about = None)]
struct Args {
    #[clap(subcommand)]
    command: Commands,
}

#[derive(Subcommand, Debug)]
enum Commands {
    /// Splits a single font sprite sheet into many individual images
    Split {
        /// The path to the source folder to pull sprite sheets and their data. Defaults to `orig`
        ///
        /// This MUST contain both the `fnt` file AND the `png` file.
        #[clap(short, long, parse(from_os_str), default_value = "./orig/")]
        orig_folder: PathBuf,

        /// The path to the folder to send split sprites to. Defaults to `split`.
        #[clap(short, long, parse(from_os_str), default_value = "./split/")]
        sprites_folder: PathBuf,
    },
    /// Recombines split images into a single sprite sheet
    Combine {
        /// The path to the source folder to individual sprites from. Defaults to `split`
        #[clap(short, long, parse(from_os_str), default_value = "./split/")]
        sprites_folder: PathBuf,

        /// The path to the folder to send the newly created sprite sheet to. Defaults to `dest`.
        #[clap(short, long, parse(from_os_str), default_value = "./dest/")]
        dest_folder: PathBuf,
    },
}

fn main() {
    let args = Args::parse();

    match &args.command {
        Commands::Split {
            orig_folder,
            sprites_folder,
        } => {
            let split = split::Split::new(orig_folder.to_path_buf(), sprites_folder.to_path_buf())
                .expect("Please ensure both your input and output paths exist");
            println!(
                "Orig: {}, Sprites: {}",
                orig_folder.display(),
                sprites_folder.display()
            );
        }
        Commands::Combine {
            dest_folder,
            sprites_folder,
        } => {
            println!(
                "Sprites: {}, Dest: {}",
                sprites_folder.display(),
                dest_folder.display()
            )
        }
    }
}
