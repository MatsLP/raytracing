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

pub trait Hittable {
    fn is_hit(&self, ray: &Ray) -> bool;
}

struct Sphere {
    center: Vec3,
    radius: f64,
}

impl Hittable for Sphere {
    fn is_hit(&self, ray: &Ray) -> bool {
        let oc = ray.base - self.center;
        let a = ray.dir.length_squared();
        let b = 2.0 * oc.dot(&ray.dir);
        let c = oc.length_squared() - self.radius * self.radius;
        let discriminant = b * b - 4.0 * a * c;
        discriminant > 0.0
    }
}

pub fn ray_color(ray: &Ray, scene: &Scene) -> Color {
    for object in scene.objects.iter() {
        if object.is_hit(ray) {
            return Color::of(1.0, 0.0, 0.0);
        }
    }
    let unit = ray.dir.unit();
    assert!(0.9999 <= unit.length() && unit.length() <= 1.00001 );
    let t = 0.5 * (unit.y + 1.0);
    assert!(0.0 <= t && t <= 1.0000);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0)
        + t * Color::from(0.5, 0.7, 1.0)
}
