use nannou_egui::{self, egui, Egui};
pub struct AppUi {
    pub egui: Egui,
    pub settings: Settings,
}
pub struct Settings {
    pub resolution: f32,
    pub amp: f32,
}

pub fn create_ui(window: &nannou::prelude::Window) -> AppUi {
    let egui = Egui::from_window(window);

    let settings = create_initial_settings();

    return AppUi { egui, settings };
}

pub fn update_settings_ui(app_ui: &mut AppUi) {
    let egui = &mut app_ui.egui;
    let settings = &mut app_ui.settings;

    let ctx = egui.begin_frame();
    egui::Window::new("Settings").show(&ctx, |ui| {
        ui.label("Resolution:");
        if ui
            .add(egui::Slider::new(&mut settings.resolution, 0.0001..=0.0005))
            .changed()
        {}

        ui.label("Amp:");
        if ui
            .add(egui::Slider::new(&mut settings.amp, 1.0..=10.0))
            .changed()
        {}
    });
}

fn create_initial_settings() -> Settings {
    let settings = Settings {
        resolution: 0.00046,
        amp: 1.0,
    };
    settings
}
