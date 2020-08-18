mod vec3;

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
