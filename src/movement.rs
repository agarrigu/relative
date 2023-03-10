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
