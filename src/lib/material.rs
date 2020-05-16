use crate::lib::{color::*, hittable::HitRecord, ray::Ray, vec3::*};

use partial_min_max::min;
use rand::prelude::*;

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { refractive_index: f64 },
}

impl Default for Material {
    fn default() -> Self {
        Material::Lambertian {
            albedo: Color::default(),
        }
    }
}

#[allow(dead_code)]
impl Material {
    pub fn lambertian(albedo: &Color) -> Self {
        Material::Lambertian { albedo: *albedo }
    }

    pub fn metal(albedo: &Color, fuzz: f64) -> Self {
        Material::Metal {
            albedo: *albedo,
            fuzz,
        }
    }

    pub fn dielectrtic(refractive_index: f64) -> Self {
        Material::Dielectric { refractive_index }
    }

    pub fn schlick(cosine: f64, ref_idx: f64) -> f64 {
        let mut r0 = (1.0 - ref_idx) / (1.0 + ref_idx);
        r0 *= r0;
        r0 + (1.0 - r0) * (1.0 - cosine).powi(5)
    }

    pub fn scatter<'a>(
        material: &Material,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &'a mut Ray,
    ) -> bool {
        match material {
            //diffuse
            Material::Lambertian { albedo } => {
                let scatter_direction = hit_rec.normal() + Vec3::random_unit_vector_lambertian();
                *scattered = Ray::new(&hit_rec.point(), &scatter_direction);
                *attenuation = *albedo;
                true
            }

            //specular
            Material::Metal { albedo, fuzz } => {
                if *fuzz > 1.0 {
                    let reflected = &ray_in.direction().unit_vector().reflect(&hit_rec.normal());
                    *scattered = Ray::new(
                        &hit_rec.point(),
                        &(*reflected + Vec3::random_in_unit_sphere()),
                    );
                    *attenuation = *albedo;
                    scattered.direction().dot(&hit_rec.normal()) > 0.0
                } else {
                    let reflected = &ray_in.direction().unit_vector().reflect(&hit_rec.normal());
                    *scattered = Ray::new(
                        &hit_rec.point(),
                        &(*reflected + (*fuzz * Vec3::random_in_unit_sphere())),
                    );
                    *attenuation = *albedo;
                    scattered.direction().dot(&hit_rec.normal()) > 0.0
                }
            }

            //glass-like
            Material::Dielectric { refractive_index } => {
                //glass absorbs nothing
                *attenuation = Color::new(1.0, 1.0, 1.0);

                let eta_over_etaprime = if hit_rec.front_face() {
                    1.0 / *refractive_index
                } else {
                    *refractive_index
                };

                let ray_in_unit_direction = ray_in.direction().unit_vector();

                let cos_theta: f64 = min(-ray_in_unit_direction.dot(&hit_rec.normal()), 1.0);
                let sin_theta: f64 = (1.0 - cos_theta * cos_theta).sqrt();

                //Must reflect if eta_over_etaprime * sin_theta > 1.0 - Total Internal Reflection
                if eta_over_etaprime * sin_theta > 1.0 {
                    let reflected = ray_in_unit_direction.reflect(&hit_rec.normal());
                    *scattered = Ray::new(&hit_rec.point(), &reflected);

                    true
                } else {
                    //refract the ray

                    let mut rng = rand::thread_rng();

                    //approximate varying reflectivity with angle
                    let reflect_probability = Material::schlick(cos_theta, *refractive_index);
                    if rng.gen::<f64>() < reflect_probability {
                        let reflected = ray_in_unit_direction.reflect(&hit_rec.normal());
                        *scattered = Ray::new(&hit_rec.point(), &reflected);

                        true
                    } else {
                        let refracted =
                            ray_in_unit_direction.refract(&hit_rec.normal(), eta_over_etaprime);
                        *scattered = Ray::new(&hit_rec.point(), &refracted);

                        true
                    }
                }
            }
        }
    }
}
