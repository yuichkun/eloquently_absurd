use nannou_audio as audio;
use ringbuf::{Rb, StaticRb};
use std::sync::{Arc, Mutex};

pub const RB_SIZE: usize = 1024;

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

pub fn update() {}

fn pass_in(model: &mut RecorderModel, buffer: &nannou_audio::Buffer) {
    for frame in buffer.frames() {
        for sample in frame {
            model.rb.lock().unwrap().push_overwrite(*sample);
        }
    }
}
pub fn collect_samples(rb: &AppAudioBuffer) -> Vec<f32> {
    let rb = rb.lock().unwrap();
    rb.iter().copied().collect()
}
