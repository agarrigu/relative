pub mod movement {
    use sdl2::event::{Event, EventPollIterator};
    use sdl2::keyboard::Keycode;
    use crate::user_video::consts::{MAX_Y, MAX_X};

    const SPEED: i32 = 6;

    pub struct Position {
        pub x: i32,
        pub y: i32,
    }

    struct Want {
        up:    bool,
        down:  bool,
        left:  bool,
        right: bool,
    }

    pub enum Way {
        Up,
        Down,
        Left,
        Right,
    }

    impl Position {
        pub fn change(&mut self, way: Way) {
            match way {
                Way::Up    => self.y -= SPEED,
                Way::Down  => self.y += SPEED,
                Way::Left  => self.x -= SPEED,
                Way::Right => self.x += SPEED,
            }
        }
    }

    impl Want {
        fn new() -> Want {
            Want {
                up:    false,
                down:  false,
                left:  false,
                right: false,
            }
        }

        fn request(&mut self, way: Way) {
            match way {
                Way::Up    => self.up    = true,
                Way::Down  => self.down  = true,
                Way::Left  => self.left  = true,
                Way::Right => self.right = true,
            }
        }

        fn cease(&mut self, way: Way) {
            match way {
                Way::Up    => self.up    = false,
                Way::Down  => self.down  = false,
                Way::Left  => self.left  = false,
                Way::Right => self.right = false,
            }
        }
    }

    pub fn move_avatar(position: &mut Position, events: Vec<Event>) {
        // let poll_iter2 = poll_iter.peekable();
        // println!("{:?}", poll_iter.peek());
        let mut want = Want::new();
        for event in events {
            match event {
                Event::KeyDown { keycode, .. } => match keycode {
                    Some(Keycode::W) => want.request(Way::Up),
                    Some(Keycode::S) => want.request(Way::Down),
                    Some(Keycode::A) => want.request(Way::Left),
                    Some(Keycode::D) => want.request(Way::Right),
                    _ => {}
                },
                Event::KeyUp { keycode, .. } => match keycode {
                    Some(Keycode::W) => want.cease(Way::Up),
                    Some(Keycode::S) => want.cease(Way::Down),
                    Some(Keycode::A) => want.cease(Way::Left),
                    Some(Keycode::D) => want.cease(Way::Right),
                    _ => {}
                },
                _ => {}
            }
        }

        if want.up {position.change(Way::Up)}
        if want.down {position.change(Way::Down)}
        if want.left {position.change(Way::Left)}
        if want.right {position.change(Way::Right)}

        // if want.up && want.down {}
        // else if want.up && position.y < 0 {}
        // else if want.down && position.y > MAX_Y as i32 {}
        // else if want.up {position.change(Way::Up)}
        // else if want.down {position.change(Way::Down)}

        // if want.left && want.right {}
        // else if want.left && position.y < 0 {}
        // else if want.right && position.y > MAX_X as i32 {}
        // else if want.left {position.change(Way::Left)}
        // else if want.right {position.change(Way::Right)}
    }
}
