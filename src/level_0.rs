use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::TimerSubsystem;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::audio::AudioQueue;
use crate::notes::notes::{Notes, gen_note_map};
use crate::user_audio::wave_gens::gen_sine_wave;
use crate::user_audio::consts::WAVE_SIZE;
use crate::Phase;
use crate::control::direccion::{Position, Direction};
use crate::user_video::consts::{FRAME_MILI, MAX_X, MAX_Y, SQUARE_SIZE};

pub fn level_0(canvas: &mut Canvas<Window>,
               timer: &mut TimerSubsystem,
               event_pump: &mut EventPump,
               device: &AudioQueue<f32>,
               wave: &mut [f32; WAVE_SIZE ]) {

    let note_map = gen_note_map();
    let light_blue = Color::RGB(123, 176, 223);
    let light_gray = Color::RGB(220, 220, 220);
    let octave_count = (note_map.get(&Notes::G6).unwrap()
                       / note_map.get(&Notes::E1).unwrap()).log2();
    let sine_wave = gen_sine_wave();
    let mut phase_player = Phase::new();
    let mut player_pitch: f32;

    /* Movement stuff */
    let mut position   = Position::new(0, 0);
    let mut move_up    = false;
    let mut move_right = false;
    let mut move_left  = false;
    let mut move_down  = false;
    'running: loop {
        let start_time = timer.ticks();

        canvas.set_draw_color(light_gray);
        canvas.clear();

        // TODO: Make better
        for event in event_pump.poll_iter() {
            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    Some(Keycode::W) => move_up    = true,
                    Some(Keycode::S) => move_down  = true,
                    Some(Keycode::A) => move_left  = true,
                    Some(Keycode::D) => move_right = true,
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::W) => move_up    = false,
                    Some(Keycode::S) => move_down  = false,
                    Some(Keycode::A) => move_left  = false,
                    Some(Keycode::D) => move_right = false,
                    _ => {}
                },
                _ => {}
            }
        }

        if move_up    { position.change(Direction::Up) }
        if move_down  { position.change(Direction::Down) }
        if move_left  { position.change(Direction::Left) }
        if move_right { position.change(Direction::Right) }

        if position.x < 0             { position.change(Direction::Right) }
        if position.y < 0             { position.change(Direction::Down) }
        if position.x >= MAX_X as i32 { position.change(Direction::Left) }
        if position.y >= MAX_Y as i32 { position.change(Direction::Up) }

        let player_octave = position.y as f32 * octave_count / MAX_Y as f32;
        player_pitch = 2_f32.powf(player_octave) * note_map.get(&Notes::E1).unwrap();

        let rect = Rect::new(position.x, position.y, SQUARE_SIZE, SQUARE_SIZE);

        canvas.set_draw_color(light_blue);
        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();
        canvas.present();

        if device.size() < wave.len() as u32 * 2 {
            let mut j = 0;
            while j < wave.len() {
                wave[j] = sine_wave[phase_player.left as usize];
                wave[j + 1] = sine_wave[phase_player.right as usize];
                j += 2;

                phase_player.next_ampl(player_pitch);
            }
            device.queue_audio(wave).unwrap();
        }

        let end_time = timer.ticks();
        let loop_duration = end_time - start_time;
        if loop_duration > 0 { timer.delay(FRAME_MILI - loop_duration) }
    }
}
