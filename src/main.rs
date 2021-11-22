use std::io::Write;
mod render;
use render::{vec3::Vec3, color::Color, ray::Ray};

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

fn ray_color(r: &Ray) -> Color {
    let center = Vec3::new(0.0, 0.0, 0.0);
    let t = hit_shpere(&center, 0.5, r);
    if t > 0.0 {
        let mut N = r.at(t) - center;
        N.to_unit();
        Color::from(0.5 * (N + Vec3::new(1.0, 1.0, 1.0)))
    } else {
        let mut unit_direction = r.direction();
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

    let aspect_ratio: f32 = 16.0 / 9.0;
    let image_width: i32 = 400;
    let image_height: i32 = (image_width as f32 / aspect_ratio) as i32;

    let viewport_height: f32 = 2.0;
    let viewport_width: f32 = viewport_height * aspect_ratio;
    let focal_lenght: f32 = 1.0;

    let origin = Vec3::new(0.0, 0.0, 0.0);
    let horizontal = Vec3::new(viewport_width, 0.0, 0.0);
    let vertical = Vec3::new(0.0, viewport_height, 0.0);
    let lower_left_corner = origin - horizontal/2.0 - vertical/2.0 - Vec3::new(0.0, 0.0, focal_lenght);

    println!("P3");
    println!("{} {}", image_width, image_height);
    println!("255");

    let mut i = image_height - 1;
    while i >= 0 {
        for j in 0..image_width {
            let u = j as f32 / (image_width - 1) as f32;
            let v = i as f32 / (image_height - 1) as f32;
            let r = Ray::new(origin, lower_left_corner + u * horizontal + v * vertical - origin);
            let pixel_color = ray_color(&r);

            pixel_color.print_color();
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