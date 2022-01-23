use crate::parser::parse_fnt;

#[derive(Debug)]
pub struct FntData {
    pub info: Info,
    pub common: Common,
    pub page: Page
}

impl From<&str> for FntData {
    fn from(i: &str) -> Self {
        parse_fnt(i)
    }
}

#[derive(Debug)]
pub struct Info {
    pub face: String,
    pub size: u32,
    pub bold: u32,
    pub italic: u32,
    pub charset: String,
    pub unicode: u32,
    pub stretchH: u32,
    pub smooth: u32,
    pub aa: u32,
    pub padding: Vec<u32>,
    pub spacing: Vec<u32>
}

#[derive(Debug)]
pub struct Common {
    pub lineHeight: u32,
    pub base: u32,
    pub scaleW: u32,
    pub scaleH: u32,
    pub pages: u32,
    pub packed: u32,
}

#[derive(Debug)]
pub struct Page {
    pub id: u32,
    pub file: String,
    pub char_count: u32,
    pub chars: Vec<Char>
}

#[derive(Debug)]
pub struct Char {
    pub id: u32,
    pub x: u32,
    pub y: u32,
    pub width: u32,
    pub height: u32,
    pub xoffset: i32,
    pub yoffset: i32,
    pub xadvance: u32,
    pub chnl: u32,
    pub letter: String,
}

