mod lib;
use lib::{
    camera::*, color::*, hittable::*, hittable_list::*, ray::*, rt_math::*, sphere::*, vec3::*,
};

use rand::prelude::*;

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_rec = HitRecord::new_invalid();

    if world.hit(ray, 0.0, INFINITY as f64, &mut hit_rec) {
        let target = hit_rec.point() + hit_rec.normal() + Vec3::random_in_unit_sphere();
        let temp_ray = Ray::new(hit_rec.point(), target - hit_rec.point());
        return 0.5 * ray_color(&temp_ray, world);
        //return 0.5 * (hit_rec.normal_to_color() + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0); //blue to white blend
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    //linear blend
    // blendedValue = (1 − t)⋅startValue + t⋅endValue
    (1.0 - t) * start_value + t * end_value
}

fn render(image_width: usize, image_height: usize, samples_per_pixel: usize) {
    println!("P3\n{} {} \n255\n", image_width, image_height);

    let mut world: HittableList = HittableList::new();

    world.add(Sphere::new_hittable(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new_hittable(Point3::new(0.0, -100.5, -1.0), 100.0));

    let cam = Camera::default();

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
                let v = (j as f64) / image_height as f64;

                let ray = &cam.get_ray(u, v); //Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);
                pixel_color = pixel_color + ray_color(ray, &world);
            }

            Color::write_color(pixel_color, samples_per_pixel);
        }
    }
    eprintln!("\nDone\n");
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_HEIGHT: usize = 144;
    const IMAGE_WIDTH: usize = (IMAGE_HEIGHT as f64 * ASPECT_RATIO) as usize;
    const SAMPLE_PER_PIXEL: usize = 100;

    render(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLE_PER_PIXEL);
    eprintln!(
        "Rendered image with dimensions:\n {} x {}",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
}
