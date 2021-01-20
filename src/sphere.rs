use crate::objects::{HitRecord, Hittable};
use crate::ray::Ray;
use crate::vec3::{Color, Point, Vec3};

pub struct Sphere {
    center: Point,
    radius: f64,
}

impl Sphere {
    pub fn new(center: Point, radius: f64) -> Sphere {
        Sphere { center, radius }
    }
}

impl Hittable for Sphere {
    fn hit(&self, r: &Ray, t: (f64, f64)) -> (bool, Option<HitRecord>) {
        let oc = *r.get_origin() - self.center;

        let a = r.get_direction().length_squared();
        let half_b = Vec3::dot(&oc, r.get_direction());
        let c = oc.length_squared() - self.radius * self.radius;

        let discriminant = half_b * half_b - a * c;
        if discriminant < 0.0 {
            return (false, None);
        }

        let sqrtd = discriminant.sqrt();

        let mut root = (-half_b - sqrtd) / a;
        if root < t.0 || t.1 < root {
            root = (-half_b + sqrtd) / a;
            if root < t.0 || t.1 < root {
                return (false, None);
            }
        }

        let t = root;
        let point = r.at(t);
        let outward_normal = (point - self.center) / self.radius;

        let mut rec = HitRecord::new(point, outward_normal, t, false);
        rec.set_face_normal(r, outward_normal);

        return (true, Some(rec));
    }
}
