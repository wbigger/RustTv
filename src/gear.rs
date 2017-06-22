use std::f64::consts::PI;

pub struct Gear {
    pub pitch_circle_radius: f64,
    pub addendum: f64,
    pub teeth_num: i32
}

impl Gear {
    pub fn addendum_circle_radius(&self) -> f64 {
        self.pitch_circle_radius + self.addendum
    }

    pub fn pitch(&self) -> f64 {
        2.0 * PI * self.pitch_circle_radius / (self.teeth_num as f64)
    }
}

pub struct Rack {
    pub width: f64,
    pub height: f64,
    pub addendum: f64,
    pub teeth_num: i32
}

impl Rack {
    pub fn pitch(&self) -> f64 {
        self.width / (self.teeth_num as f64)
    }
}