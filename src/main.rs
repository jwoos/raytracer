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

fn ray_color(r: &Ray, world: &HittableList, depth: i16) -> Color {
    if depth <= 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    let (hit, record_opt) = world.hit(r, (0.001, std::f64::INFINITY));
    if hit {
        let record = record_opt.unwrap();
        let target = record.point + Vec3::random_in_hemisphere(&record.normal);

        let (scattered, attenuation, scattered_ray) = record.material.scatter(r, &record);
        if scattered {
            return attenuation * ray_color(&scattered_ray, world, depth - 1);
        } else {
            return Color::new(0.0, 0.0, 0.0);
        }
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
    let samples_per_pixel = 100;
    let max_bounces = 50;

    // world
    /*
     *    let R = (std::f64::consts::PI / 4.0).cos();
     *    let mut world = HittableList::new();
     *
     *    let material_left = Rc::new(objects::Lambertian::new(Color::new(0.0, 0.0, 1.0)));
     *    let material_right = Rc::new(objects::Lambertian::new(Color::new(1.0, 0.0, 0.0)));
     *
     *    world.add(Rc::new(Sphere::new(
     *        Point::new(-R, 0.0, -1.0),
     *        R,
     *        material_left,
     *    )));
     *    world.add(Rc::new(Sphere::new(
     *        Point::new(R, 0.0, -1.0),
     *        R,
     *        material_right,
     *    )));
     */

    let material_ground = Rc::new(objects::Lambertian::new(Color::new(0.8, 0.8, 0.0)));
    let material_center = Rc::new(objects::Lambertian::new(Color::new(0.1, 0.2, 0.5)));
    let material_left = Rc::new(objects::Dielectric::new(1.5));
    let material_right = Rc::new(objects::Metal::new(Color::new(0.8, 0.6, 0.2), 0.0));

    let mut world = HittableList::new();
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, -100.5, -1.0),
        100.0,
        material_ground.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(0.0, 0.0, -1.0),
        0.5,
        material_center.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        0.5,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(-1.0, 0.0, -1.0),
        -0.45,
        material_left.clone(),
    )));
    world.add(Rc::new(Sphere::new(
        Point::new(1.0, 0.0, -1.0),
        0.5,
        material_right.clone(),
    )));

    // camera
    let look_from = Point::new(3.0, 3.0, 2.0);
    let look_at = Point::new(0.0, 0.0, -1.0);
    let distance_to_focus = (look_from - look_at).length();
    let camera = Camera::new(
        look_from,
        look_at,
        Vec3::new(0.0, 1.0, 0.0),
        20.0,
        aspect_ratio,
        2.0,
        distance_to_focus,
    );

    print!("P3\n{} {}\n255\n", image_width, image_height);

    for j in (0..image_height).rev() {
        eprintln!("Scanline {}", j);

        for i in 0..image_width {
            let mut pixel_color = Color::new(0.0, 0.0, 0.0);

            for s in 0..samples_per_pixel {
                let u = (i as f64 + rand::random::<f64>()) / (image_width - 1) as f64;
                let v = (j as f64 + rand::random::<f64>()) / (image_height - 1) as f64;
                let ray = camera.get_ray(u, v);
                pixel_color += ray_color(&ray, &world, max_bounces);
            }

            let scale = 1.0 / samples_per_pixel as f64;

            // divide by scale and gamma correct
            let r = (pixel_color.x() * scale).sqrt();
            let g = (pixel_color.y() * scale).sqrt();
            let b = (pixel_color.z() * scale).sqrt();

            print!(
                "{} {} {}\n",
                (256.0 * utility::clamp(r, 0.0, 0.999)) as i64,
                (256.0 * utility::clamp(g, 0.0, 0.999)) as i64,
                (256.0 * utility::clamp(b, 0.0, 0.999)) as i64
            );
        }
    }
}
