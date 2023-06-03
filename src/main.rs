use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 800;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    
    let mut img = render::Image::empty(image_width, image_height);

    let mut scene = scene::Scene::new();
    scene.add_sphere(geo::Vec3::of(0.0, 0.0, -1.0), 0.5);
    scene.add_sphere(geo::Vec3::of(0.0, -100.5, -1.0), 100.0);


    render::render(&render::Camera::default(), &scene, &mut img);

    img.write_to_display_process();
    Ok(())
}

mod render;
mod geo;
mod scene;
mod random;
