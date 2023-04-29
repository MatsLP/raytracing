use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let aspect_ratio = 16.0 / 9.0;
    let image_width: usize = 400;
    let image_height: usize = (image_width as f64 / aspect_ratio) as usize;
    let mut img = render::Image::empty(image_width, image_height);

    let mut scene = scene::Scene::new();
    scene.add_sphere(geo::Vec3::of(0.0, 0.0, -1.0), 0.5);

    render::render(&render::Viewport::default(), &scene, &mut img);

    img.write_as_ppm_to_stdout();
    Ok(())
}

mod render;
mod geo;
mod scene;
