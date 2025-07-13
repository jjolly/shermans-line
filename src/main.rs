use eframe::egui;

fn distance(a: egui::Pos2, b: egui::Pos2) -> f32 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

// Incenter and inradius
fn incenter_and_inradius(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> (egui::Pos2, f32) {
    let side_a = distance(b, c);
    let side_b = distance(a, c);
    let side_c = distance(a, b);
    let sum = side_a + side_b + side_c;
    let center = egui::pos2(
        (side_a * a.x + side_b * b.x + side_c * c.x) / sum,
        (side_a * a.y + side_b * b.y + side_c * c.y) / sum,
    );
    // Heron's formula for area
    let s = sum / 2.0;
    let area = (s * (s - side_a) * (s - side_b) * (s - side_c)).sqrt();
    let radius = area / s;
    (center, radius)
}

// Circumcenter and circumradius
fn circumcenter_and_radius(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> (egui::Pos2, f32) {
    let d = 2.0 * (a.x * (b.y - c.y) + b.x * (c.y - a.y) + c.x * (a.y - b.y));
    let ux = ((a.x.powi(2) + a.y.powi(2)) * (b.y - c.y)
        + (b.x.powi(2) + b.y.powi(2)) * (c.y - a.y)
        + (c.x.powi(2) + c.y.powi(2)) * (a.y - b.y))
        / d;
    let uy = ((a.x.powi(2) + a.y.powi(2)) * (c.x - b.x)
        + (b.x.powi(2) + b.y.powi(2)) * (a.x - c.x)
        + (c.x.powi(2) + c.y.powi(2)) * (b.x - a.x))
        / d;
    let center = egui::pos2(ux, uy);
    let radius = distance(center, a);
    (center, radius)
}

// Orthocenter (needed for nine-point circle)
fn orthocenter(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> egui::Pos2 {
    // Direction vectors for sides
    let ab = (b.x - a.x, b.y - a.y);
    let ac = (c.x - a.x, c.y - a.y);
    let bc = (c.x - b.x, c.y - b.y);

    // Altitude from A: perpendicular to BC, passes through A
    let bc_perp = (bc.1, -bc.0); // Rotate vector 90 degrees
    // Altitude from B: perpendicular to AC, passes through B
    let ac_perp = (ac.1, -ac.0);

    // Solve for intersection of two lines:
    // Line 1: A + t1 * bc_perp
    // Line 2: B + t2 * ac_perp

    let denom = ac_perp.0 * bc_perp.1 - ac_perp.1 * bc_perp.0;
    if denom.abs() < 1e-6 {
        // Degenerate triangle
        return egui::pos2((a.x + b.x + c.x) / 3.0, (a.y + b.y + c.y) / 3.0);
    }

    let dx = b.x - a.x;
    let dy = b.y - a.y;

    let t1 = (ac_perp.0 * dy - ac_perp.1 * dx) / denom;
    let ox = a.x + t1 * bc_perp.0;
    let oy = a.y + t1 * bc_perp.1;

    egui::pos2(ox, oy)
}

// Nine-point circle
fn nine_point_circle(a: egui::Pos2, b: egui::Pos2, c: egui::Pos2) -> (egui::Pos2, f32) {
    let (circumcenter, circumradius) = circumcenter_and_radius(a, b, c);
    let ortho = orthocenter(a, b, c);
    let center = egui::pos2(
        (circumcenter.x + ortho.x) / 2.0,
        (circumcenter.y + ortho.y) / 2.0,
    );
    let radius = circumradius / 2.0;
    (center, radius)
}

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

            let [a, b, c] = self.vertices;
            let (incenter, inradius) = incenter_and_inradius(a, b, c);
            let (circumcenter, circumradius) = circumcenter_and_radius(a, b, c);
            let (nine_point_center, nine_point_radius) = nine_point_circle(a, b, c);

            painter.circle_stroke(incenter, inradius, egui::Stroke::new(2.0, egui::Color32::GREEN));
            painter.circle_stroke(circumcenter, circumradius, egui::Stroke::new(2.0, egui::Color32::BLUE));
            painter.circle_stroke(nine_point_center, nine_point_radius, egui::Stroke::new(2.0, egui::Color32::YELLOW));

            for (i, v) in self.vertices.iter_mut().enumerate() {
                let radius = 8.0;
                let response = ui.interact(
                    egui::Rect::from_center_size(*v, egui::vec2(radius * 2.0, radius * 2.0)),
                    ui.make_persistent_id(i),
                    egui::Sense::click_and_drag(),
                );
               if response.dragged() {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::Grabbing);
                    *v += response.drag_delta();
                }
                else if response.hovered() {
                    ui.output_mut(|o| o.cursor_icon = egui::CursorIcon::PointingHand);
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
