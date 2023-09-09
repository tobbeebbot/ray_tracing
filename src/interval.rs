pub struct Interval {
    min: f32,
    max: f32,
}

impl Interval {
    pub fn new(min: f32, max: f32) -> Interval {
        Interval { min, max }
    }

    pub fn surrounds(&self, x:f32) -> bool {
        self.min < x && x < self.max
    }
    
    pub fn contains(&self, x:f32) -> bool {
        self.min <= x && x <= self.max
    }

    pub fn surround_where(&self, x:f32) -> Option<f32> {
        if self.surrounds(x) {
            Some(x)
        } else {
            None
        }
    }

    pub fn contains_where(&self, x:f32) -> Option<f32> {
        if self.contains(x) {
            Some(x)
        } else {
            None
        }
    }

    pub fn clamp(&self, x: f32) -> f32 {
        if x < self.min { self.min }
        else if x > self.max {self.max }
        else { x }
    }
}