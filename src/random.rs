pub fn random_f32() -> f32 {
    fastrand::f32()
}

pub fn random_f32_from_range(min: f32, max: f32) -> f32 {
    random_f32() * (max - min) + min
}
