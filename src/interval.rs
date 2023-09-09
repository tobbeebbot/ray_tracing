pub trait Surroundable {
    fn surrounds(&self, x:f32) -> bool;
    fn surround_where(&self, x:f32) -> Option<f32>;
    fn contains_where(&self, x:f32) -> Option<f32>;
}

impl Surroundable for std::ops::Range<f32> {
    fn surrounds(&self, x:f32) -> bool {
        self.start < x && x < self.end
    }

    fn surround_where(&self, x:f32) -> Option<f32> {
        if self.surrounds(x) {
            Some(x)
        } else {
            None
        }
    }

    fn contains_where(&self, x:f32) -> Option<f32> {
        if self.contains(&x) {
            Some(x)
        } else {
            None
        }
    }
}