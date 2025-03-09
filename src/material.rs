use crate::{color::Color, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub trait Material {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool;
}

// ============================================
//
// Lambertian Material
//
// The Lambertian material models "diffuse" objects - such objects have a matte appearance. This is
// achieved by having the scattered rays follow the Lambertian distribution wherein the reflected
// rays scatter in a direction near the surface normal. Another (more simplistic) approach to
// achieving a diffuse object is to have the refelcted rays randomly scatter in the hemisphere
// containing the surface normal - though these give less realistic results.
//
// ============================================

pub struct Lambertian {
    albedo: Color,
}

impl Lambertian {
    pub fn new(albedo: Color) -> Self {
        Lambertian { albedo }
    }
}

impl Material for Lambertian {
    fn scatter(
        &self,
        _: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let mut scatter_direction = rec.normal + Vec3::in_unit_sphere().into_unit();
        if scatter_direction.is_near_zero() {
            scatter_direction = rec.normal;
        }
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.point, scatter_direction);
        true
    }
}

// ============================================
//
// Metal Material
//
// Shiny, shiny metals! Scattered rays are perfectly reflected about the surface normal.
//
// ============================================

pub struct Metal {
    albedo: Color,
}

impl Metal {
    pub fn new(albedo: Color) -> Self {
        Metal { albedo }
    }
}

impl Material for Metal {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        let reflected = ray.direction().into_unit().reflect(rec.normal);
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.point, reflected);
        true
    }
}
