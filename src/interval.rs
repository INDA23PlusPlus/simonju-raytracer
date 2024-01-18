pub const EMPTY: Interval = Interval::new(f32::INFINITY, f32::NEG_INFINITY);
pub const UNIVERSE: Interval = Interval::new(f32::NEG_INFINITY, f32::INFINITY);

pub struct Interval {
    pub min: f32,
    pub max: f32,
}

impl Interval {
    pub const fn new(min: f32, max: f32) -> Self {
        Self { min, max }
    }

    pub fn contains(&self, x: f32) -> bool {
        x >= self.min && x <= self.max
    }

    pub fn surrounds(&self, x: f32) -> bool {
        (x < self.min && x < self.max) || (x > self.min && x > self.max) 
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min {
            self.min
        } else if x > self.max {
            self.max
        } else {
            x
        }
    }
}