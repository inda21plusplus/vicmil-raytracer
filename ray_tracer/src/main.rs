pub mod error_handler;
pub mod vectors;

use error_handler::*;
use std::collections::LinkedList;
use std::fs::File;
use std::io::prelude::*;
use crate::vectors::*;

#[derive(Debug, Clone, Copy)]
struct Pixel {
    r: u8,
    g: u8,
    b: u8,
}

#[allow(dead_code)]
impl Pixel {
    pub fn black_pixel() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0
        }
    }
    pub fn new(r: u8, g: u8, b: u8) -> Self {
        Self {
            r,
            g,
            b
        }
    }
    // Input numbers between 0.0 - 1.0
    pub fn from_float(r: f64, g: f64, b: f64) -> Self {
        Self {
            r: (r*255.99) as u8,
            g: (g*255.99) as u8,
            b: (b*255.99) as u8,
        }
    }
}

impl From<Vec3> for Pixel {
    fn from(vec: Vec3) -> Self {
        Self::from_float(vec[0], vec[1], vec[2])
    }
}

struct ImageBuffer {
    pixels: Vec<Pixel>,
    w: usize,
    h: usize
}

#[allow(dead_code)]
impl ImageBuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            pixels: vec![Pixel::black_pixel(); width*height],
            w: width,
            h: height
        }
    }
    pub fn get_index(&self, x: usize, y: usize) -> RayTracerResult<usize> {
        if y < self.h && x < self.w {
            return Ok(y*self.w + x);
        }
        else {
            return Err(RayTracerError::OutsideImageBuffer);
        }
    }
    pub fn save_as_ppm_file(&self, file_name: &str) -> RayTracerResult<()> {
        // Open the file
        let mut file = File::create(file_name)?;

        // Write P3 to show colors are in ascii
        file.write_all(b"P3\n")?;

        // Specify image dimensions
        let dimension_text = self.w.to_string() + " " + self.h.to_string().as_str() + "\n";
        file.write(dimension_text.as_bytes())?;

        // Specify maximum value
        file.write(b"255\n")?;

        // Input pixel values
        for y in 0..self.h {
            for x in 0..self.w {
                let y = self.h - y - 1;
                let pixel = &self.pixels[self.get_index(x, y)?];
                let pixel_string = pixel.r.to_string() + " "
                    + pixel.g.to_string().as_str() + " "
                    + pixel.b.to_string().as_str();
                // Make sure it aligns
                let add_space_count = 12 - pixel_string.len();
                let pixel_string = pixel_string + String::from_utf8(vec![' ' as u8; add_space_count]).unwrap().as_str();
                file.write(pixel_string.as_bytes())?;
                if y % 8 == 8-1 {
                    file.write(b"\n")?;
                }
            }
            file.write(b"\n")?;
        }
        Ok(())
    }
}


#[derive(Debug, Clone, Copy)]
pub struct Sphere {
    center: Vec3,
    radious: f64,
    material: Material
}

impl Sphere {
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.origin - self.center;
        let a = dot(ray.direction, ray.direction);
        let b = dot(ray.direction, oc);
        let c = dot(oc, oc) - self.radious*self.radious;
        let discriminant = b*b - a*c;
        if discriminant < 0.0 {
            return None;
        }
        let mut hit_record = HitRecord::default();
        hit_record.material = Some(self.material);
        let temp = (-b - (b*b-a*c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            hit_record.t = temp;
            hit_record.p = ray.point_at_parameter(temp);
            hit_record.normal = (hit_record.p - self.center) / self.radious;
            return Some(hit_record);
        }
        let temp = (-b + (b*b-a*c).sqrt()) / a;
        if temp < t_max && temp > t_min {
            hit_record.t = temp;
            hit_record.p = ray.point_at_parameter(temp);
            hit_record.normal = (hit_record.p - self.center) / self.radious;
            return Some(hit_record);
        }
        return None;
    }
}

struct HitableList {
    list: LinkedList<Sphere>
}

