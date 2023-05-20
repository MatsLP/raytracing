use crate::{geo::Vec3, render::{Ray, Color}};

pub struct Scene {
    objects: Vec<Sphere>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add_sphere(&mut self, center: Vec3, radius: f64) {
        self.objects.push(Sphere { center, radius });
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
        Some(HitRecord {
            p,
            t: root,
            normal: (p-self.center) / self.radius
        })
    }
}

pub fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    for object in scene.objects.iter() {
        let Some(hit_record) = object.hit(ray, f64::MIN, f64::MAX) else {
            continue;
        };
        if hit_record.t > 0.0 {
            let N = (ray.at(hit_record.t) - object.center).unit();
            let normalize = |x: f64| 0.5 * (x + 1.0);
            return Color::of(normalize(N.x), normalize(N.y), normalize(N.z));
        }
    }
    let unit = ray.dir.unit();
    assert!(0.9999 <= unit.length() && unit.length() <= 1.00001 );
    let t = 0.5 * (unit.y + 1.0);
    assert!(0.0 <= t && t <= 1.0000);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0)
        + t * Color::from(0.5, 0.7, 1.0)
}
