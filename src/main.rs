use egui_macroquad::egui;
use macroquad::{main, prelude::*};

#[main("Macroquad Example")]
async fn main() {
    let mut name = "World".to_string();
    let mut sides = 8;
    loop {
        clear_background(WHITE);
        let [x, y] = [screen_width() * 0.5, screen_height() * 0.5];
        draw_poly(x, y, sides, 200., 90., GRAY);
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Main Window").show(ctx, |ui| {
                ui.heading(&format!("Hello {name}!"));
                ui.label("What's your name?");
                ui.text_edit_singleline(&mut name);
                let drag = egui::DragValue::new(&mut sides)
                    .prefix("Edges: ")
                    .clamp_range(3..=20);
                ui.add(drag);
            });
        });
        egui_macroquad::draw();
        next_frame().await;
    }
}
