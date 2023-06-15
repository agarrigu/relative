use sdl2::pixels::Color;
use sdl2::render::Canvas;
use sdl2::video::Window;
use sdl2::ttf::Sdl2TtfContext;
use sdl2::TimerSubsystem;
use sdl2::EventPump;
use sdl2::event::Event;
use sdl2::rect::Rect;
use sdl2::keyboard::Keycode;
use sdl2::audio::AudioQueue;
use crate::notes::notes::{Notes, gen_note_map};
use crate::user_audio::wave_gens::gen_sine_wave;
use crate::user_audio::consts::WAVE_SIZE;
use crate::user_audio::phase::Phase;
use crate::control::movement::{Position, move_avatar};
use crate::user_video::consts::{FRAME_MILI,MAX_X, MAX_Y, SQUARE_SIZE};

const LIGHT_GRAY: Color = Color::RGB(220, 220, 200);
const LIGHT_BLUE: Color = Color::RGB(123, 176, 223);
const SOLID_BLACK: Color = Color::RGB(0, 0, 0);

#[derive(PartialEq, Eq, Clone, Copy)]
enum StateOctave{
    Found,
    NotFound,
    CompleteLevel
}

fn set_background_color(state_octave: StateOctave, current_time: u32 , start_time: u32) -> Color {
    let color: Color;
    match state_octave {
        StateOctave::NotFound => color = LIGHT_GRAY,
        StateOctave::CompleteLevel => color = LIGHT_BLUE,
        StateOctave::Found => {
            let mut traspased_time = (current_time - start_time) as f32;
            if traspased_time < 5000.0 {
                color = LIGHT_GRAY;
            } else {
                traspased_time -= 5000.0;
                let r: u8 = (((traspased_time / 5000.0)
                              * (LIGHT_GRAY.r as f32 - LIGHT_BLUE.r as f32))
                              + LIGHT_BLUE.r as f32) as u8;
                let g: u8 = (((traspased_time / 5000.0)
                              * (LIGHT_GRAY.g as f32 - LIGHT_BLUE.g as f32))
                              + LIGHT_BLUE.g as f32) as u8;
                let b: u8 = (((traspased_time / 5000.0)
                              * (LIGHT_GRAY.b as f32 - LIGHT_BLUE.b as f32))
                              + LIGHT_BLUE.b as f32) as u8;
                color = Color::RGB(r, g, b)
            }
        }
    } color
}

pub fn level_0(canvas: &mut Canvas<Window>,
               fonts: &mut Sdl2TtfContext,
               timer: &mut TimerSubsystem,
               event_pump: &mut EventPump,
               audio_queue: &AudioQueue<f32>,
               wave: &mut [f32; WAVE_SIZE ]) {

    let note_map = gen_note_map();
    let octave_count = (note_map.get(&Notes::G6).unwrap()
                       / note_map.get(&Notes::E1).unwrap()).log2();
    let sine_wave = gen_sine_wave();
    let mut phase_player = Phase::new();
    let mut player_pitch: f32;

    /* Movement stuff */
    let mut position = Position {x: MAX_X as i32 / 2, y: MAX_Y as i32 / 2 };

    /* State stuff */
    let mut current_state_octave = StateOctave::NotFound;
    let mut start_found_time: Option<u32> = None;
    let mut background_color = LIGHT_GRAY;

    'running: loop {
        if current_state_octave == StateOctave::CompleteLevel {break}
        let current_time = timer.ticks();

        /* State stuff */
        let last_state_octave = current_state_octave;
        if position.y == 300 {
            current_state_octave = StateOctave::Found;
            if last_state_octave == StateOctave::NotFound {
                start_found_time = Some(current_time);
            }
        } else {
            start_found_time = None;
        }

        match start_found_time {
            Some(time) => {
                let transp_found_time = current_time - time;
                if transp_found_time > 10000 {
                    current_state_octave = StateOctave::CompleteLevel;
                } else {
                    background_color = set_background_color(current_state_octave, current_time, time);
                }
            },
            None => {background_color = LIGHT_GRAY}
        }

        /* Movement stuff */
        let events: Vec<Event> = event_pump.poll_iter().collect();
        for event in &events {
            match event {
                Event::KeyDown {keycode, ..} => match keycode {
                    Some(Keycode::Escape) => break 'running,
                    _ => {}
                }
                _ => {}
            }
        }
        move_avatar(&mut position, events);

        /* Audio */
        let player_octave = position.y as f32 * octave_count / MAX_Y as f32;
        player_pitch = 2_f32.powf(player_octave)
                       * note_map.get(&Notes::E1).unwrap();
        if audio_queue.size() < wave.len() as u32 * 6 {
            let mut j = 0;
            while j < wave.len() {
                wave[j] = sine_wave[phase_player.left as usize];
                wave[j + 1] = sine_wave[phase_player.right as usize];
                j += 2;
                phase_player.next_ampl(player_pitch);
            }
            audio_queue.queue_audio(wave).unwrap();
        }

        /* Graphix */
        let rect = Rect::new(position.x, position.y, SQUARE_SIZE, SQUARE_SIZE);
        let proggy = fonts.load_font("assets/fonts/ProggyTiny.ttf", 12).unwrap();
        let surface = proggy.render(&position.y.to_string()).solid(SOLID_BLACK).unwrap();
        let tetxture_creator = canvas.texture_creator();
        let texture = tetxture_creator.create_texture_from_surface(&surface).unwrap();


        canvas.set_draw_color(background_color);
        canvas.clear();
        canvas.set_draw_color(LIGHT_BLUE);
        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();
        canvas.copy(&texture, None, rect).unwrap();
        canvas.present();

        /* Frame stuff */
        let end_time = timer.ticks();
        let loop_duration = end_time - current_time;
        if loop_duration > 0 { timer.delay(FRAME_MILI) }
    }
}
