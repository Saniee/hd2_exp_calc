#[derive(Default)]
pub struct AppGui {
    current_exp: i64,
    wanted_exp: i64,
    mission_time: i64,
    xp_arr: Vec<i64>,
    time_arr: Vec<i64>,
    result: i64,
}

impl eframe::App for AppGui {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.vertical(|ui| {
                ui.add(egui::Slider::new(&mut self.wanted_exp, 0..=1168000).text("Wanted Amount of XP"));
                ui.add(egui::Slider::new(&mut self.current_exp, 0..=1168000).text("Current XP"));
                ui.add(egui::Slider::new(&mut self.mission_time, 0..=40).text("Mission Time (Minutes)"));
            });
            if ui.button("Calculate").clicked() {
                // Placeholder
                self.result = self.current_exp / self.mission_time  // calculate()
            };
            ui.label(format!("Hello world! {}", self.result))
        });
    }
}