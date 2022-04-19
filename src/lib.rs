
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

    pub fn to_veci2d(&self) -> Veci2d {
        Veci2d::new(self.x.round() as i32, self.y.round() as i32)
    }
}

#[derive(Clone, Copy)]
pub struct Veci2d {
    pub x: i32,
    pub y: i32,
}

impl Veci2d {
    pub fn new(x: i32, y: i32) -> Veci2d {
        Veci2d { x, y }
    }

    pub fn zero() -> Veci2d {
        Veci2d::new(0, 0)
    }
}

pub struct Player {
    pos: Vecf2d,
    dir: f32, // rads
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

    pub fn update_dir(&mut self, step: f32) {
        self.dir += step * 0.55;
    }

    pub fn move_forward(&mut self, step: f32) {
        self.pos.y += self.dir().cos() * step;
        self.pos.x += self.dir().sin() * step;
    }

    pub fn move_backward(&mut self, step: f32) {
        self.pos.y -= self.dir().cos() * step;
        self.pos.x -= self.dir().sin() * step;
    }
}

// #[cfg(test)]
// mod tests {
// }
