use eframe::emath::Vec2;
mod app;
fn main() {
    let options = eframe::NativeOptions {
        initial_window_size: Some(Vec2 {
            x: 1280.0,
            y: 720.0,
        }),
        ..Default::default()
    };
    eframe::run_native(
        "RSZ Editor",
        options,
        Box::new(|cc| Box::new(app::Editor::new(cc))),
    )
    .expect("Failed to start GUI!");
}
