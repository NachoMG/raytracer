use crate::vec3::Vector3;

pub fn write_color(pixel_color: Vector3) {
    let ir = (255.999 * pixel_color[0]) as u64;
    let ig = (255.999 * pixel_color[1]) as u64;
    let ib = (255.999 * pixel_color[2]) as u64;

    println!("{} {} {}", ir, ig, ib);
}
