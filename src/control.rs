pub struct Position {
    pub x: i32,
    pub y: i32,
    speed: i32,
}

pub enum Dire {
    Up,
    Down,
    Left,
    Right,
}

impl Position {
    pub fn new(x: i32, y: i32) -> Position {
        Position {
            x: x,
            y: y,
            speed: 2,
        }
    }

    pub fn change(&mut self, dire: Dire, speed: i32) {
        match dire {
            Dire::Up    => self.y -= speed,
            Dire::Down  => self.y += speed,
            Dire::Left  => self.x -= speed,
            Dire::Right => self.x += speed,
        }
    }
}
