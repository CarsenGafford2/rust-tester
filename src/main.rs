use eframe::egui;

struct MyApp;

impl eframe::App for MyApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.label("Clicks: 0");
            if ui.button("Press me!").clicked() {
                ui.label("Button was pressed!");
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let mut options = eframe::NativeOptions::default();
    options.vsync = true;
    eframe::run_native(
        "Button Clicker",
        options,
        Box::new(|_cc| Box::new(MyApp)),
    )
}