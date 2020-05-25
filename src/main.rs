mod lib;
use lib::{
    camera::*, color::*, hittable::*, hittable_list::*, material::Material, ray::*, rt_math::*,
    sphere::*, vec3::*,
};

use rand::prelude::*;
use rayon::prelude::*;
use std::time;

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
        Point3::new(0.0, -1000.0, -1.0),
        1000.0,
        Material::lambertian(&Color::new(0.5, 0.5, 0.5)),
    ));

    world.add(Sphere::new_hittable(
        Point3::new(-4.0, 1.0, 0.0),
        1.0,
        Material::lambertian(&Color::new(0.4, 0.2, 0.1)),
    ));
    world.add(Sphere::new_hittable(
        Point3::new(4.0, 1.0, 0.0),
        1.0,
        Material::metal(&Color::new(0.7, 0.6, 0.5), 0.0),
    ));
    world.add(Sphere::new_hittable(
        Point3::new(0.0, 1.0, 0.0),
        1.0,
        Material::dielectrtic(1.5),
    ));

    make_random_spheres(&mut world);

    let v_up = Vec3::new(0.0, 1.0, 0.0);
    let look_from = Point3::new(13.0, 2.0, 3.0);
    let look_at = Point3::new(0.0, 0.0, 0.0);
    let focus = 10.0;
    let aperture = 0.1;
    let v_fov = 20.0;

    let cam = Camera::new(
        &look_from,
        &look_at,
        &v_up,
        v_fov,
        aspect_ratio,
        aperture,
        focus,
        0.0,
        0.1,
    );

    //let cam = Camera::default();

    //vector of rgb tuples for each pixel
    let mut screen = vec![(0usize, 0usize, 0usize); image_width * image_height];

    screen
        .par_iter_mut()
        .enumerate()
        .for_each(|(index, pixel)| {
            let mut rng = rand::thread_rng();

            let column = index % image_width;
            let row = image_height - index / image_width;

            let mut pixel_color = Color::default();

            for _ in 0..samples_per_pixel {
                let u = (column as f64 + rng.gen::<f64>()) / image_width as f64;
                let v = (row as f64 + rng.gen::<f64>()) / image_height as f64;

                let ray = &cam.get_ray(u, v);
                pixel_color = pixel_color + ray_color(ray, &world, max_depth);
            }

            pixel_color = &pixel_color / (samples_per_pixel as f64);
            //sqrt to correct gamma (gamma = 4.0)
            pixel_color = Color::new(
                pixel_color.r().sqrt(),
                pixel_color.g().sqrt(),
                pixel_color.b().sqrt(),
            );

            //Write the tranlated [0,255] value of each color component
            let r_temp = (255.99 * pixel_color.r()) as usize;
            let g_temp = (255.99 * pixel_color.g()) as usize;
            let b_temp = (255.99 * pixel_color.b()) as usize;

            *pixel = (r_temp, g_temp, b_temp);
        });

    for (r, g, b) in screen {
        println!("{} {} {}", r, g, b);
    }

    eprintln!("\nDone\n");
}

fn make_random_spheres(world: &mut lib::hittable_list::HittableList) {
    let mut rng = rand::thread_rng();
    let mut center: Vec3;
    let mut albedo: Color;
    let mut fuzz: f64;
    let radius = 0.2;

    for a in -11..11 {
        let choose_mat = rng.gen::<f64>();
        for b in -11..11 {
            center = Point3::new(a as f64, 0.2, b as f64 + 0.9 * rng.gen::<f64>());
            if (center - Vec3::new(4.0, 0.2, 0.0)).length() > 0.9 {
                if choose_mat < 0.7 {
                    //diffuse
                    albedo = &Color::random() * &Color::random();
                    let center_end = center + Vec3::new(0.0, rng.gen::<f64>() * 0.5 + 1.0, 0.0);
                    world.add(MoveableSphere::new_hittable(
                        center,
                        center_end,
                        radius,
                        Material::lambertian(&albedo),
                        0.0,
                        1.0,
                    ));
                } else if choose_mat < 0.90 {
                    albedo = Color::new(rng.gen::<f64>(), 0.5, 1.0);
                    fuzz = rng.gen::<f64>() * 0.5;
                    world.add(Sphere::new_hittable(
                        center,
                        radius,
                        Material::metal(&albedo, fuzz),
                    ));
                } else {
                    world.add(Sphere::new_hittable(
                        center,
                        radius,
                        Material::dielectrtic(1.5),
                    ))
                }
            }
        }
    }
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: usize = 1080;
    const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;
    const SAMPLE_PER_PIXEL: usize = 100;
    const MAX_DEPTH: u16 = 50;

    let start_time = time::Instant::now();

    render(ASPECT_RATIO, IMAGE_HEIGHT, SAMPLE_PER_PIXEL, MAX_DEPTH);

    let duration = time::Instant::now() - start_time;
    eprintln!(
        "Rendered image with dimensions:\n {} x {}\n in {:?}",
        IMAGE_WIDTH, IMAGE_HEIGHT, duration
    );
}
