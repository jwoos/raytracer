use crate::vec3;

pub struct Ray {
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
	pub fn new(origin: vec3::Point, direction: vec3::Vec3) -> Ray {
		Ray { origin, direction }
	}

	pub fn get_origin(&self) -> &vec3::Point {
		&self.origin
	}

	pub fn get_direction(&self) -> &vec3::Vec3 {
		&self.direction
	}

	pub fn at(&self, t: f64) -> vec3::Point {
		self.origin + (t * self.direction)
	}
}
