use std::error::Error;
use std::io::Cursor;

use crate::overlay::image::{Image, OverlayText, PositionType};
use serde::{Deserialize, Serialize};

use super::helpers::load_font_from_bytes;
use super::image::BlendMode;

#[derive(Deserialize, Serialize)]
pub struct BookCoverParams {
    pub author: String,
    pub author_position: PositionType,
    pub title: String,
    pub title_position: PositionType,
    pub blend_mode: BlendMode,
    pub alfa: String,
    pub line_length: String,
}

pub fn put_text(
    img_bytes: Vec<u8>,
    author_font_bytes: Vec<u8>,
    title_font_bytes: Vec<u8>,
    params: BookCoverParams,
) -> Result<Vec<u8>, Box<dyn Error>> {
    let line_length = params.line_length.parse::<u8>()?;
    let alfa = params.alfa.parse::<f32>()?;
    let mut image = Image::from_bytes(img_bytes).expect("Failed to load image");

    let title_splits = textwrap::wrap(params.title.as_str(), line_length as usize);

    let rev_title_splits = title_splits
        .into_iter()
        .map(|s| s.to_string())
        .rev()
        .collect::<Vec<String>>();

    let author_font = load_font_from_bytes(author_font_bytes).expect("Failed to load font");
    let title_font = load_font_from_bytes(title_font_bytes).expect("Failed to load font");

    let author = OverlayText {
        text_list: vec![params.author],
        color: (255, 255, 255),
        offset: (0, 0),
        alpha: alfa,
        font: author_font,
        position: params.author_position,
        blend: params.blend_mode,
    };

    let title = OverlayText {
        text_list: rev_title_splits.clone(),
        color: (255, 255, 255),
        offset: (0, 0),
        alpha: alfa,
        font: title_font.clone(),
        position: params.title_position,
        blend: params.blend_mode,
    };

    image.put_text(author).put_text(title);

    let mut buf: Vec<u8> = Vec::new();
    image
        .dyn_img
        .write_to(&mut Cursor::new(&mut buf), image::ImageOutputFormat::Png)
        .expect("Failed to write image");
    Ok(buf)
}
