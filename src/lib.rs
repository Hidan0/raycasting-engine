#[derive(Clone, Copy)]
pub struct Vecf2d {
    pub x: f32,
    pub y: f32,
}

impl Vecf2d {
    pub fn new(x: f32, y: f32) -> Vecf2d {
        Vecf2d { x, y }
    }

    pub fn zero() -> Vecf2d {
        Vecf2d::new(0.0, 0.0)
    }
}

pub struct Player {
    pos: Vecf2d,
    dir: f32,
}

impl Player {
    pub fn new(start_pos: Vecf2d, start_dir: f32) -> Player {
        Player {
            pos: start_pos,
            dir: start_dir,
        }
    }

    pub fn pos(&self) -> Vecf2d {
        self.pos
    }

    pub fn dir(&self) -> f32 {
        self.dir
    }

    pub fn dir_as_yx(&self) -> (f32, f32) {
        self.dir.sin_cos()
    }

    pub fn add_x(&mut self, step: f32) {
        self.pos.x += step;
    }

    pub fn add_y(&mut self, step: f32) {
        self.pos.y += step;
    }
        
    pub fn update_dir(&mut self, step: f32) {
        self.dir += step;
    }
}
