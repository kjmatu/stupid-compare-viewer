// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::fs;
use std::path::PathBuf;

use tauri::api::dialog::blocking::FileDialogBuilder;

use serde::Serialize;
use ts_rs::TS;

#[derive(TS)]
#[ts(export)]
#[derive(Serialize)]
#[serde(rename_all = "camelCase")]
struct LoadedImageData {
  file_path: PathBuf,
}


fn get_all_image_files_in_dir(path: PathBuf, target_extensions: Vec<&str>) -> Vec<PathBuf> {
  let mut file_paths = Vec::new();
  for entry in fs::read_dir(path).unwrap() {
    let entry = entry.unwrap();
    let path = entry.path();
    if path.is_file() {
      match path.extension() {
        Some(extension) => {
          if target_extensions.contains(&extension.to_ascii_lowercase().to_str().unwrap()) {
            file_paths.push(path);
          }
        },
        None => continue,
      }
    }
  }
  file_paths
}

#[tauri::command]
async fn open_image()  -> Result<LoadedImageData, String> {
  let dialog_result = FileDialogBuilder::new().pick_folder();
  match dialog_result {
    Some(path) => {
      let file_paths = get_all_image_files_in_dir(path, vec!["jpg", "jpeg", "png", "heic"]);
      println!("{:?}", file_paths);
      let image_data = LoadedImageData {
        file_path: file_paths[0].clone(),
      };
      Ok(image_data)
    },
    None => Ok(LoadedImageData{
      file_path: "".into(),
    }),
  }
}

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![open_image])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
