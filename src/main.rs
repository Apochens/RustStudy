use std::{cell::RefCell, rc::Rc};
mod render;

use render::{
    color::Color,
    ray::{HitRecord, Hittable, Ray}, 
    utils, vec3::Vec3, 
    hittable_list::HittableList,
    sphere::Sphere,
    camera::Camera, 
    color::write_color, 
    utils::random,
    material::{Lambertian, Metal}
};

fn ray_color(ray: &Ray, world: &dyn Hittable, depth: i32) -> Color {
    if depth <= 0 { return Color::new(0., 0., 0.) }

    let mut rec = HitRecord::new();

    if world.hit(ray, 0.001, utils::INFINITY, &mut rec) {
        let mut scattered = Ray::new(Vec3::new_empty(), Vec3::new_empty());
        let mut attenuation = Color::from(Vec3::new_empty());
        if rec.mat_ptr().as_ref().borrow().scatter(ray, &rec, &mut attenuation, &mut scattered) {
            attenuation * ray_color(&scattered, world, depth - 1)
        } else {
            Color::from(Vec3::new_empty())
        }
    } else {
        let mut unit_direction = ray.direction();
        unit_direction.to_unit();
        let t = 0.5 * (unit_direction.y() + 1.0);
        Color::from((1.0 - t) * Vec3::new(1.0, 1.0, 1.0) + t * Vec3::new(0.5, 0.7, 1.0))
    }
}

fn main() {

    // Image
    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;
    let samples_per_pixel = 100;
    let max_depth = 50i32;

    // World    
    let mut world = HittableList::new();
    let material_ground = Lambertian::new(Color::new(0.8, 0.8, 0.0));
    let material_center = Lambertian::new(Color::new(0.7, 0.3, 0.3));
    let material_left = Metal::new(Color::new(0.8, 0.8, 0.8), 0.3);
    let material_right = Metal::new(Color::new(0.8, 0.6, 0.2), 1.0);
    world.add(Rc::new(RefCell::new(Sphere::new(
        Vec3::new(0., -100.5, -1.),
        100.,
        Rc::new(RefCell::new(material_ground))
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Vec3::new(0., 0., -1.),
        0.5,
        Rc::new(RefCell::new(material_center))
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Vec3::new(-1., 0., -1.),
        0.5,
        Rc::new(RefCell::new(material_left))
    ))));
    world.add(Rc::new(RefCell::new(Sphere::new(
        Vec3::new(1., 0., -1.),
        0.5,
        Rc::new(RefCell::new(material_right))
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
                pixel_color = pixel_color + ray_color(&ray, &world, max_depth);
            }
            write_color(pixel_color, samples_per_pixel);
        }
        i -= 1;
    }
}