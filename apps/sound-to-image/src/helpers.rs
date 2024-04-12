use nannou::prelude::*;

pub trait AppHelpers {
    fn show_fps(&self, frame: &Frame);
}

impl AppHelpers for App {
    fn show_fps(&self, frame: &Frame) {
        let app = self;
        let fps = app.fps();
        let fps_text = format!("FPS: {:.2}", fps);

        // Position the text in the top right corner
        let draw = app.draw();
        draw.text(&fps_text)
            .color(WHITE)
            .font_size(16)
            .xy(app.window_rect().top_right() + vec2(-60.0, -20.0)); // Adjust the position as needed
        draw.to_frame(app, &frame).unwrap();
    }
}
