use rand::{rngs::SmallRng, SeedableRng, Rng};

pub struct MySmallRng {
    rng: SmallRng
}

pub trait MyRng {
    fn random_f32(&mut self) -> f32;

    fn random_f32_from_range(&mut self, min: f32, max: f32) -> f32;
}

impl MyRng for MySmallRng {
    fn random_f32(&mut self) -> f32{
        self.rng.gen::<f32>()
    }

    fn random_f32_from_range(&mut self, min: f32, max: f32) -> f32 {
        self.random_f32() * (max - min) + min
    }
}

impl MySmallRng {
    pub fn new() -> Self {
        Self {
            rng: SmallRng::from_entropy()
        }
    }
}
