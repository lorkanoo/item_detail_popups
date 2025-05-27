use serde::{Deserialize, Serialize};
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Dimensions {
    pub width: f32,
    pub height: f32,
}

impl Dimensions {
    pub fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn small() -> Self {
        Self {
            width: 20.0,
            height: 20.0,
        }
    }

    pub fn medium() -> Self {
        Self {
            width: 40.0,
            height: 40.0,
        }
    }

    pub fn tuple(&self) -> (f32, f32) {
        (self.width, self.height)
    }
}
