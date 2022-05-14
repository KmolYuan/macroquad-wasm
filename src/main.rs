use egui_macroquad::egui;
use macroquad::{main, prelude::*};

#[main("Macroquad Example")]
async fn main() {
    let mut name = "World".to_string();
    loop {
        clear_background(WHITE);
        let [x, y] = [screen_width() * 0.5, screen_height() * 0.5];
        draw_poly(x, y, 8, 200., 0., GRAY);
        egui_macroquad::ui(|ctx| {
            egui::Window::new("Main Window").show(ctx, |ui| {
                ui.heading(&format!("Hello {name}!"));
                ui.label("What's your name?");
                ui.text_edit_singleline(&mut name);
            });
        });
        egui_macroquad::draw();
        next_frame().await;
    }
}
