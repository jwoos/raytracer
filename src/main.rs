mod camera;
mod objects;
mod ray;
mod sphere;
mod utility;
mod vec3;

use std::rc::Rc;
use vec3::{Color, Point, Vec3};

use rand;

use camera::Camera;
use objects::{HitRecord, HittableList};
use ray::Ray;
use sphere::Sphere;

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
    let samples_per_pixel = 1000;

    // world
    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(Point::new(0.0, 0.0, -1.0), 0.5)));
    world.add(Rc::new(Sphere::new(Point::new(0.0, -100.5, -1.0), 100.0)));

    // camera
    let camera = Camera::new();

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanline {}", j);

        for i in (0..image_width).rev() {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for s in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world);
            }

            let scale = 1.0 / samples_per_pixel as f64;
            let r = pixel_color.x() * scale;
            let g = pixel_color.y() * scale;
            let b = pixel_color.z() * scale;

            print!(
                "{} {} {}\n",
                (256.0 * utility::clamp(r, 0.0, 0.999)) as i64,
                (256.0 * utility::clamp(g, 0.0, 0.999)) as i64,
                (256.0 * utility::clamp(b, 0.0, 0.999)) as i64
            );
        }
    }
}
