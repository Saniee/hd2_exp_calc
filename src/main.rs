#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use gui::AppGui;

use crate::helldivers_data::functions::load_table;

mod gui;
mod helldivers_data;

#[tokio::main]
async fn main() -> eframe::Result {
    let table_elements = load_table().await.unwrap();
    for element in table_elements {
        println!("{:?}", element);
    }
    
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default().with_inner_size([340.0, 240.0]),
        ..Default::default()
    };
    eframe::run_native("Helldivers 2 Experience Calculator", options, Box::new(|_| {
        Ok(Box::<AppGui>::default())
    }))
}
