use ray::Ray;
use vec3::{Color, Point, Vec3};

struct HitRecord {
	point: Point,
	normal: Vec3,
	t: f64,
}

pub trait Hittable {
	fn hit(&self, r: &Ray, f64 t_min) -> (bool, Option<HitRecord>);
}
