// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::api::dialog::blocking::FileDialogBuilder;
use std::io::Cursor;
use std::path::PathBuf;
use serde::Serialize;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use ts_rs::TS;
use image::{ImageFormat, ImageReader};

#[derive(TS)]
#[ts(export)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LoadedImageData {
  file_path: PathBuf,
  base64_data: String,
  width: u32,
  height: u32,
}

#[tauri::command]
async fn open_image()  -> Result<LoadedImageData, String> {
  let dialog_result = FileDialogBuilder::new().pick_file();
  match dialog_result {
    Some(path) => {
      let img_reader = ImageReader::open(&path).expect("Failed to open image").with_guessed_format().expect("Failed to guess image format");
      let img_fmt = img_reader.format().expect("Failed to get image format");
      let img = img_reader.decode().expect("Failed to decode image");
      let mut base64_img = Cursor::new(Vec::new());
      img.write_to(&mut base64_img, match img_fmt {
        ImageFormat::Png => image::ImageFormat::Png,
        ImageFormat::Jpeg => image::ImageFormat::Jpeg,
        ImageFormat::Gif => image::ImageFormat::Gif,
        _ => return Err("Unsupported image format".to_string()),
      }).expect("Failed to write image to buffer");
      let image_data = LoadedImageData {
        file_path: path,
        base64_data: STANDARD.encode(base64_img.get_ref()),
        width: img.width(),
        height: img.height(),
      };
      Ok(image_data)
    },
    None => Ok(LoadedImageData{
      file_path: "".into(),
      base64_data: "".to_string(),
      width: 0,
      height: 0,
    }),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_image])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
