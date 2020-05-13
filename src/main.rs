mod lib;

use lib::{
    camera::*, color::*, hittable::*, hittable_list::*, ray::*, rt_math::*, sphere::*, vec3::*,
};

use rand::prelude::*;

//look of ray hits something. if it doesn't color the background. if it does, delegate coloring
//the 'hit()' function
fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut hit_rec = HitRecord::default();

    if world.hit(ray, 0.0, INFINITY, &mut hit_rec) {
        return 0.5 * (hit_rec.normal_to_color() + Color::new(1.0, 1.0, 1.0));
    }

    let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0); //blue to white blend
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    //linear blend for the background
    // blendedValue = (1 − t)⋅startValue + t⋅endValue
    (1.0 - t) * start_value + t * end_value
}

fn render(image_width: usize, image_height: usize, samples_per_pixel: usize) {
    println!("P3\n{} {} \n255\n", image_width, image_height);

    let cam = Camera::default();
    let mut world = HittableList::new();

    world.add(Sphere::new(Point3::new(0.0, 0.0, -1.0), 0.5));
    world.add(Sphere::new(Point3::new(0.0, -100.5, -1.0), 100.0));

    //random number generator
    let mut rng = rand::thread_rng();

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);

        for i in 0..image_width {
            let mut pixel_color = Color::default();

            for _ in 0..samples_per_pixel {
                let u = ((i + rng.gen::<usize>()) / (image_width - 1)) as f64;
                let v = ((j + rng.gen::<usize>()) / (image_width - 1)) as f64;
                let ray = &cam.get_ray(u, v);
                pixel_color += ray_color(ray, &world);
            }
            Color::write_color(&pixel_color, samples_per_pixel as f64);
        }
    }
    eprintln!("\nDone\n");
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 200;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;
    const SAMPLES_PER_PIXEL: usize = 100;

    render(IMAGE_WIDTH, IMAGE_HEIGHT, SAMPLES_PER_PIXEL);

    eprintln!(
        "Rendered image with dimensions:\n {} x {}",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
}
