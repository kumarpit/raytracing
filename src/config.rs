use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub aspect_ratio: Vec<f64>,
    pub image_width: i32,
    pub viewport_height: f64,
    pub focal_length: f64,
    pub samples_per_pixel: i32,
    pub max_ray_bounces: i32,
    pub vertical_field_of_view: i32,
}

impl CameraConfig {
    pub fn new() -> Self {
        let config_path = "./raytracer.config.toml";
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
