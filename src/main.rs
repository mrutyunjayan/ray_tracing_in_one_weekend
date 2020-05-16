mod lib;
use lib::{
    camera::*, color::*, hittable::*, hittable_list::*, material::Material, ray::*, rt_math::*,
    sphere::*, vec3::*,
};

use rand::prelude::*;
//use std::f64::consts::*;

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: u16) -> Color {
    let mut hit_rec = HitRecord::new_invalid();

    //if we've exceeded the ray bounce limit, no more light is gathered
    if depth == 0 {
        return Color::new(0.0, 0.0, 0.0);
    }

    if world.hit(ray, 0.001, INFINITY as f64, &mut hit_rec) {
        let mut scattered = Ray::default();
        let mut attenuation = Color::default();

        if Material::scatter(
            &hit_rec.material(),
            ray,
            &hit_rec,
            &mut attenuation,
            &mut scattered,
        ) {
            return &attenuation * &ray_color(&scattered, world, depth - 1); //not real vector multiplication - just scaling by the attenuation values
        }
        return Color::new(0.0, 0.0, 0.0);
    }

    let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0); //blue to white blend
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    //linear blend
    // blendedValue = (1 − t)⋅startValue + t⋅endValue
    (1.0 - t) * &start_value + t * &end_value
}

fn render(aspect_ratio: f64, image_height: usize, samples_per_pixel: usize, max_depth: u16) {
    let image_width = (image_height as f64 * aspect_ratio) as usize;

    println!("P3\n{} {} \n255\n", image_width, image_height);

    //let r: f64 = (PI / 4.0).cos();
    let mut world: HittableList = HittableList::new();

    world.add(Sphere::new_hittable(
        Point3::new(0.0, 0.0, -1.0),
        0.5,
        Material::lambertian(&Color::new(0.1, 0.2, 0.5)),
    ));
    world.add(Sphere::new_hittable(
        Point3::new(0.0, -100.5, -1.0),
        100.0,
        Material::lambertian(&Color::new(0.8, 0.8, 0.0)),
    ));
    world.add(Sphere::new_hittable(
        Point3::new(1.0, 0.0, -1.0),
        0.5,
        Material::metal(&Color::new(0.8, 0.6, 0.2), 0.0),
    ));
    world.add(Sphere::new_hittable(
        Point3::new(-1.0, 0.0, -1.0),
        0.5,
        Material::dielectrtic(1.5),
    ));
    world.add(Sphere::new_hittable(
        Point3::new(-1.0, 0.0, -1.0),
        -0.45,
        Material::dielectrtic(1.5),
    ));

    let v_up = Vec3::new(0.0, 1.0, 0.0);

    let cam = Camera::new(
        &Point3::new(-2.0, 2.0, 1.0),
        &Point3::new(0.0, 0.0, -1.0),
        &v_up,
        90.0,
        aspect_ratio,
    );

    //let cam = Camera::default();

    let mut rng = rand::thread_rng();

    //vertical lines
    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);

        //horizontal lines
        for i in 0..image_width {
            let mut pixel_color = Color::default();

            //sampling each pixel multiple times for anti-aliasing
            for _ in 0..samples_per_pixel {
                let u = (i as f64 + rng.gen::<f64>()) / image_width as f64;
                let v = (j as f64 + rng.gen::<f64>()) / image_height as f64;

                let ray = &cam.get_ray(u, v); //Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
                pixel_color = pixel_color + ray_color(ray, &world, max_depth);
            }

            pixel_color = &pixel_color / (samples_per_pixel as f64);
            //sqrt to correct gamma (gamma = 2.0)
            pixel_color = Color::new(
                pixel_color.r().sqrt(),
                pixel_color.g().sqrt(),
                pixel_color.b().sqrt(),
            );
            Color::write_color(&pixel_color);
        }
    }
    eprintln!("\nDone\n");
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: usize = 360;
    const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;
    const SAMPLE_PER_PIXEL: usize = 100;
    const MAX_DEPTH: u16 = 50;

    render(ASPECT_RATIO, IMAGE_HEIGHT, SAMPLE_PER_PIXEL, MAX_DEPTH);
    eprintln!(
        "Rendered image with dimensions:\n {} x {}",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
}
