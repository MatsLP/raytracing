use crate::{
    geo::Vec3,
    scene::{ray_color, Scene},
};

pub struct Ray {
    pub base: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f64) -> Vec3 {
        self.base + t * self.dir
    }
}

pub struct Camera {
    origin: Vec3,
    viewport: Viewport,
}

struct Viewport {
    base: Vec3,
    v0: Vec3,
    v1: Vec3,
}

impl Camera {
    pub fn default() -> Self {
        Self {
            origin: Vec3::zero(),
            viewport: Viewport::default(),
        }
    }

    fn get_ray(&self, u: f64, v: f64) -> Ray {
        let base = self.origin;
        let dir = self.viewport.base + u * self.viewport.v0 + v * self.viewport.v1 - base;

        Ray { base, dir }
    }
}

impl Viewport {
    pub fn default() -> Self {
        Self {
            base: Vec3::of(-2.0, 1.0, -1.0),
            v0: Vec3::of(4.0, 0.0, 0.0),
            v1: Vec3::of(0.0, -2.0, 0.0),
        }
    }
}

pub fn render(camera: &Camera, scene: &Scene, img: &mut Image) {
    for y in 0..img.height {
        for x in 0..img.width {
            let mut color = Vec3::zero();
            const N_SAMPLES: i32 = 100;
            for _ in 0..N_SAMPLES {
                let u = (x as f64 + rand::random::<f64>()) / img.width as f64;
                let v = (y as f64 + rand::random::<f64>())/ img.height as f64;
                let ray = camera.get_ray(u, v);
                color += ray_color(&ray, scene);
            }
            
            *img.get_mut(x, y).unwrap() = color / N_SAMPLES as f64;
        }
    }
}

pub type Color = Vec3;

impl Color {
    fn ppm_string(&self) -> String {
        let clamp = |x| if x < 0.0 {0.0} else if x > 1.0 {1.0} else {x};
        let r = (clamp(self.x) * 255.99999).floor() as u8;
        let g = (clamp(self.y) * 255.99999).floor() as u8;
        let b = (clamp(self.z) * 255.99999).floor() as u8;
        format!("{r} {g} {b}")
    }
    pub fn from(r: f64, g: f64, b: f64) -> Self {
        assert!(0.0 <= r && r <= 1.0);
        assert!(0.0 <= g && g <= 1.0);
        assert!(0.0 <= b && b <= 1.0);
        Color { x: r, y: g, z: b }
    }
}

pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
}

impl Image {
    fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Color> {
        if x > self.width || y > self.height {
            return None;
        }
        return self.data.get_mut(y * self.width + x);
    }

    fn get(&self, x: usize, y: usize) -> Option<&Color> {
        if x > self.width || y > self.height {
            return None;
        }
        return self.data.get(y * self.width + x);
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::from(0.0, 0.0, 0.0,); width * height],
        }
    }

    pub fn test_image() -> Self {
        let width = 256;
        let height = 256;
        let data: Vec<Color> = vec![Color::from(0.0, 0.0, 0.0,); width * height];
        let mut img = Image {
            width,
            height,
            data,
        };
        for y in 0..height {
            eprintln!("lines remaining: {}", height - y);
            for x in 0..width {
                let r = y as f64 / (width - 1) as f64;
                let g = x as f64 / (height - 1) as f64;
                let b = 0.25 as f64;
                *img.get_mut(x, y).unwrap() = Color::from(r, g, b);
            }
        }
        img
    }

    pub fn write_as_ppm_to_stdout(&self) {
        println!("P3\n{w}\n{h}\n255\n", w = self.width, h = self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y).unwrap();
                println!("{}", c.ppm_string());
            }
        }
    }
}
