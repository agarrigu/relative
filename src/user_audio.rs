pub mod consts {
    use std::mem::size_of;
    const FRAME_RATE:      u32   = 60;
    pub const SAMPLE_RATE: i32   = 48_000;
    pub const TABLE_SIZE:  usize = SAMPLE_RATE as usize;
    pub const VOLUME:      f32   = 0.07;
    pub const CHANNELS:    u8    = 2;
    pub const WAVE_SIZE:   usize = size_of::<f32>()
                                   * CHANNELS as usize
                                   * (TABLE_SIZE / FRAME_RATE as usize);
}

pub mod phase {
    use crate::user_audio::consts::TABLE_SIZE;

    pub struct Phase {
        // TODO: Impl get method, privatize left and right, maybe?
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

        pub fn next_ampl(&mut self, freq: f32) {
            let table_size = TABLE_SIZE as f32;
            self.left += freq;
            self.right += freq;
            if self.left >= table_size {self.left -= table_size}
            if self.right >= table_size {self.right -= table_size}
        }
    }
}

pub mod wave_gens {
    use std::f64::consts::PI;
    use crate::user_audio::consts::{TABLE_SIZE, VOLUME};

    pub fn gen_sqr_wave() -> [f32; TABLE_SIZE] {
        let mut square_wave = [VOLUME; TABLE_SIZE];
        for i in TABLE_SIZE/2..TABLE_SIZE {
            square_wave[i] = -VOLUME;
        } square_wave
    }

    pub fn gen_sine_wave() -> [f32; TABLE_SIZE] {
        let mut sine_wave   = [0.0; TABLE_SIZE];
        let mut i           = 0;
        while i < TABLE_SIZE {
            sine_wave[i] = (i as f64 / TABLE_SIZE as f64 * PI * 2.0).sin() as f32 * VOLUME;
            i += 1;
        } sine_wave
    }

    pub fn gen_tri_wave() -> [f32; TABLE_SIZE] {
        let mut triangle_wave = [0.0; TABLE_SIZE];
        let mut i = 0;
        let mut amplitude = -1f32;
        let step = 2f32 / TABLE_SIZE as f32;
        while i < TABLE_SIZE {
            triangle_wave[i] = amplitude;
            amplitude += step;
            i += 1;
        } triangle_wave
    }
}