impl HitableList {
    pub fn hit(&self, ray: Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut hit_record: Option<HitRecord> = None;
        let mut closest_so_far = t_max;
        for i in self.list.iter() {
            let hit =  i.hit(ray, t_min, closest_so_far);
            if hit.is_some() {
                closest_so_far = hit.as_ref().unwrap().t;
                hit_record = hit;
            }
        }
        return hit_record;
    }
    pub fn new() -> Self {
        Self {
            list: LinkedList::new()
        }
    }
}



/*fn get_color(ray: Ray, hit_list: &HitableList) -> Vec3 {
    let hit = hit_list.hit(ray, 0.0, f64::MAX);
    if hit.is_some() {
        let mut hit = hit.unwrap();
        let normal = hit.normal.unit_vector();
        //return Vec3::from_color(1.0, 1.0, 1.0);
        return 0.5*Vec3::new(normal.x()+1.0, normal.y()+1.0, normal.z()+1.0);
    }
    return Vec3::from_color(0.0, 0.0, 0.0);
}*/

extern crate trait_enum;
use trait_enum::trait_enum;

trait_enum!{
    #[derive(Debug, Clone, Copy)]
    pub enum Material: MaterialTrait {
        Metal,
        Lambertian
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Metal {
    albedo: Vec3,
    fuzz: f64
}

impl Metal {
    pub fn new(albedo: Vec3, fuzz: f64) -> Self {
        if fuzz < 1.0 {
            return Self {
                albedo,
                fuzz
            };
        }
        else {
            return Self {
                albedo,
                fuzz: 1.0
            };
        }
    }
}

impl MaterialTrait for Metal {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
        let reflected = reflect(&ray_in.direction, &hit_record.normal);
        let scattered = Ray::new(hit_record.p, reflected + self.fuzz*random_in_unit_sphere());
        *attenuation = self.albedo;
        if dot(scattered.direction, hit_record.normal) > 0.0 {
            return Some(scattered);
        }
        return None;
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Lambertian {
    albedo: Vec3
}

impl MaterialTrait for Lambertian {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord, attenuation: &mut Vec3) -> Option<Ray> {
        let target = hit_record.p + hit_record.normal + random_in_unit_sphere();
        let scattered = Ray::new(hit_record.p, target - hit_record.p);
        *attenuation = self.albedo;
        return Some(scattered);
    }
}

pub trait MaterialTrait {
    fn scatter(&self, ray_in: Ray, hit_record: &HitRecord, attenuation: &mut Vec3) -> Option<Ray>;
}

fn get_color(ray: Ray, hit_list: &HitableList, depth: i64) -> Vec3 {
    let hit = hit_list.hit(ray, 0.001, f64::MAX);
    if hit.is_some() {
        let mut hit = hit.unwrap();
        let mut attenuation: Vec3 = Vec3::new(0.0, 0.0, 0.0);
        let scattered = hit.material.unwrap().scatter(ray, &hit, &mut attenuation);
        //println!("att: {}", attenuation.to_string());
        if depth < 50 && scattered.is_some() {
            return attenuation*get_color(scattered.unwrap(), hit_list, depth+1);
        }
        else {
            return Vec3::from_color(0.0, 0.0, 0.0);
        }
    }
    else {
        let unit_direction = ray.direction.unit_vector();
        let t = 0.5*(unit_direction.y() + 1.0);
        return (1.0-t)*Vec3::new(1.0, 1.0, 1.0) + t*Vec3::new(0.5, 0.7, 1.0);
    }
    //return Vec3::from_color(0.0, 0.0, 0.0);
}

#[derive(Debug, Clone, Copy)]
pub struct Ray {
    origin: Vec3,
    direction: Vec3
}

impl Ray {
    pub fn new(origin: Vec3, direction: Vec3) -> Self {
        Self {
            origin,
            direction
        }
    }
    pub fn point_at_parameter(&self, t: f64) -> Vec3 {
        return self.origin + self.direction * t;
    }
}

fn reflect(v: &Vec3, n: &Vec3) -> Vec3 {
    return (*v) - 2.0*dot(*v, *n) * (*n);
}

#[derive(Debug, Clone)]
pub struct HitRecord {
    t: f64,
    p: Vec3,
    normal: Vec3,
    material: Option<Material>
}

impl Default for HitRecord {
    fn default() -> Self {
        Self {
            t: f64::default(),
            p: Vec3::default(),
            normal: Vec3::default(),
            material: None,
        }
    }
}

struct Camera {
    origin: Vec3,
    lower_left_corner: Vec3,
    horizontal: Vec3,
    vertical: Vec3
}

impl Camera {
    pub fn new() -> Self {
        let lower_left_corner = Vec3::new(-2.0, -1.0, -1.0);
        let horizontal = Vec3::new(4.0, 0.0, 0.0);
        let vertical = Vec3::new(0.0, 2.0, 0.0);
        let origin = Vec3::new(0.0, 0.0, 0.0);
        Self {
            lower_left_corner,
            horizontal,
            vertical,
            origin
        }
    }
    pub fn get_ray(&self, u: f64, v: f64) -> Ray {
        Ray::new(self.origin, self.lower_left_corner + self.horizontal * u + self.vertical * v)
    }
}

pub fn random_from_0_to_1() -> f64 {
    let precision = 10000;
    use rand::Rng;
    rand::thread_rng().gen_range(0..precision) as f64 / precision as f64
}

pub fn random_in_unit_sphere() -> Vec3 {
    loop {
        //println!("get unit vector!");
        let p = 2.0*Vec3::new(random_from_0_to_1(), random_from_0_to_1(), random_from_0_to_1()) 
        - Vec3::new(1.0, 1.0, 1.0);
        //print!("Vector: {}", p.to_string());
        if p.squared_length() <= 1.0 {
            //println!("return unit vector!");
            return p;
        }
    }
}

fn main() {
    println!("Running program!");
    let mut buf = ImageBuffer::new(400, 200);
    let camera = Camera::new();

    let ns = 100;
    
    let mut hit_list = HitableList::new();
    hit_list.list.push_back(Sphere { center: Vec3::new(0.0, 0.0, -1.0), radious: 0.5,
        material: Material::Lambertian(Lambertian{albedo: Vec3::new(0.8, 0.3, 0.3)})});

    hit_list.list.push_back(Sphere { center: Vec3::new(0.0, -100.5, -1.0), radious: 100.0,
        material: Material::Lambertian(Lambertian{albedo: Vec3::new(0.8, 0.8, 0.0)})});

    hit_list.list.push_back(Sphere { center: Vec3::new(1.0, 0.0, -1.0), radious: 0.5, 
        material: Material::Metal(Metal::new(Vec3::new(0.8, 0.6, 0.2), 0.3))});

    hit_list.list.push_back(Sphere { center: Vec3::new(-1.0, 0.0, -1.0), radious: 0.5, 
        material: Material::Metal(Metal::new(Vec3::new(0.8, 0.8, 0.8), 1.0))});

    for y in 0..buf.h {
        for x in 0..buf.w {
            let mut col = Vec3::new(0.0, 0.0, 0.0);
            for _ in 0..ns {
                let precision = 10000;
                let rand1 = random_from_0_to_1();
                let u = (x as f64 + rand1) / buf.w as f64;
                let rand2 = random_from_0_to_1();
                let v = (y as f64 + rand2) / buf.h as f64;
                let ray = camera.get_ray(u, v);
                let color = get_color(ray, &hit_list, 0);
                col = col + color;
            }
            // Compensate for antialiasing
            col = col / ns as f64;

            // Make color brighter
            col = col.gamma2_on_color();

            // Save pixel value
            let index = buf.get_index(x, y).unwrap();
            let pixel = &mut buf.pixels[index];
            *pixel = Pixel::from(col);
        }
    }
    let _ = buf.save_as_ppm_file("foo.ppm");

    let mut vec1 = crate::vectors::Vec3::new(1.0, 2.0, 3.0);
    vec1[0] += vec1[1];
    println!("{}", vec1[0]);

    println!("Program finished!");
}
