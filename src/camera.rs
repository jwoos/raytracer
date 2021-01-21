use crate::ray::Ray;
use crate::utility;
use crate::vec3::{Point, Vec3};

pub struct Camera {
    origin: Point,
    lower_left_corner: Point,
    horizontal: Vec3,
    vertical: Vec3,
}

impl Camera {
    pub fn new(from: Point, at: Point, vup: Vec3, vfov: f64, aspect_ratio: f64) -> Camera {
        let theta = utility::degrees_to_radians(vfov);
        let h = (theta / 2.0).tan();
        let viewport_height = 2.0 * h;
        let viewport_width = viewport_height * aspect_ratio;

        let w = (from - at).unit();
        let u = Vec3::cross_product(&vup, &w).unit();
        let v = Vec3::cross_product(&w, &u);

        let origin = from;
        let horizontal = viewport_width * u;
        let vertical = viewport_height * v;
        let lower_left_corner = origin - (horizontal / 2.0) - (vertical / 2.0) - w;

        Camera {
            origin,
            lower_left_corner,
            horizontal,
            vertical,
        }
    }

    pub fn get_ray(&self, s: f64, t: f64) -> Ray {
        Ray::new(
            self.origin,
            self.lower_left_corner + s * self.horizontal + t * self.vertical - self.origin,
        )
    }
}
