use crate::{common::math::clamp, vec3::Vec3};
use std::io::Write;

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
    // Translate each color component to a value in the RGB range [0, 255]
    let translated_pixel_pixel_color =
        gamma_space_pixel_color.map(|x| -> f64 { 256.0 * clamp(0.0, 0.999, x) });
    writeln!(
        out,
        "{} {} {}",
        translated_pixel_pixel_color.0 as i32,
        translated_pixel_pixel_color.1 as i32,
        translated_pixel_pixel_color.2 as i32
    )
    .expect("writing color");
}
