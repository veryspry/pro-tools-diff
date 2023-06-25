// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::time::Duration;
use serde::{Serialize};

fn main() {
  tauri::Builder::default()
    .invoke_handler(tauri::generate_handler![greet, get_connected_drives])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}

#[tauri::command]
fn greet(name: &str) -> String {
   format!("Hello, {}!", name)
}

#[derive(Serialize)]
pub struct Drive {
  bus_number: u8,
  address: u8,
  vendor_id: u16,
  product_id: u16,
  product_name: String
}

#[tauri::command]
fn get_connected_drives() -> Vec<Drive> {
  let mut drives = Vec::new();

  for device in rusb::devices().unwrap().iter() {
    let device_desc = device.device_descriptor().unwrap();
    let handle = device.open().unwrap();

    let languages = handle.read_languages(Duration::from_secs(3)).unwrap();
    // finds the english language id
    // let language = languages.into_iter().find(|l| l == &0x0409).unwrap();
    let language = languages.first().unwrap();

    let product_str_idx = device_desc.product_string_index();
    let name_descriptor = handle.read_string_descriptor(
      *language,
      product_str_idx.unwrap(),
      Duration::from_secs(3),
    ).unwrap();

    drives.push(Drive {
      bus_number: device.bus_number(), 
      address: device.address(),
      vendor_id: device_desc.vendor_id(),
      product_id: device_desc.product_id(),
      product_name: name_descriptor,
    });

    // println!("Bus {:03} Device {:03} ID {:04x}:{:04x}",
    //   device.bus_number(),
    //   device.address(),
    //   device_desc.vendor_id(),
    //   device_desc.product_id()
    //);

  }

  drives
}
