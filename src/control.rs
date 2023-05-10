pub mod direccion {

    pub struct Position {
        pub x: i32,
        pub y: i32,
        speed: i32,
    }

    pub struct Movement {
        move_x: bool,
        move_y: bool,
    }

    pub enum Direction {
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

        pub fn change(&mut self, direction: Direction) {
            match direction {
                Direction::Up    => self.y -= self.speed,
                Direction::Down  => self.y += self.speed,
                Direction::Left  => self.x -= self.speed,
                Direction::Right => self.x += self.speed,
            }
        }
    }
}
