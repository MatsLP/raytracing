use std::error::Error;

use cpu_time::ProcessTime;
use render::Color;
use scene::Material;

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 600;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;

    let mut img = render::Image::empty(image_width, image_height);

    let mut scene = scene::Scene::new();

    let material_ground = Material::Lambertian {
        albedo: Color::of(0.8, 0.8, 0.0),
    };
    let material_center = Material::Lambertian {
        albedo: Color::of(0.1, 0.2, 0.5),
    };
    let material_left = Material::Dieletric {
        index_of_refraction: 1.5,
    };
    let material_right = Material::Metal {
        albedo: Color::of(0.8, 0.6, 0.2),
        fuzz: 1.0,
    };

    scene.add_sphere(geo::Vec3::of(0.0, -100.5, -1.0), 100.0, material_ground);
    scene.add_sphere(geo::Vec3::of(0.0, 0.0, -1.0), 0.5, material_center);
    scene.add_sphere(geo::Vec3::of(-1.0, 0.0, -1.0), 0.5, material_left);
    scene.add_sphere(geo::Vec3::of(-1.0, 0.0, -1.0), -0.4, material_left);
    scene.add_sphere(geo::Vec3::of(1.0, 0.0, -1.0), 0.5, material_right);

    let start = ProcessTime::now();
    render::render(&render::Camera::default(), &scene, &mut img);
    println!(
        "Raytracing took {:?}ms of cpu time.",
        start.elapsed().as_millis()
    );

    img.write_to_display_process()?;
    Ok(())
}

mod geo;
mod random;
mod render;
mod scene;
