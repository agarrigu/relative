extern crate sdl2;

use std::f64::consts::PI;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::audio::AudioSpecDesired;

const DEFAULT_WIDTH: u32    = 800;
const DEFAULT_HEIGHT: u32   = 600;
const LIGHT_BLUE: Color     = Color::RGB(123, 176, 223);
const FRAME_RATE: u32       = 60;
const FRAME_MILI: u32       = 1000 / FRAME_RATE;
const SAMPLE_RATE: i32      = 44_100;
const TABLE_SIZE: usize     = SAMPLE_RATE as usize;
const CHANNELS: u8          = 2;
const C4: f32               = 261.63;
const E4: f32               = 329.63;
const VOLUME: f32           = 0.2;

struct Position {
    pub x: i32,
    pub y: i32,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
        }
    }

    pub fn change(&mut self, dir: &str, speed: i32) {
        match dir {
            "up" => self.y -= speed,
            "down" => self.y += speed,
            "left" => self.x -= speed,
            "right" => self.x += speed,
            _ => {}
        }
    }
}


struct Phase {
    pub left: f32,
    pub right: f32,
}

impl Phase {
    pub fn new() -> Phase {
        Phase {
            left: 0.0,
            right: 0.0,
        }
    }

    pub fn incr(&mut self, freq: f32) {
        let table_size = TABLE_SIZE as f32;
        self.left += freq;
        self.right += freq;
        if self.left >= table_size {self.left -= table_size}
        if self.right >= table_size {self.right -= table_size}
    }
}


fn gen_sqr_wave() -> [f32; TABLE_SIZE] {
    let mut square_wave = [VOLUME; TABLE_SIZE];
    for i in TABLE_SIZE/2..TABLE_SIZE {
        square_wave[i] = -VOLUME;
    } square_wave
}

fn gen_sine_wave() -> [f32; TABLE_SIZE] {
    let mut sine_wave   = [0.0; TABLE_SIZE];
    let mut i           = 0;
    while i < TABLE_SIZE {
        sine_wave[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32 * VOLUME;
        i += 1;
    } sine_wave
}

fn main() {
    /* General stuff */
    let sdl_context = sdl2::init().unwrap();
    /* Graphic stuff */
    let video       = sdl_context.video().unwrap();
    let window      = video.window("rELaTivE", DEFAULT_WIDTH, DEFAULT_HEIGHT).build().unwrap();
    let mut canvas  = window.into_canvas().build().unwrap();
    canvas.set_draw_color(Color::RGB(0, 255, 255));
    canvas.clear();
    canvas.present();
    /* Audio stuff */
    let audio       = sdl_context.audio().unwrap();
    let audio_spec  = AudioSpecDesired {
        freq: Some(SAMPLE_RATE),
        channels: Some(CHANNELS),
        samples: None,
    };
    let device      = audio.open_queue::<f32, _>(None, &audio_spec).unwrap();
    let square_wave = gen_sqr_wave();
    let sine_wave   = gen_sine_wave();
    let mut wave    = [0.0 as f32; ((TABLE_SIZE / FRAME_RATE as usize) * 3) + 1 ];
    device.queue_audio(&wave).unwrap();
    device.resume();
    let mut phase_c4 = Phase::new();
    let mut phase_e4 = Phase::new();
    let mut phase_player = Phase::new();
    let mut player_pitch: f32;
    /* Timer stuff */
    let mut timer = sdl_context.timer().unwrap();
    /* Event stuff */
    let mut event_pump = sdl_context.event_pump().unwrap();
    /* Movement stuff */
    let mut x = 0;
    let mut y = 0;
    let mut move_up     = false;
    let mut move_right  = false;
    let mut move_left   = false;
    let mut move_down   = false;
    /* Loop stuff */
    let mut i = 0;

    /* Main loop */
    'running: loop {
        // let start_time = timer.ticks();
        i = (i + 1) % 102;

        canvas.set_draw_color(Color::RGB(i, 102 - i, 64));
        canvas.clear();

        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => {
                    match keycode {
                        Some(Keycode::Escape) => break 'running,
                        Some(Keycode::W) => move_up     = true,
                        Some(Keycode::S) => move_down   = true,
                        Some(Keycode::A) => move_left   = true,
                        Some(Keycode::D) => move_right  = true,
                        _ => {}
                    }

                },
                Event::KeyUp {keycode, .. } => {
                    match keycode {
                        Some(Keycode::W) => move_up     = false,
                        Some(Keycode::S) => move_down   = false,
                        Some(Keycode::A) => move_left   = false,
                        Some(Keycode::D) => move_right  = false,
                        _ => {}
                    }
                },
                _ => {}
            }
        }

        if move_up      {y -= 2}
        if move_down    {y += 2}
        if move_left    {x -= 2}
        if move_right   {x += 2}

        player_pitch = (880.0 * (y as f32 / DEFAULT_HEIGHT as f32)) + 30.0;

        let rect = Rect::new(x, y, 48, 48);

        canvas.set_draw_color(LIGHT_BLUE);
        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();
        canvas.present();


        if device.size() < wave.len() as u32 * 8 {
            let mut j = 0;
            while j < wave.len()  {
                wave[j]     = (sine_wave[phase_c4.left as usize]
                               + sine_wave[phase_e4.left as usize]
                               + square_wave[phase_player.left as usize]) / 3.0;
                wave[j + 1] = (sine_wave[phase_e4.right as usize]
                               + sine_wave[phase_e4.right as usize]
                               + square_wave[phase_player.right as usize]) / 3.0;
                j += 2;

                phase_c4.incr(C4);
                phase_e4.incr(E4);
                phase_player.incr(player_pitch);
            }
            device.queue_audio(&wave).unwrap();
        }
        // let end_time = timer.ticks();
        // let loop_duration = end_time - start_time;
        // if i == 24 { println!("{:?}", loop_duration);}
        // timer.delay(FRAME_MILI - loop_duration);
        timer.delay(FRAME_MILI);
    }
}
