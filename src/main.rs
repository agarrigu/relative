extern crate sdl2;
use sdl2::pixels::Color;
use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::rect::Rect;
use sdl2::audio::AudioSpecDesired;

mod audio;
use audio::{gen_sqr_wave, gen_sine_wave, Phase};
use audio::{SAMPLE_RATE, CHANNELS, WAVE_SIZE};
mod control;
use control::{Position, Dire};


const DEFAULT_WIDTH:    u32 = 800;
const DEFAULT_HEIGHT:   u32 = 600;
const SQUARE_SIZE:      u32 = 48;
const MAX_Y:            u32 = DEFAULT_HEIGHT - SQUARE_SIZE;
const LIGHT_BLUE:     Color = Color::RGB(123, 176, 223);
const GREEN_BLUE:     Color = Color::RGB(0, 255, 255);
const FRAME_RATE:       u32 = 60;
const FRAME_MILI:       u32 = 1000 / FRAME_RATE;
const E1:               f32 = 41.20;
const C4:               f32 = 261.63;
const E4:               f32 = 329.63;
const G6:               f32 = 1567.98;
const SPEED:            i32 = 2;


fn main() {

    /* Main init */
    let sdl_context = sdl2::init().unwrap();

    /* Graphic stuff */
    let video       = sdl_context.video().unwrap();
    let window      = video.window("rELaTivE", DEFAULT_WIDTH, DEFAULT_HEIGHT).build().unwrap();
    let mut canvas  = window.into_canvas().build().unwrap();
    canvas.set_draw_color(GREEN_BLUE);
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
    let octave_count = (G6 / E1).log2();
    let square_wave = gen_sqr_wave();
    let sine_wave   = gen_sine_wave();
    let mut wave    = [0.0 as f32; WAVE_SIZE];
    device.queue_audio(&wave).unwrap();
    device.resume();
    let mut phase_c4        = Phase::new();
    let mut phase_e4        = Phase::new();
    let mut phase_player    = Phase::new();
    let mut player_pitch: f32;

    /* Timer stuff */
    let mut timer = sdl_context.timer().unwrap();

    /* Event stuff */
    let mut event_pump = sdl_context.event_pump().unwrap();

    /* Movement stuff */
    let mut position = Position::new(0, 0);
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


        if move_up      {position.change(Dire::Up,      SPEED)}
        if move_down    {position.change(Dire::Down,    SPEED)}
        if move_left    {position.change(Dire::Left,    SPEED)}
        if move_right   {position.change(Dire::Right,   SPEED)}

        if position.x < 0 {position.change(Dire::Right, SPEED)}
        if position.y < 0 {position.change(Dire::Down,  SPEED)}
        if position.x >= MAX_Y as i32 {
            position.change(Dire::Left, SPEED)
        }
        if position.y >= MAX_Y as i32 {
            position.change(Dire::Up, SPEED)
        }

        // player_pitch = k * (position.y as f32).log2() + c;
        let player_octave = position.y as f32 * octave_count / MAX_Y as f32;
        player_pitch = 2_f32.powf(player_octave) * E1;


        let rect = Rect::new(position.x, position.y, SQUARE_SIZE, SQUARE_SIZE);

        canvas.set_draw_color(LIGHT_BLUE);
        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();
        canvas.present();

        if device.size() < wave.len() as u32 * 3 {
            let mut j = 0;
            while j < wave.len()  {
                wave[j]     = (sine_wave[phase_c4.left as usize]
                               + square_wave[phase_e4.left as usize]
                               + sine_wave[phase_player.left as usize]) / 3.0;
                wave[j + 1] = (square_wave[phase_e4.right as usize]
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
