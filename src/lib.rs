use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{encode, decode};
use image::load_from_memory;
use image::ImageOutputFormat::Png;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> String{
    log(&"Grayscale called π".into());

    let base64_to_vector = decode(encoded_file).unwrap();
    log(&"Image decoded π".into());

    let mut img = load_from_memory(&base64_to_vector).unwrap();
    log(&"Image loaded π€ͺ".into());

    img = img.grayscale();
    log(&"Grayscale effect applied π".into());

    let mut buffer = vec![];
    img.write_to(&mut buffer, Png).unwrap();
    log(&"New Image Writtenπ".into());

    let encoded_img = encode(&buffer);
    let data_url = format!(
        "data:image/png;base64,{}",
        encoded_img
    );
    data_url
}
