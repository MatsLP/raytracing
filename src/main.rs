use std::error::Error;

#[derive(Clone, Copy)]
struct Color {
    r: f32,
    g: f32,
    b: f32,
}
struct Image {
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

    fn test_image() -> Self {
        let width = 256;
        let height = 256;
        let data: Vec<Color> = vec![
            Color {
                r: 0.0,
                g: 0.0,
                b: 0.0
            };
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
                let r = y as f32 / (width - 1) as f32;
                let g = x as f32 / (height - 1) as f32;
                let b = 0.25 as f32;
                *img.get_mut(x, y).unwrap() = Color { r, g, b };
            }
        }
        img
    }

    fn write_as_ppm_to_stdout(&self) {
        println!("P3\n{w}\n{h}\n255\n", w = self.width, h = self.height);

        for y in 0..self.height {
            for x in 0..self.width {
                let r = (self.get(x, y).unwrap().r * 255.99999) as u8;
                let g = (self.get(x, y).unwrap().g * 255.99999) as u8;
                let b = (self.get(x, y).unwrap().b * 255.99999) as u8;
                println!("{r} {g} {b}\n");
            }
        }
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    Image::test_image().write_as_ppm_to_stdout();
    Ok(())
}

mod geo {
    use std::ops;

    #[derive(Clone, Copy, Debug)]
    pub struct Vec3 {
        x: f64,
        y: f64,
        z: f64,
    }

    impl Vec3 {
        fn zero() -> Self {
            return Self {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            };
        }

        fn of(x: f64, y: f64, z: f64) -> Self {
            return Self { x, y, z };
        }

        fn length_squared(&self) -> f64 {
            self.x * self.x * self.y * self.y * self.z * self.z
        }

        fn length(&self) -> f64 {
            self.length_squared().sqrt()
        }

        fn dot(&self, rhs: &Self) -> f64 {
            self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
        }

        fn cross(&self, rhs: &Self) -> Self {
            Self {
                x: self.y * rhs.z - self.z * rhs.y,
                y: self.z * rhs.x - self.x * rhs.z,
                z: self.x * rhs.y - self.y * rhs.x,
            }
        }
        
        fn unit(&self) -> Self {
            self / self.length()
        }
    }

    impl ops::AddAssign<Vec3> for Vec3 {
        fn add_assign(&mut self, rhs: Vec3) {
            self.x += rhs.x;
            self.y += rhs.y;
            self.z += rhs.z;
        }
    }

    impl ops::Neg for Vec3 {
        type Output = Vec3;

        fn neg(self) -> Self::Output {
            Vec3 {
                x: -self.x,
                y: -self.y,
                z: -self.z,
            }
        }
    }

    impl ops::MulAssign<f64> for Vec3 {
        fn mul_assign(&mut self, rhs: f64) {
            self.x *= rhs;
            self.y *= rhs;
            self.z *= rhs;
        }
    }

    impl ops::DivAssign<f64> for Vec3 {
        fn div_assign(&mut self, rhs: f64) {
            self.x /= rhs;
            self.y /= rhs;
            self.z /= rhs;
        }
    }

    impl ops::Add<Vec3> for Vec3 {
        type Output = Vec3;

        fn add(self, rhs: Vec3) -> Self::Output {
            Vec3 {
                x: self.x + rhs.x,
                y: self.y + rhs.y,
                z: self.z + rhs.z,
            }
        }
    }

    impl ops::Sub<Vec3> for Vec3 {
        type Output = Vec3;

        fn sub(self, rhs: Vec3) -> Self::Output {
            Vec3 {
                x: self.x - rhs.x,
                y: self.y - rhs.y,
                z: self.z - rhs.z,
            }
        }
    }

    impl ops::Mul<Vec3> for Vec3 {
        type Output = Vec3;

        fn mul(self, rhs: Vec3) -> Self::Output {
            Vec3 {
                x: self.x * rhs.x,
                y: self.y * rhs.y,
                z: self.z * rhs.z,
            }
        }
    }

    impl ops::Mul<Vec3> for f64 {
        type Output = Vec3;

        fn mul(self, rhs: Vec3) -> Self::Output {
            Vec3 {
                x: rhs.x * self,
                y: rhs.y * self,
                z: rhs.z * self,
            }
        }
    }

    impl ops::Div<f64> for &Vec3 {
        type Output = Vec3;

        fn div(self, rhs: f64) -> Self::Output {
            Vec3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }

    impl ops::Div<f64> for Vec3 {
        type Output = Vec3;

        fn div(self, rhs: f64) -> Self::Output {
            Vec3 {
                x: self.x / rhs,
                y: self.y / rhs,
                z: self.z / rhs,
            }
        }
    }
}
