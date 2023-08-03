use std::{
    io::{Write},
    process::{Command, Stdio},
    thread::{self, ScopedJoinHandle}, error::Error, fmt::Display,
};

use crate::{
    geo::Vec3,
    random::random_f32,
    scene::{ray_color, Scene},
};

pub struct Ray {
    pub base: Vec3,
    pub dir: Vec3,
}

impl Ray {
    pub fn at(&self, t: f32) -> Vec3 {
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

    fn get_ray(&self, u: f32, v: f32) -> Ray {
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
    const N_THREADS: usize = 4;

    let process_row = move |col_skip: usize, img: &mut Image| {
        for y in 0..img.height {
            if y % N_THREADS != col_skip {
                continue;
            }
            for x in 0..img.width {
                let mut color = Vec3::zero();

                for _ in 0..img.samples_per_pixel {
                    let u = (x as f32 + random_f32()) / img.width as f32;
                    let v = (y as f32 + random_f32()) / img.height as f32;
                    let ray = camera.get_ray(u, v);
                    color += ray_color(&ray, scene, 0);
                }
                *img.get_mut(x, y).unwrap() = color;
            }
        }
    };
    if N_THREADS != 1 {
        thread::scope(|s| {
            let mut threads: Vec<ScopedJoinHandle<Image>> = vec![];
            for i in 0..N_THREADS {
                let j = i;
                let f = process_row.clone();
                let mut img = img.clone();
                threads.push(s.spawn(move || -> Image {
                    f(j, &mut img);
                    img
                }));
            }
            for join_handle in threads {
                let computed_partial_img = join_handle.join().expect("These threads can not panic.");
                img.add(&computed_partial_img);
            }
        });
    } else {
        process_row(0, img);
    }
}

pub type Color = Vec3;

impl Color {
    fn ppm_string(&self, samples_per_pixel: i32) -> String {
        let scale = 1.0f32 / samples_per_pixel as f32;
        let scale_and_clamp = |x: f32| {
            let y = (x * scale).sqrt();
            if y < 0.0 {
                0.0
            } else if y > 1.0 {
                1.0
            } else {
                y
            }
        };

        let r = (scale_and_clamp(self.x) * 255.99999).floor() as u8;
        let g = (scale_and_clamp(self.y) * 255.99999).floor() as u8;
        let b = (scale_and_clamp(self.z) * 255.99999).floor() as u8;
        format!("{r} {g} {b}")
    }
}

#[derive(Debug, Clone)]
pub struct Image {
    width: usize,
    height: usize,
    data: Vec<Color>,
    samples_per_pixel: i32,
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

    fn add(&mut self, rhs: &Image) {
        let mut rhs_iter = rhs.data.iter();
        for c in self.data.iter_mut() {
            *c += *(rhs_iter.next().unwrap());
        }
    }

    pub fn empty(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Color::of(0.0, 0.0, 0.0,); width * height],
            samples_per_pixel: 100,
        }
    }

    pub fn test_image() -> Self {
        let width = 256;
        let height = 256;
        let data: Vec<Color> = vec![Color::of(0.0, 0.0, 0.0,); width * height];
        let mut img = Image {
            width,
            height,
            data,
            samples_per_pixel: 100, // HACK
        };
        for y in 0..height {
            eprintln!("lines remaining: {}", height - y);
            for x in 0..width {
                let r = y as f32 / (width - 1) as f32;
                let g = x as f32 / (height - 1) as f32;
                let b = 0.25 as f32;
                *img.get_mut(x, y).unwrap() = Color::of(r, g, b);
            }
        }
        img
    }

    pub fn write_as_ppm_to_stdout(&self, samples_per_pixel: i32) {
        println!("P3\n{w}\n{h}\n255\n", w = self.width, h = self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let c = self.get(x, y).unwrap();
                println!("{}", c.ppm_string(samples_per_pixel));
            }
        }
    }

    pub fn write_to_display_process(&self) -> Result<(), SubprocessError> {
        let mut cmd = Command::new("display")
            .stdin(Stdio::piped())
            .spawn()?;
        {
            let mut stdin = cmd.stdin.take().expect("stdin is configured");

            let line = format!("P3\n{w}\n{h}\n255\n", w = self.width, h = self.height);
            stdin.write(line.as_bytes())?;

            for y in 0..self.height {
                for x in 0..self.width {
                    let c = self.get(x, y).unwrap();
                    let line = format!("{}\n", c.ppm_string(self.samples_per_pixel));
                    stdin.write(line.as_bytes())?;
                }
            }
        }
        cmd.wait()?;
        Ok(())
    }
}

impl From<std::io::Error> for SubprocessError {
    fn from(value: std::io::Error) -> Self {
        Self { cause: Box::new(value) }
    }
}


#[derive(Debug)]
pub struct SubprocessError {
    cause: Box<dyn Error + 'static>
}

impl Error for SubprocessError {
    fn source(&self) -> Option<&(dyn Error + 'static)> {
        None
    }
}

impl Display for SubprocessError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}