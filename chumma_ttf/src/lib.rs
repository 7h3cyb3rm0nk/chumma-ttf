use harfbuzz_wasm::{debug, Font, Glyph, GlyphBuffer};
use wasm_bindgen::prelude::*;
mod dictionary;
use dictionary::generate_words;
use lazy_static::lazy_static;
use std::collections::HashMap;
lazy_static! {
    static ref WORDS: HashMap<String, String> = generate_words();
}

#[wasm_bindgen]
pub fn shape(
    _shape_plan: u32,
    font_ref: u32,
    buf_ref: u32,
    _features: u32,
    _num_features: u32,
) -> i32 {
    let font = Font::from_ref(font_ref);
    let mut buffer = GlyphBuffer::from_ref(buf_ref);

    let buf_u8: Vec<u8> = buffer
        .glyphs
        .iter()
        .map(|g| g.codepoint as u8)
        .collect::<Vec<u8>>();
    let str_buf = String::from_utf8_lossy(&buf_u8);
    let mut result = str_buf.to_string();
    let list: Vec<&str> = str_buf.split(" ").collect();
    for word in list.iter().map(|x| x.to_string()) {
        if WORDS.contains_key(&word) {
            result = result.replace(&word, WORDS.get(&word).expect(&word));
        }
    }

    let result = result.replace("Hello", "Hi");
    debug(&result);
    buffer.glyphs = result
        .chars()
        .enumerate()
        .map(|(ix, x)| Glyph {
            codepoint: x as u32,
            flags: 0,
            x_advance: 0,
            y_advance: 0,
            cluster: ix as u32,
            x_offset: 0,
            y_offset: 0,
        })
        .collect();
    for item in buffer.glyphs.iter_mut() {
        item.codepoint = font.get_glyph(item.codepoint, 0);
        item.x_advance = font.get_glyph_h_advance(item.codepoint);
    }
    1
}
