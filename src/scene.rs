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
    fn hit(&self, ray: &Ray) -> f64;
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn hit(&self, ray: &Ray) -> f64 {
        let oc = ray.base - self.center;
        let a = ray.dir.length_squared();
        let b = 2.0 * oc.dot(&ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        if discriminant <= 0.0 {
            return -1.0;
        }
        return (-b -discriminant.sqrt()) / (2.0 * a);
    }
}

pub fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    for object in scene.objects.iter() {
        let t = object.hit(ray);
        if t > 0.0 {
            let N = (ray.at(t) - object.center).unit();
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
