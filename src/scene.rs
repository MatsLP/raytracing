use crate::{
    geo::Vec3,
    render::{Color, Ray},
};

pub struct Scene {
    objects: Vec<Object>,
}

impl Scene {
    pub fn new() -> Self {
        Self { objects: vec![] }
    }

    pub fn add_sphere(&mut self, center: Vec3, radius: f64, material: Material) {
        self.objects.push(Object {
            shape: Shape::Sphere { center, radius },
            material,
        });
    }

    fn closest_hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        let mut closest_hit: Option<HitRecord> = None;
        let mut closest_t_abs = f64::MAX;
        for object in self.objects.iter() {
            let Some(hit_record) = object.hit(ray, t_min, t_max) else {
                continue;
            };
            let t_abs = hit_record.t.abs();
            if t_abs < closest_t_abs {
                closest_t_abs = t_abs;
                closest_hit = Some(hit_record);
            }
        }
        closest_hit
    }
}

struct Object {
    shape: Shape,
    material: Material,
}

enum Shape {
    Sphere { center: Vec3, radius: f64 },
}

pub enum Material {
    Lambertian { albedo: Color },
    Metal { albedo: Color, fuzz: f64 },
}

impl Material {
    fn scatter(&self, ray_in: &Ray, hit_record: &HitRecord) -> Option<ScatterResult> {
        match self {
            Self::Lambertian { albedo } => {
                let mut scatter_direction = hit_record.normal + Vec3::random_on_unit_sphere();
                if scatter_direction.near_zero() {
                    scatter_direction = hit_record.normal;
                }
                let ray_out = Ray {
                    base: hit_record.p,
                    dir: scatter_direction,
                };
                Some(ScatterResult {
                    ray_out,
                    attenuation: *albedo,
                })
            }
            Self::Metal { albedo, fuzz } => {
                let reflected = ray_in.dir.unit().reflect(&hit_record.normal);
                if reflected.dot(&hit_record.normal) > 0.0 {
                    let dir = reflected + fuzz * Vec3::random_in_unit_sphere();
                    let ray_out = Ray {
                        base: hit_record.p,
                        dir,
                    };
                    Some(ScatterResult {
                        ray_out,
                        attenuation: *albedo,
                    })
                } else {
                    None
                }
            }
        }
    }
}

struct ScatterResult {
    ray_out: Ray,
    attenuation: Color,
}

impl Hittable for Object {
    fn hit(&self, ray: &Ray, t_min: f64, t_max: f64) -> Option<HitRecord> {
        match &self.shape {
            Shape::Sphere { center, radius } => {
                let oc = ray.base - center;
                let a = ray.dir.length_squared();
                let half_b = oc.dot(&ray.dir);
                let c = oc.length_squared() - radius * radius;
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
                Some(HitRecord::new(
                    p,
                    root,
                    (p - center) / radius,
                    ray.dir,
                    &self.material,
                ))
            }
        }
    }
}

trait Hittable {
    fn hit(&self, ray: &Ray, tmin: f64, tmax: f64) -> Option<HitRecord>;
}

struct HitRecord<'a> {
    p: Vec3,
    normal: Vec3,
    t: f64,
    face: FACE,
    material: &'a Material,
}

impl<'a> HitRecord<'a> {
    fn new(p: Vec3, t: f64, unaligned_normal: Vec3, ray_dir: Vec3, material: &'a Material) -> Self {
        let tmp = if ray_dir.dot(&unaligned_normal) < 0.0 {
            (FACE::FRONT, unaligned_normal)
        } else {
            (FACE::BACK, -unaligned_normal)
        };
        HitRecord {
            p,
            normal: tmp.1,
            t,
            face: tmp.0,
            material,
        }
    }
}

enum FACE {
    FRONT,
    BACK,
}

const MAX_BOUNCE_DEPTH: i32 = 50;

pub fn ray_color(ray: &Ray, scene: &Scene, depth: i32) -> Color {
    if depth == MAX_BOUNCE_DEPTH {
        return Color::zero();
    }

    match scene.closest_hit(ray, 0.001, f64::MAX) {
        Some(hit_record) => {
            let scatter_result = hit_record.material.scatter(ray, &hit_record);
            match scatter_result {
                Some(scatter_result) => {
                    scatter_result.attenuation
                        * ray_color(&scatter_result.ray_out, scene, depth + 1)
                }
                None => Color::zero(),
            }
        }
        None => {
            let unit = ray.dir.unit();
            assert!(0.9999 <= unit.length() && unit.length() <= 1.00001);
            let t = 0.5 * (unit.y + 1.0);
            assert!(0.0 <= t && t <= 1.0000);
            (1.0 - t) * Color::from(1.0, 1.0, 1.0) + t * Color::from(0.5, 0.7, 1.0)
        }
    }
}
