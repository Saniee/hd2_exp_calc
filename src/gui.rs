// This is here so the compiler doesnt complain about unused values. Will be removed when they are.
#![allow(unused)]

use crate::helldivers_data::functions::{self, DataHandling};

#[derive(Clone)]
pub struct AppGui {
    current_exp: i64,
    wanted_exp: i64,
    recieved_exp: i64,
    mission_time: i64,
    xp_arr: Vec<i64>,
    time_arr: Vec<i64>,
    result: String,
    current_rank: i64,
    wanted_rank: i64,
    data_handler: DataHandling
}

impl AppGui {
    pub async fn new() -> Self {
        let mut data_handler = DataHandling::new();
        data_handler.load_table().await.unwrap();
        AppGui {
            current_exp: i64::default(),
            wanted_exp: i64::default(),
            recieved_exp: i64::default(),
            mission_time: i64::default(),
            xp_arr: Vec::new(),
            time_arr: Vec::new(),
            result: String::new(),
            current_rank: i64::default(),
            wanted_rank: i64::default(),
            data_handler
        }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.label(format!("Current Rank: {}\nWanted Rank: {}", self.current_rank, self.wanted_rank));
                let want_xp = ui.add(egui::Slider::new(&mut self.wanted_exp, 0..=1168000).text("Wanted Amount of XP"));
                if want_xp.changed() {
                    self.wanted_rank = functions::find_rank(self.data_handler.clone(), self.wanted_exp);
                }
                let cur_xp = ui.add(egui::Slider::new(&mut self.current_exp, 0..=1168000).text("Current XP"));
                if cur_xp.changed() {
                    self.current_rank = functions::find_rank(self.data_handler.clone(), self.current_exp);
                }
            });

            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.mission_time, 0..=40).text("Mission Time (Minutes)"));
                ui.add(egui::Slider::new(&mut self.recieved_exp, 0..=3000).text("Recieved Experience"))
            });

            if ui.add(egui::Button::new("Calculate!")).clicked() {
                (self.xp_arr, self.time_arr, self.result) = functions::calculate(self.data_handler.clone(), self.current_rank, self.wanted_exp, self.current_exp, self.recieved_exp, self.mission_time, self.xp_arr.clone(), self.time_arr.clone());
            };
            
            ui.label(self.result.to_string());
        });
    }
}