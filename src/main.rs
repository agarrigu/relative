extern crate sdl2;
use sdl2::audio::AudioSpecDesired;

mod user_audio;
mod user_video;
mod notes;
mod control;

use user_video::consts::{DEFAULT_WIDTH, DEFAULT_HEIGHT};
use user_audio::phase::Phase;
use user_audio::consts::{CHANNELS, SAMPLE_RATE, WAVE_SIZE};

mod level_0;
use level_0::level_0;


fn main() {
    /* Main init */
    let sdl_context = sdl2::init().unwrap();

    /* Graphic stuff */
    let video = sdl_context.video().unwrap();
    let window = video
        .window("rELaTivE", DEFAULT_WIDTH, DEFAULT_HEIGHT)
        .build()
        .unwrap();
    let mut canvas = window.into_canvas().build().unwrap();

    /* Audio stuff */
    let audio            = sdl_context.audio().unwrap();
    let audio_spec       = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(CHANNELS),
        samples: None,
    };
    let device = audio.open_queue::<f32, _>(None, &audio_spec).unwrap();
    let mut wave = [0.0 as f32; WAVE_SIZE];
    device.queue_audio(&wave).unwrap();
    // TODO: Fix resume device before wave has valid data
    device.resume();

    /* Timer stuff */
    let mut timer = sdl_context.timer().unwrap();

    /* Event stuff */
    let mut event_pump = sdl_context.event_pump().unwrap();

    /* Levels */
    level_0(&mut canvas, &mut timer, &mut event_pump, &device, &mut wave);
}
