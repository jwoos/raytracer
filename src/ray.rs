struct Ray {
    origin: vec3::Point,
    direction: vec3::Vec3,
}

/* P(t) = A + tb
 * P is a 3D position along a line
 * A is the origin
 * b is the ray direction
 * t is a real number - the ray parameter
 */
impl Ray {
    fn new(origin: vec3::Point, direction: vec3::Vec3) -> Ray {
        Ray {origin, direction}
    }

    // TODO make operations commutative and move t in front
    fn at(&self, f64 t) -> {
        self.origin + (self.direction * t)
    }
}
