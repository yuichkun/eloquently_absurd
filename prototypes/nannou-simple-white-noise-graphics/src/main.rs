extern crate nannou;
use nannou::prelude::*;

fn main() {
    println!("Hello, world!");
    nannou::app(model)
        .update(update)
        .simple_window(view)
        .run();
}

struct Model {}

fn model(_app: &App) -> Model {
    Model {}
}

fn update(_app: &App, _model: &mut Model, _update: Update) {
    println!("update");
}

fn view(_app: &App, _model: &Model, frame: Frame){
    println!("view");
    frame.clear(RED);
}
