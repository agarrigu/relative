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
use crate::user_audio::phase::Phase;
use crate::control::movement::{Position, move_avatar};
use crate::user_video::consts::{FRAME_MILI, MAX_Y, SQUARE_SIZE};

pub fn level_0(canvas: &mut Canvas<Window>,
               timer: &mut TimerSubsystem,
               event_pump: &mut EventPump,
               audio_queue: &AudioQueue<f32>,
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
    let mut position = Position {x: 0, y: 0 };

    'running: loop {
        let start_time = timer.ticks();

        // let poll_iter = event_pump.poll_iter();

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
            // for amp in wave.iter(). { }
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
        canvas.set_draw_color(light_gray);
        canvas.clear();
        canvas.set_draw_color(light_blue);
        canvas.draw_rect(rect).unwrap();
        canvas.fill_rect(rect).unwrap();
        canvas.present();

        /* Frame stuff */
        let end_time = timer.ticks();
        let loop_duration = end_time - start_time;
        if loop_duration > 0 { timer.delay(FRAME_MILI) }
    }
}
