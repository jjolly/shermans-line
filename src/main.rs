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

fn midpoint(a: egui::Pos2, b: egui::Pos2) -> egui::Pos2 {
    egui::pos2((a.x + b.x) / 2.0, (a.y + b.y) / 2.0)
}

fn foot_of_perpendicular(p: egui::Pos2, a: egui::Pos2, b: egui::Pos2) -> egui::Pos2 {
    let ap = (p.x - a.x, p.y - a.y);
    let ab = (b.x - a.x, b.y - a.y);
    let ab_len2 = ab.0 * ab.0 + ab.1 * ab.1;
    let dot = ap.0 * ab.0 + ap.1 * ab.1;
    let t = dot / ab_len2;
    let proj = (a.x + t * ab.0, a.y + t * ab.1);
    egui::pos2(proj.0, proj.1)
}

fn extend_line(a: egui::Pos2, b: egui::Pos2, factor: f32) -> (egui::Pos2, egui::Pos2) {
    let dx = b.x - a.x;
    let dy = b.y - a.y;
    let new_a = egui::pos2(a.x - dx * factor, a.y - dy * factor);
    let new_b = egui::pos2(b.x + dx * factor, b.y + dy * factor);
    (new_a, new_b)
}

struct TriangleApp {
    vertices: [egui::Pos2; 3],
    show_perpendiculars: bool,
    show_ortho_segments: bool,
    show_side_midpoints: bool,
    show_feet_of_altitudes: bool,
    show_ortho_vertex_midpoints: bool,
    show_extensions: bool,
}

impl Default for TriangleApp {
    fn default() -> Self {
        Self {
            vertices: [
                egui::Pos2::new(200.0, 200.0),
                egui::Pos2::new(400.0, 200.0),
                egui::Pos2::new(300.0, 400.0)],
            show_perpendiculars: false,
            show_ortho_segments: false,
            show_side_midpoints: false,
            show_feet_of_altitudes: false,
            show_ortho_vertex_midpoints: false,
            show_extensions: false,
        }
    }
}

impl eframe::App for TriangleApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            ui.heading("Sherman's Line Visualization");

            ui.horizontal(|ui| {
                ui.checkbox(&mut self.show_perpendiculars, "Show altitudes");
                ui.checkbox(&mut self.show_ortho_segments, "Show orthocenter-vertex segments");
                ui.checkbox(&mut self.show_side_midpoints, "Show side midpoints");
                ui.checkbox(&mut self.show_feet_of_altitudes, "Show feet of altitudes");
                ui.checkbox(&mut self.show_ortho_vertex_midpoints, "Show orthocenter-vertex midpoints");
                ui.checkbox(&mut self.show_extensions, "Show extensions")
            });
            
            let factor = 2.0; // Adjust for your window size
            let painter = ui.painter();
            for i in 0..3 {
                let a = self.vertices[i];
                let b = self.vertices[(i + 1) % 3];
                painter.line_segment([a, b], egui::Stroke::new(2.0, egui::Color32::WHITE));
                if self.show_extensions {
                    let (ext1, ext2) = extend_line(a, b, factor);
                    painter.line_segment([ext1, ext2], egui::Stroke::new(1.0, egui::Color32::DARK_GRAY));
                }
            }

            if self.show_perpendiculars {
                let [a, b, c] = self.vertices;
                let foot_a = foot_of_perpendicular(a, b, c);
                let foot_b = foot_of_perpendicular(b, a, c);
                let foot_c = foot_of_perpendicular(c, a, b);
                painter.line_segment([a, foot_a], egui::Stroke::new(1.5, egui::Color32::LIGHT_GREEN));
                painter.line_segment([b, foot_b], egui::Stroke::new(1.5, egui::Color32::LIGHT_GREEN));
                painter.line_segment([c, foot_c], egui::Stroke::new(1.5, egui::Color32::LIGHT_GREEN));
                if self.show_extensions {
                    let ortho = orthocenter(a, b, c);
                    let feet = [foot_a, foot_b, foot_c];
                    let verts = [a, b, c];
                    for i in 0..3 {
                        let (ext1, ext2) = extend_line(verts[i], feet[i], factor);
                        painter.line_segment([ext1, ext2], egui::Stroke::new(1.0, egui::Color32::DARK_GREEN));
                    }
                }
            }

            if self.show_ortho_segments {
                let [a, b, c] = self.vertices;
                let ortho = orthocenter(a, b, c);
                painter.line_segment([a, ortho], egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE));
                painter.line_segment([b, ortho], egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE));
                painter.line_segment([c, ortho], egui::Stroke::new(1.5, egui::Color32::LIGHT_BLUE));
            }

            if self.show_side_midpoints {
                let [a, b, c] = self.vertices;
                let m_ab = midpoint(a, b);
                let m_bc = midpoint(b, c);
                let m_ca = midpoint(c, a);
                for m in [m_ab, m_bc, m_ca] {
                    painter.circle_filled(m, 5.0, egui::Color32::GOLD);
                }
            }

            if self.show_feet_of_altitudes {
                let [a, b, c] = self.vertices;
                let foot_a = foot_of_perpendicular(a, b, c);
                let foot_b = foot_of_perpendicular(b, a, c);
                let foot_c = foot_of_perpendicular(c, a, b);
                for f in [foot_a, foot_b, foot_c] {
                    painter.circle_filled(f, 5.0, egui::Color32::RED);
                }
            }

            if self.show_ortho_vertex_midpoints {
                let [a, b, c] = self.vertices;
                let ortho = orthocenter(a, b, c);
                let m_a = midpoint(a, ortho);
                let m_b = midpoint(b, ortho);
                let m_c = midpoint(c, ortho);
                for m in [m_a, m_b, m_c] {
                    painter.circle_filled(m, 5.0, egui::Color32::LIGHT_RED);
                }
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
