pub mod consts {
    pub const DEFAULT_WIDTH: u32 = 800;
    pub const DEFAULT_HEIGHT: u32 = 600;
    pub const SQUARE_SIZE: u32 = 48;
    pub const MAX_Y: u32 = DEFAULT_HEIGHT - SQUARE_SIZE;
    pub const MAX_X: u32 = DEFAULT_WIDTH - SQUARE_SIZE;
    const DEFAULT_FRAME_RATE: u32 = 60;
    pub const FRAME_MILI: u32 = 1000 / DEFAULT_FRAME_RATE;
}
