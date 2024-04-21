use super::{HEIGHT, WIDTH};
use nannou_audio as audio;
use ringbuf::{Rb, StaticRb};
use std::sync::{Arc, Mutex};

pub const RB_SIZE: usize = WIDTH * HEIGHT;

pub type AppAudioBuffer = Arc<Mutex<StaticRb<f32, RB_SIZE>>>;

pub type RecorderInStream = audio::Stream<RecorderModel>;

pub struct RecorderModel {
    rb: AppAudioBuffer,
}

pub fn create() -> (AppAudioBuffer, RecorderInStream) {
    let rb = Arc::new(Mutex::new(StaticRb::<f32, RB_SIZE>::default()));
    let input_rb = rb.clone();
    let recorder_model = RecorderModel { rb: input_rb };
    let audio_host = audio::Host::new();
    let in_stream = audio_host
        .new_input_stream(recorder_model)
        .capture(pass_in)
        .build()
        .unwrap();
    in_stream.play().unwrap();
    return (rb, in_stream);
}

fn pass_in(model: &mut RecorderModel, buffer: &nannou_audio::Buffer) {
    // if rb is full, empty it first
    {
        if model.rb.lock().unwrap().len() == RB_SIZE {
            println!("rb full, emptying");
            model.rb.lock().unwrap().clear();
        }
    }

    buffer.frames().for_each(|frame| {
        let ch1 = frame.get(0).unwrap();
        model.rb.lock().unwrap().push_overwrite(*ch1);
    });
}
pub fn collect_samples(rb: &AppAudioBuffer) -> Vec<f32> {
    let rb = rb.lock().unwrap();
    rb.iter().copied().collect()
}
