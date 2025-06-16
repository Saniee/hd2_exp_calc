// This is here so the compiler doesnt complain about unused values. Will be removed when they are.
#![allow(unused)]

use crate::helldivers_data::functions::{self, DataHandling};

#[derive(Clone, Default)]
pub struct CalcResult {
    pub avg_time: i64,
    pub avg_xp: i64,
}

#[derive(Clone)]
pub struct AppGui {
    current_exp: i64,
    wanted_exp: i64,
    recieved_exp: i64,
    mission_time: i64,
    xp_arr: Vec<i64>,
    time_arr: Vec<i64>,
    result: CalcResult,
    current_rank: i64,
    wanted_rank: i64,
    data_handler: DataHandling
}

impl AppGui {
    pub async fn new() -> Self {
        let mut data_handler = DataHandling::new();
        data_handler.load_table().await.unwrap();
        let result = CalcResult::default();
        AppGui {
            current_exp: i64::default(),
            wanted_exp: i64::default(),
            recieved_exp: i64::default(),
            mission_time: i64::default(),
            xp_arr: Vec::new(),
            time_arr: Vec::new(),
            result,
            current_rank: 1,
            wanted_rank: 1,
            data_handler
        }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            // Main sliders for knowing where the player is. Later ill add the wiki link.
            // So people know how much they need to input.
            ui.vertical(|ui| {
                ui.label(format!("Current Rank: {}\nWanted Rank: {}", self.current_rank, self.wanted_rank));
                let want_xp = ui.add(egui::Slider::new(&mut self.wanted_exp, 0..=1168000).text("Wanted Amount of XP"));
                // Determine where the rank would be with the amount of xp inputed.
                if want_xp.changed() {
                    self.wanted_rank = functions::find_rank(self.data_handler.clone(), self.wanted_exp);
                }
                let cur_xp = ui.add(egui::Slider::new(&mut self.current_exp, 0..=1168000).text("Current XP"));
                // Likewise like the previous if statement.
                if cur_xp.changed() {
                    self.current_rank = functions::find_rank(self.data_handler.clone(), self.current_exp);
                }
            });

            // THE main sliders, these are used to calculate the avg time and xp needed to finish the wanted rank.
            // The values should be reset on clicking the Calculate button.
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.mission_time, 0..=40).text("Mission Time (Minutes)"));
                ui.add(egui::Slider::new(&mut self.recieved_exp, 0..=3000).text("Recieved Experience"))
            });

            // The button which does all the work.
            // Call the main functions under this if statement.
            if ui.add(egui::Button::new("Calculate!")).clicked() {
                (self.xp_arr, self.time_arr, self.result) = functions::calculate_avg(self.mission_time, self.recieved_exp, self.xp_arr.clone(), self.time_arr.clone())
            };
            
            ui.label(format!("Average Time: {} Min.\nAverage XP: {}", self.result.avg_time, self.result.avg_xp));
        });
    }
}