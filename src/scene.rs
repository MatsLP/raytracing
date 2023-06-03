use crate::{geo::Vec3, render::{Ray, Color}};

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add_sphere(&mut self, center: Vec3, radius: f64) {
        let sphere = Sphere { center, radius };
        self.objects.push(Object::Sphere(sphere));
    }
}

enum Object {
    Sphere(Sphere)
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match self {
            Object::Sphere(s) => s.hit(ray, t_min, t_max)
        }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

struct HitRecord {
    p: Vec3,
    normal: Vec3,
    t: f64,
    face: FACE,
}

impl HitRecord {
    fn new(p: Vec3, t: f64, unaligned_normal: Vec3, ray_dir: Vec3) -> Self{
        let tmp = if ray_dir.dot(&unaligned_normal) < 0.0 {
            (FACE::FRONT, unaligned_normal)
        } else {
            (FACE::BACK, -unaligned_normal)
        };
        HitRecord { p, normal: tmp.1, t, face: tmp.0 }
    }
}

enum FACE {
    FRONT, BACK
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let oc = ray.base - self.center;
        let a = ray.dir.length_squared();
        let half_b = oc.dot(&ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = half_b * half_b - a * c;
        if discriminant <= 0.0 {
            return None;
        }
        let sqrtd = discriminant.sqrt();
        let mut root = (-half_b - sqrtd) / a;
        if root < t_min || t_max < root {
            root = (-half_b + sqrtd) / a;
            if root < t_min || t_max < root {
                return None;
            }
        }
        let p = ray.at(root);
        Some(HitRecord::new(p, root, (p-self.center) / self.radius, ray.dir))
    }
}

const MAX_BOUNCE_DEPTH: i32 = 50;

pub fn ray_color(ray: &Ray, scene: &Scene, depth: i32) -> Color {
    if depth == MAX_BOUNCE_DEPTH {
        return Color::zero();
    }
    for object in scene.objects.iter() {

        let Some(hit_record) = object.hit(ray, 0.0, f64::MAX) else {
            continue;
        };
        let target = Ray {
            base: hit_record.p,
            dir: hit_record.p + hit_record.normal + Vec3::random_in_unit_sphere()
        };
        return 0.5 * ray_color(&target, scene, depth + 1);
    }
    let unit = ray.dir.unit();
    assert!(0.9999 <= unit.length() && unit.length() <= 1.00001 );
    let t = 0.5 * (unit.y + 1.0);
    assert!(0.0 <= t && t <= 1.0000);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0)
        + t * Color::from(0.5, 0.7, 1.0)
}
