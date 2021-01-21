use rand;

pub fn clamp(x: f64, min: f64, max: f64) -> f64 {
    if x < min {
        return min;
    }

    if x > max {
        return max;
    }

    return x;
}

pub fn random(min: f64, max: f64) -> f64 {
    min + (max - min) * rand::random::<f64>()
}

pub fn degrees_to_radians(degrees: f64) -> f64 {
    degrees * std::f64::consts::PI / 180.0
}
