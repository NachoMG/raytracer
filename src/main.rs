use color::write_color;
use vec3::Vector3;

mod color;
mod vec3;

fn main() {
    let image_width = 256;
    let image_height = 256;

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    for row in 0..image_height {
        eprint!("\rScanlines remaining: {}", image_height - row - 1);

        for col in 0..image_width {
            let pixel_color = Vector3::new(
                (col as f64) / ((image_width - 1) as f64),
                (row as f64) / ((image_height - 1) as f64),
                0.0,
            );
            write_color(pixel_color)
        }
    }

    eprintln!("\nDone.");
}
