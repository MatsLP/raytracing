use crate::geo::Vec3;
    
struct Ray {
    base: Vec3,
    dir: Vec3,
}

impl Ray {
    fn at(&self, t: f64) -> Vec3 {
        self.base + t * self.dir
    }
}

pub struct Viewport {
    base: Vec3,
    v0: Vec3,
    v1: Vec3,
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

pub fn render(viewport: &Viewport, img: &mut Image) {
    for y in 0..img.height {
        for x in 0..img.width {
            let base = Vec3::zero();
            let dir = 
                viewport.base 
                + (x as f64 / img.width as f64) * viewport.v0
                + (y as f64 / img.height as f64) * viewport.v1
                - base;

            let ray = Ray {
                base,
                dir
            };
            *img.get_mut(x, y).unwrap() = ray_color(&ray);
        }
    }
}

fn ray_color(ray: &Ray) -> Color {
    let unit = ray.dir.unit();
    assert!(0.9999 <= unit.length() && unit.length() <= 1.00001 );
    let t = 0.5 * (unit.y + 1.0);
    assert!(0.0 <= t && t <= 1.0000);
    (1.0 - t) * Color::from(1.0, 1.0, 1.0)
        + t * Color::from(0.5, 0.7, 1.0)
}

type Color = Vec3;


impl Color {
    fn ppm_string(&self) -> String {
        assert!(-0.0001 <= self.x && self.x <= 1.00001);
        assert!(-0.0001 <= self.y && self.y <= 1.00001);
        assert!(-0.0001 <= self.z && self.z <= 1.00001);
        let r = (self.x * 255.99999).floor() as u8;
        let g = (self.y * 255.99999).floor() as u8;
        let b = (self.z * 255.99999).floor() as u8;
        format!("{r} {g} {b}")
    }
    fn from(r: f64, g: f64, b: f64) -> Self{
        assert!(0.0 <= r && r <= 1.0);
        assert!(0.0 <= g && g <= 1.0);
        assert!(0.0 <= b && b <= 1.0);
        Color {x: r, y: g, z: b}
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
            data: vec![
                Color::from(
                    0.0,
                    0.0,
                    0.0,
                );
                width * height
            ],
        }
    }

    pub fn test_image() -> Self {
        let width = 256;
        let height = 256;
        let data: Vec<Color> = vec![
            Color::from(
                0.0,
                0.0,
                0.0,
            );
            width * height
        ];
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