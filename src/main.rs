mod objects;
mod ray;
mod sphere;
mod vec3;

use objects::{HitRecord, HittableList};
use ray::Ray;
use sphere::Sphere;
use std::rc::Rc;
use vec3::{Color, Point, Vec3};

fn ray_color(r: &Ray, world: &HittableList) -> Color {
    let (hit, record) = world.hit(r, (0.0, std::f64::INFINITY));
    if hit {
        return 0.5 * (record.unwrap().normal + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction = r.get_direction().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    (1.0 - t) * Color::new(1.0, 1.0, 1.0) + t * Color::new(0.5, 0.7, 1.0)
}

fn hit_sphere(center: &Point, radius: f64, r: &Ray) -> f64 {
    let oc = *r.get_origin() - *center;

    let a = r.get_direction().length_squared();
    let half_b = Vec3::dot(&oc, r.get_direction());
    let c = oc.length_squared() - radius * radius;
    let discriminant = half_b * half_b - a * c;

    if discriminant < 0.0 {
        return -1.0;
    } else {
        return (-half_b - discriminant.sqrt()) / a;
    }
}

fn main() {
    // image
    let aspect_ratio = 16.0 / 9.0;
    let image_width = 400;
    let image_height = (image_width as f64 / aspect_ratio) as i64;

    // world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let viewport_height = 2.0;
    let viewport_width = viewport_height * aspect_ratio;
    let focal_length = 1.0;

    let origin = Point::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner =
        origin - (horizontal / 2.0) - (vertical / 2.0) - Vec3::new(0.0, 0.0, focal_length);

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanline {}", j);

        for i in (0..image_width).rev() {
            let u = (i as f64) / (image_width - 1) as f64;
            let v = (j as f64) / (image_height - 1) as f64;
            let ray = Ray::new(
                origin,
                lower_left_corner + u * horizontal + v * vertical - origin,
            );
            let pixel_color = ray_color(&ray, &world);

            print!(
                "{} {} {}\n",
                (255.0 * pixel_color.x()) as i64,
                (255.0 * pixel_color.y()) as i64,
                (255.0 * pixel_color.z()) as i64
            );
        }
    }
}
