#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")] // hide console window on Windows in release

use eframe::egui;
use gui::AppGui;

mod gui;
mod helldivers_data;

#[tokio::main]
async fn main() -> eframe::Result {
    let options = eframe::NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_resizable(false)
            .with_maximize_button(false)
            .with_inner_size([360.0, 250.0]),
        ..Default::default()
    };
    let app = AppGui::new().await;
    eframe::run_native("Helldivers 2 Experience Calculator", options, Box::new(|_| {
        Ok(Box::new(app))
    }))
}
