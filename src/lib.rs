use wasm_bindgen::prelude::*;
use web_sys::console::log_1 as log;
use base64::{encode, decode};
use image::{load_from_memory, DynamicImage, ColorType};
use image::codecs::png::PngEncoder;
use image::ImageEncoder; // Import the ImageEncoder trait
use std::io::Cursor;

#[wasm_bindgen]
pub fn grayscale(encoded_file: &str) -> Result<String, JsValue> {
    log(&"🟢 Step 1: Starting grayscale conversion".into());

    // Step 1: Decode base64 string
    log(&"🟡 Step 2: Decoding Base64 input".into());
    let base64_to_vector = decode(encoded_file)
        .map_err(|e| JsValue::from_str(&format!("Base64 decode error: {}", e)))?;
    log(&"✅ Step 2: Base64 decoded successfully".into());

    // Step 2: Load image from decoded bytes
    log(&"🟡 Step 3: Loading image from memory".into());
    let image = load_from_memory(&base64_to_vector)
        .map_err(|e| JsValue::from_str(&format!("Image loading error: {}", e)))?;
    log(&"✅ Step 3: Image loaded successfully".into());

    // Step 3: Convert image to grayscale
    log(&"🟡 Step 4: Converting image to grayscale".into());
    let gray_image = image.grayscale();
    log(&"✅ Step 4: Grayscale conversion completed".into());

    // Step 4: Encode grayscale image to PNG format
    log(&"🟡 Step 5: Encoding grayscale image to PNG".into());
    let encoded_bytes = write_png(gray_image)?;
    log(&"✅ Step 5: Image encoded to PNG".into());

    // Step 5: Encode PNG bytes back to Base64
    log(&"🟡 Step 6: Encoding PNG bytes to Base64".into());
    let encoded_img = encode(&encoded_bytes);
    log(&"✅ Step 6: PNG bytes encoded to Base64 successfully".into());

    // Step 6: Format final Data URL
    log(&"🟡 Step 7: Formatting Data URL".into());
    let data_url = format!("data:image/png;base64,{}", encoded_img);
    log(&"✅ Step 7: Data URL formatted successfully".into());

    log(&"🏁 Finished grayscale conversion".into());

    Ok(data_url)
}

// Helper function to write PNG
fn write_png(image: DynamicImage) -> Result<Vec<u8>, JsValue> {
    let mut buf = Cursor::new(Vec::new());

    let rgba8 = image.to_rgba8();
    let (width, height) = rgba8.dimensions();

    // Use the `write_image` method from the `ImageEncoder` trait
    PngEncoder::new(&mut buf)
        .write_image(&rgba8, width, height, ColorType::Rgba8)
        .map_err(|e| JsValue::from_str(&format!("Encoding PNG failed: {}", e)))?;

    Ok(buf.into_inner())
}