use std::{cell::RefCell, io::Write, rc::Rc};
mod render;

use render::{
    color::Color,
    ray::{HitRecord, Hittable, Ray}, 
    utils, vec3::Vec3, 
    hittable_list::HittableList,
    sphere::Sphere
};

use crate::render::{camera::Camera, color::write_color, utils::random};

fn hit_shpere(center: &Vec3, radius: f32, ray: &Ray) -> f32 {

    let oc = ray.origin() - *center;
    let direction = ray.direction();

    let a = direction.len_square();
    let half_b = oc.dot_mul(&direction);
    let c = oc.len_square() - radius * radius;
    let discriminant = half_b*half_b - a*c;

    if discriminant < 0.0 { -1.0 } 
    else { (-half_b - discriminant.sqrt()) / a }
}

fn ray_color(ray: &Ray, world: &dyn Hittable) -> Color {
    let mut rec = HitRecord::new();

    if world.hit(ray, 0., utils::INFINITY, &mut rec) {
        Color::from(0.5 * (rec.normal() + Vec3::new(1., 1., 1.)))
    } else {
        let mut unit_direction = ray.direction();
        unit_direction.to_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::from((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
    }
}

#[derive(Debug)]
struct Render {
    image_height: i32,
    image_width: i32
}

impl Render {
    fn new(image_width: i32, image_height: i32) -> Self {
        Self{image_width, image_height}
    }

    fn default(&self, fp: &str) -> std::io::Result<String> {
        let mut file = std::fs::File::create(fp)?;

        file.write(b"P3\n")?;
        file.write(format!("{} {}\n", self.image_width, self.image_height).as_bytes())?;
        file.write(b"255\n")?;

        let mut i = self.image_height - 1;
        while i >= 0 {
            for j in 0..self.image_width  {
                let r: f32 = j as f32 / (self.image_width - 1) as f32;
                let g: f32 = i as f32 / (self.image_height - 1) as f32;
                let b: f32 = 0.25;

                let color = Color::new(r, g, b);
                color.print_color_to_file(&mut file)?;
            }
            i -= 1;
        }

        Ok("Finished rendering.".to_string())
    }
}


fn main() {

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;

    // World
    let mut world = HittableList::new();
    world.add(Rc::new(RefCell::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.
    ))));


    // Camera
    let camera = Camera::new();

    // Render
    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut i = image_height - 1;
    while i >= 0 {
        for j in 0..image_width {
            let mut pixel_color = Color::new(0., 0., 0.);
            for _ in 0..samples_per_pixel {
                let u = (j as f32 + random()) / (image_width - 1) as f32;
                let v = (i as f32 + random()) / (image_height - 1) as f32;
                let ray = camera.get_ray(u, v);
                pixel_color = pixel_color + ray_color(&ray, &world);
            }
            write_color(pixel_color, samples_per_pixel);
        }
        i -= 1;
    }
    

    // let render = Render::new(256, 256);
    // let fp = "./image.ppm";
    // match render.default(fp) {
    //     Ok(msg) => println!("{}", msg),
    //     Err(msg) => println!("{}", msg)
    // }
}