use std::fs;

use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CameraConfig {
    pub aspect_ratio: Vec<f64>,
    pub image_width: i32,
    pub lookat: Vec<f64>, // TODO: add length checks, or possibly use a tuple instead
    pub lookfrom: Vec<f64>,
    pub vup: Vec<f64>,
    pub samples_per_pixel: i32,
    pub max_ray_bounces: i32,
    pub vertical_field_of_view: i32,
}

#[derive(Debug, Deserialize)]
pub struct OutConfig {
    pub file: String,
}

#[derive(Debug, Deserialize)]
pub struct Config {
    pub camera: Option<CameraConfig>,
    pub out: Option<OutConfig>,
}

impl Config {
    pub fn new() -> Self {
        let config_path = "./raytracer.config.toml";
        let content = fs::read_to_string(config_path).unwrap();
        toml::from_str(&content).unwrap()
    }
}
