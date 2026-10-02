mod camera;
mod framebuffer;
mod planet;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use framebuffer::{Framebuffer, rgb};
use minifb::{Key, Scale, Window, WindowOptions};
use planet::{Planet, create_planets};
use sphere::Sphere;
use std::time::Instant;
use vector::Vec3;

const WIDTH: usize = 400;
const HEIGHT: usize = 300;

fn shade(sphere: &Sphere, point: Vec3, base_color: u32, sun_position: Vec3) -> u32 {
    let normal = sphere.normal_at(point);

    let light_direction = (sun_position - point).normalize();

    let diffuse = normal.dot(&light_direction).max(0.0);

    let brightness = (0.12 + diffuse * 0.88).min(1.0);

    let r = ((base_color >> 16) & 255) as f32;
    let g = ((base_color >> 8) & 255) as f32;
    let b = (base_color & 255) as f32;

    rgb(
        (r * brightness) as u32,
        (g * brightness) as u32,
        (b * brightness) as u32,
    )
}

fn render(framebuffer: &mut Framebuffer, camera: &Camera, planets: &[Planet]) {
    let background = rgb(2, 3, 10);

    framebuffer.clear(background);

    let sun = Sphere::new(Vec3::new(0.0, 0.0, 0.0), 1.2);

    for y in 0..HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, HEIGHT);

            let mut closest_t = f32::INFINITY;
            let mut color = background;

            if let Some(t) = sun.intersect(&ray) {
                closest_t = t;
                color = rgb(255, 185, 35);
            }

            for planet in planets {
                let sphere = planet.sphere();

                if let Some(t) = sphere.intersect(&ray) {
                    if t < closest_t {
                        closest_t = t;

                        let point = ray.at(t);

                        color = shade(&sphere, point, planet.color, Vec3::new(0.0, 0.0, 0.0));
                    }
                }
            }

            framebuffer.set_pixel(x, y, WIDTH, color);
        }
    }
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut planets = create_planets();

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 22.0);

    let mut window = Window::new(
        "Solar System Raytracer",
        WIDTH,
        HEIGHT,
        WindowOptions {
            scale: Scale::X2,
            ..WindowOptions::default()
        },
    )
    .expect("No se pudo crear la ventana");

    let mut last_time = Instant::now();

    while window.is_open() && !window.is_key_down(Key::Escape) {
        let now = Instant::now();

        let dt = now.duration_since(last_time).as_secs_f32();

        last_time = now;

        for planet in &mut planets {
            planet.update(dt);
        }

        if window.is_key_down(Key::Left) {
            camera.orbit(0.03, 0.0);
        }

        if window.is_key_down(Key::Right) {
            camera.orbit(-0.03, 0.0);
        }

        if window.is_key_down(Key::Up) {
            camera.orbit(0.0, 0.03);
        }

        if window.is_key_down(Key::Down) {
            camera.orbit(0.0, -0.03);
        }

        if window.is_key_down(Key::W) {
            camera.zoom(-0.25);
        }

        if window.is_key_down(Key::S) {
            camera.zoom(0.25);
        }

        render(&mut framebuffer, &camera, &planets);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
