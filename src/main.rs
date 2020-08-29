mod ray;
mod vec3;

fn ray_color(r: &ray::Ray) -> vec3::Color {
    let unit_direction = r.getDirection().unit();
    let t = 0.5 * (unit_direction.y() + 1.0);
    vec3::Vec3::new(0.0, 0.0, 0.0)
}

fn main() {
    let width = 256;
    let height = 256;

    print!("P3\n{} {}\n255\n", width, height);

    for i in 0..width {
        eprintln!("Scanline {}", i);

        for j in 0..height {
            let r = i;
            let b = 64;
            let g = j;

            print!("{} {} {}\n", r, g, b);
        }
    }
}
