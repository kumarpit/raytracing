use std::io::Write;

use crate::vec3::Vec3;

pub type Color = Vec3;

pub fn linear_to_gamma(linear_component: f64) -> f64 {
    if linear_component > 0.0 {
        linear_component.sqrt()
    } else {
        0.0
    }
}

pub fn write_color(out: &mut impl Write, pixel_color: Color) {
    let gamma_space_pixel_color = pixel_color.map(linear_to_gamma);
    // Write the translated [0, 255] value of each color component
    let r = (255.999 * gamma_space_pixel_color.0) as i32;
    let g = (255.999 * gamma_space_pixel_color.1) as i32;
    let b = (255.999 * gamma_space_pixel_color.2) as i32;
    writeln!(out, "{} {} {}", r, g, b).expect("writing color");
}
