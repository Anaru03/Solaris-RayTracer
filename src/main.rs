mod camera;
mod cube;
mod framebuffer;
mod planet;
mod ray;
mod sphere;
mod vector;

use camera::Camera;
use cube::Cube;
use framebuffer::{Framebuffer, rgb};
use minifb::{Key, KeyRepeat, Scale, Window, WindowOptions};
use planet::{Planet, create_planets};
use sphere::Sphere;
use std::time::Instant;
use vector::Vec3;

const WIDTH: usize = 320;
const HEIGHT: usize = 240;
const UI_HEIGHT: usize = 45;
const SCENE_HEIGHT: usize = HEIGHT - UI_HEIGHT;

fn shade_sphere(sphere: &Sphere, point: Vec3, base_color: u32, sun_position: Vec3) -> u32 {
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

fn shade_cube(cube: &Cube, point: Vec3, base_color: u32, sun_position: Vec3) -> u32 {
    let normal = cube.normal_at(point);

    let light_direction = (sun_position - point).normalize();

    let diffuse = normal.dot(&light_direction).max(0.0);

    let brightness = (0.18 + diffuse * 0.82).min(1.0);

    let r = ((base_color >> 16) & 255) as f32;
    let g = ((base_color >> 8) & 255) as f32;
    let b = (base_color & 255) as f32;

    rgb(
        (r * brightness) as u32,
        (g * brightness) as u32,
        (b * brightness) as u32,
    )
}

fn planet_camera_distance(index: usize) -> f32 {
    match index {
        0 => 1.8,
        1 => 2.1,
        2 => 2.4,
        3 => 2.0,
        4 => 3.8,
        5 => 3.5,
        6 => 2.8,
        7 => 2.8,
        _ => 22.0,
    }
}

fn handle_planet_selection(
    window: &Window,
    selected_planet: &mut Option<usize>,
    camera: &mut Camera,
) {
    if window.is_key_pressed(Key::Key0, KeyRepeat::No) {
        *selected_planet = None;

        camera.target = Vec3::new(0.0, 0.0, 0.0);

        camera.distance = 22.0;
    }

    let keys = [
        Key::Key1,
        Key::Key2,
        Key::Key3,
        Key::Key4,
        Key::Key5,
        Key::Key6,
        Key::Key7,
        Key::Key8,
    ];

    for (index, key) in keys.iter().enumerate() {
        if window.is_key_pressed(*key, KeyRepeat::No) {
            *selected_planet = Some(index);

            camera.distance = planet_camera_distance(index);
        }
    }
}

fn draw_rect(
    framebuffer: &mut Framebuffer,
    x: usize,
    y: usize,
    width: usize,
    height: usize,
    color: u32,
) {
    for py in y..(y + height).min(HEIGHT) {
        for px in x..(x + width).min(WIDTH) {
            framebuffer.set_pixel(px, py, WIDTH, color);
        }
    }
}

fn draw_ui(framebuffer: &mut Framebuffer, planets: &[Planet], selected_planet: Option<usize>) {
    draw_rect(
        framebuffer,
        0,
        SCENE_HEIGHT,
        WIDTH,
        UI_HEIGHT,
        rgb(10, 12, 24),
    );

    draw_rect(framebuffer, 0, SCENE_HEIGHT, WIDTH, 1, rgb(80, 100, 150));

    let button_width = WIDTH / 8;

    for (index, planet) in planets.iter().enumerate() {
        let x = index * button_width;

        let background = if selected_planet == Some(index) {
            rgb(55, 65, 95)
        } else {
            rgb(20, 24, 40)
        };

        draw_rect(
            framebuffer,
            x + 2,
            SCENE_HEIGHT + 4,
            button_width - 4,
            UI_HEIGHT - 8,
            background,
        );

        let center_x = x + button_width / 2;
        let center_y = SCENE_HEIGHT + 18;

        let size = match index {
            4 => 6,
            5 => 5,
            _ => 4,
        };

        draw_rect(
            framebuffer,
            center_x.saturating_sub(size),
            center_y.saturating_sub(size),
            size * 2,
            size * 2,
            planet.color,
        );
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    planets: &[Planet],
    moon_angle: f32,
    selected_planet: Option<usize>,
) {
    let background = rgb(2, 3, 10);

    framebuffer.clear(background);

    let sun_position = Vec3::new(0.0, 0.0, 0.0);
    let sun = Sphere::new(sun_position, 1.2);

    let earth = &planets[2];
    let moon_position = earth.moon_position(moon_angle);
    let moon = Sphere::new(moon_position, 0.14);

    let planet_cubes: Vec<Vec<Cube>> = planets.iter().map(|planet| planet.voxel_cubes()).collect();

    for y in 0..SCENE_HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, SCENE_HEIGHT);

            let mut closest_t = f32::INFINITY;
            let mut color = background;

            if let Some(t) = sun.intersect(&ray) {
                closest_t = t;
                color = rgb(255, 190, 40);
            }

            for (planet_index, cubes) in planet_cubes.iter().enumerate() {
                for cube in cubes {
                    if let Some(t) = cube.intersect(&ray) {
                        if t < closest_t {
                            closest_t = t;

                            let point = ray.at(t);

                            color =
                                shade_cube(cube, point, planets[planet_index].color, sun_position);
                        }
                    }
                }
            }

            if let Some(t) = moon.intersect(&ray) {
                if t < closest_t {
                    let point = ray.at(t);

                    color = shade_sphere(&moon, point, rgb(175, 175, 165), sun_position);
                }
            }

            framebuffer.set_pixel(x, y, WIDTH, color);
        }
    }

    draw_ui(framebuffer, planets, selected_planet);
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut planets = create_planets();

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 22.0);

    let mut selected_planet: Option<usize> = None;

    let mut moon_angle = 0.0_f32;

    let mut window = Window::new(
        "SOLAR SYSTEM",
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

        let dt = now.duration_since(last_time).as_secs_f32().min(0.05);

        last_time = now;

        for planet in &mut planets {
            planet.update(dt);
        }

        moon_angle += 2.5 * dt;

        handle_planet_selection(&window, &mut selected_planet, &mut camera);

        if let Some(index) = selected_planet {
            camera.target = planets[index].position;
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
            camera.zoom(-0.15);
        }

        if window.is_key_down(Key::S) {
            camera.zoom(0.15);
        }

        let title = match selected_planet {
            Some(index) => {
                format!(
                    "SOLAR SYSTEM | {} | 0: Sistema | Flechas: Orbitar | W/S: Zoom",
                    planets[index].name
                )
            }

            None => {
                "SOLAR SYSTEM | 1-8: Seleccionar planeta | Flechas: Orbitar | W/S: Zoom".to_string()
            }
        };

        window.set_title(&title);

        render(
            &mut framebuffer,
            &camera,
            &planets,
            moon_angle,
            selected_planet,
        );

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
