use crate::{color::Color, common::math::clamp, hittable::HitRecord, ray::Ray, vec3::Vec3};

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
// Shiny, shiny metals! Scattered rays are perfectly reflected about the surface normal. Also
// includes a "fuzzy" parameter that achieves a fuzzy appearance by altering the endpoint of the
// reflected ray. The length of this alteration is determined by the fuzz factor.
//
// ============================================
//

pub struct Metal {
    albedo: Color,
    fuzz: f64,
}

impl Metal {
    pub fn new(albedo: Color, fuzz: f64) -> Self {
        Metal {
            albedo,
            fuzz: clamp(0.0, 1.0, fuzz),
        }
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
        let reflected = ray.direction().into_unit().reflect(rec.normal).into_unit()
            + self.fuzz * Vec3::on_unit_sphere();
        *attenuation = self.albedo;
        *scattered = Ray::new(rec.point, reflected);
        scattered.direction().dot(rec.normal) > 0.0
    }
}
