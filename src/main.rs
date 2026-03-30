use eframe::egui;

struct MyApp {
    click_count: u32,
}

impl Default for MyApp {
    fn default() -> Self {
        Self { click_count: 0 }
    }
}

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label(format!("Clicks: {}", self.click_count));

            if ui.button("Press me!").clicked() {
                self.click_count += 1;
                ui.label("Button was pressed!");
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Button Clicker",
        options,
        Box::new(|_cc| Box::new(MyApp::default())),
    )
}