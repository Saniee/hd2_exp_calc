// This is here so the compiler doesnt complain about unused values. Will be removed when they are.
#![allow(unused)]

use egui::{Color32, Hyperlink, RichText};
use time::Duration;

use crate::helldivers_data::functions::{self, RankHandling};

// Stores the values after calculating avg time and xp per mission.
#[derive(Clone, Default)]
pub struct AvgResult {
    pub avg_time: Duration,
    pub avg_xp: i64,
}

// Stores the inputs for xperience values.
#[derive(Clone, Default)]
pub struct ExperienceInputs {
    pub current_xp: i64,
    pub wanted_xp: i64
}

// Stores the inputs that are used for calculating averages.
#[derive(Clone, Default)]
pub struct MissionInputs {
    pub recieved_exp: i64,
    pub mission_time: i64
}

#[derive(Clone)]
pub struct AppGui {
    xp_inputs: ExperienceInputs,
    mission_inputs: MissionInputs,
    rank_handler: RankHandling,
    wanted_rank: i64,
    current_rank: i64,
    xp_arr: Vec<i64>,
    time_arr: Vec<Duration>,
    avg_results: AvgResult,
    needed_xp: i64,
    final_result: Duration,
}

const LABEL_FONT_SIZE: f32 = 16.0;
const HIGHLIGHT_COLOR: Color32 = Color32::CYAN;

impl AppGui {
    pub async fn new() -> Self {
        let mut rank_handler = RankHandling::new();
        rank_handler.load_table().await.unwrap();
        let avg_results = AvgResult::default();
        let xp_inputs = ExperienceInputs::default();
        let mission_inputs = MissionInputs::default();
        
        AppGui {
            xp_inputs,
            mission_inputs,
            rank_handler,
            wanted_rank: 1,
            current_rank: 1,
            xp_arr: Vec::new(),
            time_arr: Vec::new(),
            avg_results,
            needed_xp: i64::default(),
            final_result: Duration::minutes(0)
        }
    }
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(RichText::new(format!("Current Rank: {}\nWanted Rank: {}", self.current_rank, self.wanted_rank)).size(LABEL_FONT_SIZE).color(HIGHLIGHT_COLOR));
            // Main sliders for knowing where the player is. Later ill add the wiki link.
            // So people know how much they need to input.
            ui.vertical(|ui| {
                // Link to the wiki so people can see how much xp they already have.
                ui.add(Hyperlink::from_label_and_url("Link to all Experiences values.", "https://helldivers.wiki.gg/wiki/Ranks#Ranks,_Experience_and_Unlocks"));
                let cur_xp = ui.add(egui::Slider::new(&mut self.xp_inputs.current_xp, 0..=1168000).text("Current XP"));
                let want_xp = ui.add(egui::Slider::new(&mut self.xp_inputs.wanted_xp, 0..=1168000).text("Wanted Amount of XP"));

                // Determine where the ranks would be with the amount of xp inputed.
                // And calculate needed xp.
                if want_xp.changed() || cur_xp.changed() {
                    self.current_rank = self.rank_handler.find_rank(self.xp_inputs.current_xp);
                    self.wanted_rank = self.rank_handler.find_rank(self.xp_inputs.wanted_xp);

                    // Even if the functions fails, the program continues, albeit faulty.
                    // ? This is good for when people input: current_rank >= wanted_rank.
                    self.needed_xp = self.rank_handler.sum_needed_xp(self.current_rank, self.wanted_rank).unwrap_or_default();
                    // println!("{:?}", self.needed_xp);
                }
            });

            // THE main sliders, these are used to calculate the avg time and xp needed to finish the wanted rank.
            // The values should be reset on clicking the Calculate button.
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.mission_inputs.mission_time, 1..=40).text("Mission Time (Minutes)"));
                ui.add(egui::Slider::new(&mut self.mission_inputs.recieved_exp, 1..=3000).text("Recieved Experience"))
            });

            // The button which does all the work.
            if ui.add(egui::Button::new("Calculate!")).clicked() {
                // Call the main functions under here.
                (self.time_arr, self.xp_arr, self.avg_results) = functions::calculate_avg(Duration::minutes(self.mission_inputs.mission_time), self.mission_inputs.recieved_exp, self.xp_arr.clone(), self.time_arr.clone());
                self.final_result = functions::estimate_time_needed(self.time_arr.clone(), self.needed_xp, self.avg_results.clone(), self.rank_handler.clone());

                // Manipulate the UI so it gives all the info.
                self.xp_inputs = ExperienceInputs { current_xp: self.xp_inputs.current_xp + self.mission_inputs.recieved_exp, wanted_xp: self.xp_inputs.wanted_xp };
                self.current_rank = self.rank_handler.find_rank(self.xp_inputs.current_xp);
                self.mission_inputs = MissionInputs { recieved_exp: 0, mission_time: 0 };
            };
            
            // Main Labels
            ui.label(RichText::new(format!("Average Time: {}\nAverage XP: {}", self.avg_results.avg_time, self.avg_results.avg_xp)).size(LABEL_FONT_SIZE).color(HIGHLIGHT_COLOR));
            ui.label(RichText::new(format!("Time needed to wanted rank: {}", self.final_result)).size(LABEL_FONT_SIZE).color(HIGHLIGHT_COLOR));

            // Clear data button if user f-up.
            if ui.add(egui::Button::new("Clear Data.")).clicked() {
                self.avg_results = AvgResult {avg_time: Duration::minutes(0), avg_xp: 0};
                self.final_result = Duration::minutes(0);
                self.needed_xp = 0;
                self.mission_inputs = MissionInputs {recieved_exp: 0, mission_time: 0};
                self.time_arr = Vec::new();
                self.xp_arr = Vec::new();

                self.xp_inputs.current_xp = 0;
                self.xp_inputs.wanted_xp = 0;
                self.current_rank = self.rank_handler.find_rank(self.xp_inputs.current_xp);
                self.wanted_rank = self.rank_handler.find_rank(self.xp_inputs.wanted_xp);
            }
        });
    }
}