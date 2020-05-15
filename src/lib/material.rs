use crate::lib::{color::*, hittable::HitRecord, ray::Ray, vec3::*};

#[derive(Copy, Clone)]
pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
    Dielectric { albedo: Color },
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

    pub fn dielectrtic(albedo: &Color) -> Self {
        Material::Dielectric { albedo: *albedo }
    }

    pub fn scatter<'a>(
        material: &Material,
        ray_in: &Ray,
        hit_rec: &HitRecord,
        attenuation: &mut Color,
        scattered: &'a mut Ray,
    ) -> bool {
        match material {
            Material::Lambertian { albedo } => {
                let scatter_direction = hit_rec.normal() + Vec3::random_unit_vector_lambertian();
                *scattered = Ray::new(&hit_rec.point(), &scatter_direction);
                *attenuation = *albedo;
                return true;
            }
            Material::Metal { albedo, fuzz } => {
                if *fuzz > 1.0 {
                    let reflected = &ray_in.direction().unit_vector().reflect(&hit_rec.normal());
                    *scattered = Ray::new(
                        &hit_rec.point(),
                        &(*reflected + Vec3::random_unit_vector_lambertian()),
                    );
                    *attenuation = *albedo;
                    return scattered.direction().dot(&hit_rec.normal()) > 0.0;
                } else {
                    let reflected = &ray_in.direction().unit_vector().reflect(&hit_rec.normal());
                    *scattered = Ray::new(
                        &hit_rec.point(),
                        &(*reflected + *fuzz * Vec3::random_in_unit_sphere()),
                    );
                    *attenuation = *albedo;
                    return scattered.direction().dot(&hit_rec.normal()) > 0.0;
                }
            }
            Material::Dielectric { albedo } => {}
        }
        false
    }
}
