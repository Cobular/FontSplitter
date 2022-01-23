use crate::data_format::{Char, Common, FntData, Info, Page};
use nom::{
    branch::alt,
    bytes::complete::{tag, take_until},
    character::complete::{i32, multispace0, space0, u32},
    combinator::eof,
    error::context,
    multi::{many_m_n, separated_list1},
    sequence::{delimited, pair, preceded},
    IResult,
};

pub fn parse_fnt(input: &str) -> FntData {
    let (rest, info_line) = get_line("info ", input).unwrap();
    let info = parse_info_line(info_line);
    let (rest, common_line) = get_line("common ", rest).unwrap();
    let common = parse_common_line(common_line);
    let (rest, page_line) = get_line("page ", rest).unwrap();
    let (rest, chars_line) = get_line("chars ", rest).unwrap();
    let (_, count) = get_u32_property("count", chars_line).unwrap();
    let (_, char_lines) = get_many_lines("char ", rest, count.try_into().unwrap()).unwrap();
    let page = parse_page(page_line, count, char_lines);

    return FntData { info, common, page };
}

fn parse_page(page_line: &str, char_count: u32, char_lines: Vec<&str>) -> Page {
    let (page_rest, page_id) = get_u32_property("id", page_line).unwrap();
    let (_, page_file) = get_str_property("file", page_rest).unwrap();
    let mut chars: Vec<Char> = Vec::with_capacity(char_count.try_into().unwrap());
    for char_line in char_lines {
        let (char_rest, char_id) = get_u32_property("id", char_line).unwrap();
        let (char_rest, char_x) = get_u32_property("x", char_rest).unwrap();
        let (char_rest, char_y) = get_u32_property("y", char_rest).unwrap();
        let (char_rest, char_width) = get_u32_property("width", char_rest).unwrap();
        let (char_rest, char_height) = get_u32_property("height", char_rest).unwrap();
        let (char_rest, char_xoffset) = get_i32_property("xoffset", char_rest).unwrap();
        let (char_rest, char_yoffset) = get_i32_property("yoffset", char_rest).unwrap();
        let (char_rest, char_xadvance) = get_u32_property("xadvance", char_rest).unwrap();
        let (char_rest, _) = get_u32_property("page", char_rest).unwrap();
        let (char_rest, char_chnl) = get_u32_property("chnl", char_rest).unwrap();
        let (_, char_letter) = get_str_property("letter", char_rest).unwrap();

        chars.push(Char {
            id: char_id,
            x: char_x,
            y: char_y,
            width: char_width,
            height: char_height,
            xoffset: char_xoffset,
            yoffset: char_yoffset,
            xadvance: char_xadvance,
            chnl: char_chnl,
            letter: char_letter.to_string(),
        })
    }

    return Page {
        id: page_id,
        file: page_file.to_string(),
        char_count,
        chars,
    };
}

fn parse_common_line(common_line: &str) -> Common {
    let (common_line, line_height) = get_u32_property("lineHeight", common_line).unwrap();
    let (common_line, base) = get_u32_property("base", common_line).unwrap();
    let (common_line, scale_w) = get_u32_property("scaleW", common_line).unwrap();
    let (common_line, scale_h) = get_u32_property("scaleH", common_line).unwrap();
    let (common_line, pages) = get_u32_property("pages", common_line).unwrap();
    let (_, packed) = get_u32_property("packed", common_line).unwrap();

    return Common {
        lineHeight: line_height,
        base,
        scaleW: scale_w,
        scaleH: scale_h,
        pages,
        packed,
    };
}

fn parse_info_line(info_line: &str) -> Info {
    let (info_line, face) = get_str_property("face", info_line).unwrap();
    let (info_line, size) = get_u32_property("size", info_line).unwrap();
    let (info_line, bold) = get_u32_property("bold", info_line).unwrap();
    let (info_line, italic) = get_u32_property("italic", info_line).unwrap();
    let (info_line, charset) = get_str_property("charset", info_line).unwrap();
    let (info_line, unicode) = get_u32_property("unicode", info_line).unwrap();
    let (info_line, stretch_h) = get_u32_property("stretchH", info_line).unwrap();
    let (info_line, smooth) = get_u32_property("smooth", info_line).unwrap();
    let (info_line, aa) = get_u32_property("aa", info_line).unwrap();
    let (info_line, padding) = get_u32_list_property("padding", info_line).unwrap();
    let (_, spacing) = get_u32_list_property("spacing", info_line).unwrap();

    return Info {
        face: face.to_string(),
        size,
        bold,
        italic,
        charset: charset.to_string(),
        unicode,
        stretchH: stretch_h,
        smooth,
        aa,
        padding,
        spacing,
    };
}

/// Gets a single line based on the prefix.
fn get_line<'a>(prefix: &'a str, input: &'a str) -> IResult<&'a str, &'a str> {
    preceded(pair(multispace0, tag(prefix)), take_until("\n"))(input)
}

fn get_many_lines<'a>(
    prefix: &'a str,
    input: &'a str,
    lines_count: usize,
) -> IResult<&'a str, Vec<&'a str>> {
    many_m_n(
        lines_count,
        lines_count,
        preceded(pair(multispace0, tag(prefix)), alt((take_until("\n"), eof))),
    )(input)
}

/// Gets any property surrounded with quotes, like `face="Pusab"` => `Pusab`
fn get_str_property<'a>(property: &'a str, input: &'a str) -> IResult<&'a str, &'a str> {
    context(
        "get_str_property",
        delimited(
            pair(space0, pair(tag(property), tag("=\""))),
            take_until("\""),
            pair(tag("\""), multispace0),
        ),
    )(input)
}

/// Gets any property surrounded with quotes, like `face="Pusab"` => `Pusab`
fn get_i32_property<'a>(property: &'a str, input: &'a str) -> IResult<&'a str, i32> {
    context(
        "get_u64_property",
        delimited(
            pair(space0, pair(tag(property), tag("="))),
            i32,
            multispace0,
        ),
    )(input)
}

/// Gets any property surrounded with quotes, like `face="Pusab"` => `Pusab`
fn get_u32_property<'a>(property: &'a str, input: &'a str) -> IResult<&'a str, u32> {
    context(
        "get_u64_property",
        delimited(
            pair(space0, pair(tag(property), tag("="))),
            u32,
            multispace0,
        ),
    )(input)
}

fn get_u32_list_property<'a>(property: &'a str, input: &'a str) -> IResult<&'a str, Vec<u32>> {
    context(
        "get_u64_property",
        delimited(
            pair(space0, pair(tag(property), tag("="))),
            separated_list1(tag(","), u32),
            multispace0,
        ),
    )(input)
}
