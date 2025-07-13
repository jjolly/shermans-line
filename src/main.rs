use eframe::egui;

struct TriangleApp;

impl eframe::App for TriangleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sherman's Line Visualization");
        });
    }
}
fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sherman's Line Visualization",
        options,
        Box::new(|_cc| Box::new(TriangleApp)),
    )
}
