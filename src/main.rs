mod lib;
use lib::color::*;
use lib::ray::*;
use lib::vec3::*;

#[allow(dead_code)]
fn write_ppm(image_width: usize, image_height: usize) {
    println!("P3\n{} {} \n255\n", image_width, image_height);

    for y in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", y);
        for x in 0..image_width {
            let pixel_color = Color::new(
                x as f64 / image_width as f64,
                y as f64 / image_height as f64,
                0.25,
            );

            Color::write_color(pixel_color);
        }
    }
    eprintln!("\nDone\n");
}

fn ray_color(ray: &Ray) -> Color {
    if hit_sphere(&Point3::new(0.0, 0.0, -1.0), 0.5, &ray) {
        return Color::new(1.0, 0.0, 0.0);
    }

    let unit_direction: Vec3 = Vec3::unit_vector(&ray.direction());
    let t = 0.5 * (unit_direction.y() + 1.0); //blue to white blend
    let start_value = Color::new(1.0, 1.0, 1.0);
    let end_value = Color::new(0.5, 0.7, 1.0);

    //linear blend
    // blendedValue = (1 − t)⋅startValue + t⋅endValue
    (1.0 - t) * start_value + t * end_value
}

fn render(image_width: usize, image_height: usize) {
    println!("P3\n{} {} \n255\n", image_width, image_height);

    let origin: Point3 = Point3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(4.0, 0.0, 0.0);
    let vertical = Vec3::new(0.0, 2.25, 0.0);
    let lower_left_corner: Point3 =
        origin - horizontal / 2.0 - vertical / 2.0 - Vec3::new(0.0, 0.0, 1.0);

    for j in (0..image_height).rev() {
        eprintln!("\rScanlines remaining: {}", j);
        for i in 0..image_width {
            let u = i as f64 / image_width as f64;
            let v = j as f64 / image_height as f64;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical);

            let pixel_color = ray_color(&r);
            Color::write_color(pixel_color);
        }
    }
    eprintln!("\nDone\n");
}

fn hit_sphere(center: &Point3, radius: f64, r: &Ray) -> bool {
    // t^2b⋅b + 2tb⋅(A−C) + (A−C)⋅(A−C) − r^2 = 0

    let oc: Vec3 = r.origin() - *center; // (A-C)
    let a = &r.direction().dot(&r.direction()); // b.b
    let b = 2.0 * &oc.dot(&r.direction()); // (A-C)⋅b
    let c = &oc.dot(&oc) - radius * radius; // (A-C)⋅(A-C)
    let discriminant = b * b - 4.0 * a * c;

    discriminant > 0.0
}

fn main() {
    const ASPECT_RATIO: f64 = 16.0 / 9.0;
    const IMAGE_WIDTH: usize = 384;
    const IMAGE_HEIGHT: usize = (IMAGE_WIDTH as f64 / ASPECT_RATIO) as usize;

    render(IMAGE_WIDTH, IMAGE_HEIGHT);
    eprintln!(
        "Rendered image with dimensions:\n {} x {}",
        IMAGE_WIDTH, IMAGE_HEIGHT
    );
}
