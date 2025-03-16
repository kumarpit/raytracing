use crate::{color::Color, common::math::random, hittable::HitRecord, ray::Ray, vec3::Vec3};

pub enum Material {
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
    Lambertian {
        albedo: Color,
    },

    // ============================================
    //
    // Metal Material
    //
    // Shiny, shiny metals! Scattered rays are perfectly reflected about the surface normal. Also
    // includes a "fuzzy" parameter that achieves a fuzzy appearance by randomly altering the endpoint of the
    // reflected ray. The length of this alteration is determined by the fuzz factor.
    //
    // ============================================
    Metal {
        albedo: Color,
        fuzz: f64,
    },

    // ============================================
    //
    // Dielectric Material
    //
    // Materials that refract!
    //
    // ============================================
    Dielectric {
        // This is really the "context-aware" refractive index of the object. Meaning that it should
        /// used as the ratio of the refractive index of the object divided by the refractive index of
        /// the enclosing medium. In most cases the enclosing medium is air (i.e refractive index of
        /// 1.0), but if you have embedded objects, you need to be careful to divide by the
        ///   appropriate refractive index.
        refractive_index: f64,
    },
}

/// Represents the reflected/refracted ray properties from a material interaction
pub struct ScatterRecord {
    pub attenuation: Color,
    pub scattered: Ray,
}

impl Material {
    pub fn scatter(&self, ray: &Ray, rec: &HitRecord) -> Option<ScatterRecord> {
        match self {
            Material::Lambertian { albedo } => {
                let mut scatter_direction = rec.normal + Vec3::in_unit_sphere().into_unit();
                if scatter_direction.is_near_zero() {
                    scatter_direction = rec.normal;
                }
                let attenuation = albedo;
                let scattered = Ray::new(rec.point, scatter_direction);
                Some(ScatterRecord {
                    attenuation: *attenuation,
                    scattered,
                })
            }
            Material::Metal { albedo, fuzz } => {
                let reflected = ray.direction().into_unit().reflect(rec.normal).into_unit()
                    + *fuzz * Vec3::on_unit_sphere();
                let attenuation = albedo;
                let scattered = Ray::new(rec.point, reflected);
                Some(ScatterRecord {
                    attenuation: *attenuation,
                    scattered,
                })
                .filter(|_| scattered.direction().dot(rec.normal) > 0.0)
            }
            Material::Dielectric { refractive_index } => {
                let attenuation = Color::from(1.0);

                let (etai, etat) = if rec.did_hit_front_frace {
                    // The ray is going from the environment _into_ this object
                    (1.0, *refractive_index)
                } else {
                    // The ray is emerging from _within_ the object into the environment
                    (*refractive_index, 1.0)
                };

                let cos_theta = -ray.direction().into_unit().dot(rec.normal).min(1.0);
                let etai_over_etat = etai / etat;
                let sin_theta = (1.0 - cos_theta.powi(2)).sqrt();
                let can_refract = etai_over_etat * sin_theta <= 1.0;
                let direction = if can_refract && schlick(*refractive_index, cos_theta) <= random()
                {
                    ray.direction()
                        .into_unit()
                        .refract(rec.normal.into_unit(), etai, etat)
                } else {
                    ray.direction().into_unit().reflect(rec.normal)
                };

                let scattered = Ray::new(rec.point, direction);
                Some(ScatterRecord {
                    attenuation,
                    scattered,
                })
            }
        }
    }
}

#[inline]
fn schlick(refractive_index: f64, cos_theta: f64) -> f64 {
    // Using Schlick's approximation for reflectance
    let r0 = ((1.0 - refractive_index) / (1.0 + refractive_index)).powi(2);
    r0 + (1.0 - r0) * (1.0 - cos_theta).powi(5)
}
