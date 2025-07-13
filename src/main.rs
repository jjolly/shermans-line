use eframe::egui;

struct TriangleApp {
    vertices: [egui::Pos2; 3],
}

impl Default for TriangleApp {
    fn default() -> Self {
        Self {
            vertices: [
                egui::Pos2::new(200.0, 200.0),
                egui::Pos2::new(400.0, 200.0),
                egui::Pos2::new(300.0, 400.0)],
        }
    }
}

impl eframe::App for TriangleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sherman's Line Visualization");

            let painter = ui.painter();
            for i in 0..3 {
                let a = self.vertices[i];
                let b = self.vertices[(i + 1) % 3];
                painter.line_segment([a, b], egui::Stroke::new(2.0, egui::Color32::WHITE));
            }

            for (i, v) in self.vertices.iter_mut().enumerate() {
                let radius = 8.0;
                let response = ui.interact(
                    egui::Rect::from_center_size(*v, egui::vec2(radius * 2.0, radius * 2.0)),
                    ui.make_persistent_id(i),
                    egui::Sense::click_and_drag(),
                );
               if response.dragged() {
                    *v += response.drag_delta();
                }
                painter.circle_filled(*v, radius, egui::Color32::RED);
            }
        });
    }
}

fn main() -> eframe::Result<()> {
    let options = eframe::NativeOptions::default();
    eframe::run_native(
        "Sherman's Line Visualization",
        options,
        Box::new(|_cc| Box::new(TriangleApp::default())),
    )
}
