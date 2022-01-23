# Font Splitter

_A tool to split and recombobulate geometry dash font sprite sheets_
Designed to allow <ins>you</ins> to make your own custom fonts for GD with ease!

# How to Get

From releases (soon!) or clone the repo and run `cargo build --release`

# How to Use

This has 2 subcommands:

1.  `split` - Splits a single font sprite sheet into individual images for each character
2.  `combine` - Merge pre-split images back into a single image. Must be the images split up by this tool or the sizes probably won't work out.

Uses 3 folders:

1.  `orig` - The source folder for where to find the `fnt` file and the associated `png` for the spritesheet. These must have the same name and must be the only two files in the folder.
2.  `split` - Where all the split files end up.
3.  `dest` - Where the recombobulated single spritesheet will end up.

-------------

Made with 🧡 for Torzod
