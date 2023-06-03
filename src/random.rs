
pub fn random_f64() -> f64 {
    fastrand::f64()
}

pub fn random_f64_from_range(min: f64, max: f64) ->f64{
    random_f64() * (max - min) + min
}