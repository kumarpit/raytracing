use crate::{
    color::Color,
    common::math::{clamp, random},
    hittable::HitRecord,
    ray::Ray,
    vec3::Vec3,
};

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
// includes a "fuzzy" parameter that achieves a fuzzy appearance by randomly altering the endpoint of the
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

// ============================================
//
// Dielectric Material
//
// Materials that refract!
//
// ============================================
//

pub struct Dielectric {
    /// This is really the "context-aware" refractive index of the object. Meaning that it should
    /// used as the ratio of the refractive index of the object divided by the refractive index of
    /// the enclosing medium. In most cases the enclosing medium is air (i.e refractive index of
    /// 1.0), but if you have embedded objects, you need to be careful to divide by the
    ///   appropriate refractive index.
    refractive_index: f64,
}

impl Dielectric {
    pub fn new(refractive_index: f64) -> Self {
        Dielectric { refractive_index }
    }

    fn check_can_refract(cos_theta: f64, etai: f64, etat: f64) -> bool {
        let etai_over_etat = etai / etat;
        let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
        etai_over_etat * sin_theta <= 1.0
    }

    fn reflectance(&self, cos_theta: f64) -> f64 {
        // Using Schlick's approximation for reflectance
        let r0 = ((1.0 - self.refractive_index) / (1.0 + self.refractive_index)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
    }
}

impl Material for Dielectric {
    fn scatter(
        &self,
        ray: &Ray,
        rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &mut Ray,
    ) -> bool {
        *attenuation = Color::from(1.0);

        let (etai, etat) = if rec.did_hit_front_frace {
            // The ray is going from the environment _into_ this object
            (1.0, self.refractive_index)
        } else {
            // The ray is emerging from _within_ the object into the environment
            (self.refractive_index, 1.0)
        };

        let cos_theta = -ray.direction().into_unit().dot(rec.normal).min(1.0);
        let direction = if Dielectric::check_can_refract(cos_theta, etai, etat)
            && self.reflectance(cos_theta) <= random()
        {
            ray.direction()
                .into_unit()
                .refract(rec.normal.into_unit(), etai, etat)
        } else {
            ray.direction().into_unit().reflect(rec.normal)
        };

        *scattered = Ray::new(rec.point, direction);
        true
    }
}
