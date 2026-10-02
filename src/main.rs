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
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};
use planet::{Planet, create_planets};
use sphere::Sphere;
use std::time::Instant;
use vector::Vec3;

const WIDTH: usize = 280;
const HEIGHT: usize = 210;

const UI_HEIGHT: usize = 40;
const SCENE_HEIGHT: usize = HEIGHT - UI_HEIGHT;

// Sol + 8 planetas
const MENU_ITEMS: usize = 9;

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

// 0 = Sol
// 1 = Mercurio
// 2 = Venus
// ...
// 8 = Neptuno
fn target_camera_distance(index: usize) -> f32 {
    match index {
        0 => 6.0, // Sol
        1 => 1.8,
        2 => 2.1,
        3 => 2.4,
        4 => 2.0,
        5 => 3.8,
        6 => 3.5,
        7 => 2.8,
        8 => 2.8,
        _ => 22.0,
    }
}

fn select_target(index: usize, selected: &mut Option<usize>, camera: &mut Camera) {
    *selected = Some(index);

    camera.distance = target_camera_distance(index);
}

fn handle_keyboard_selection(window: &Window, selected: &mut Option<usize>, camera: &mut Camera) {
    // 0 siempre vuelve a la vista completa.
    if window.is_key_pressed(Key::Key0, KeyRepeat::No) {
        *selected = None;

        camera.target = Vec3::new(0.0, 0.0, 0.0);

        camera.distance = 22.0;

        return;
    }

    // 1 = Sol
    // 2 = Mercurio
    // ...
    // 9 = Neptuno
    let keys = [
        Key::Key1,
        Key::Key2,
        Key::Key3,
        Key::Key4,
        Key::Key5,
        Key::Key6,
        Key::Key7,
        Key::Key8,
        Key::Key9,
    ];

    for (index, key) in keys.iter().enumerate() {
        if window.is_key_pressed(*key, KeyRepeat::No) {
            select_target(index, selected, camera);
        }
    }
}

fn handle_mouse_selection(
    window: &Window,
    selected: &mut Option<usize>,
    camera: &mut Camera,
    was_mouse_down: &mut bool,
) {
    let mouse_down = window.get_mouse_down(MouseButton::Left);

    if mouse_down && !*was_mouse_down {
        if let Some((mouse_x, mouse_y)) = window.get_mouse_pos(MouseMode::Clamp) {
            if mouse_y >= SCENE_HEIGHT as f32 {
                let button_width = WIDTH as f32 / MENU_ITEMS as f32;

                let index = (mouse_x / button_width).floor() as usize;

                if index < MENU_ITEMS {
                    select_target(index, selected, camera);
                }
            }
        }
    }

    *was_mouse_down = mouse_down;
}

fn draw_pixel(framebuffer: &mut Framebuffer, x: i32, y: i32, color: u32) {
    if x >= 0 && x < WIDTH as i32 && y >= 0 && y < SCENE_HEIGHT as i32 {
        framebuffer.set_pixel(x as usize, y as usize, WIDTH, color);
    }
}

fn draw_orbits(framebuffer: &mut Framebuffer, camera: &Camera, planets: &[Planet]) {
    // 90 puntos es suficiente visualmente
    // y sigue siendo muy barato.
    let segments = 90;

    for planet in planets {
        for i in 0..segments {
            let angle = i as f32 / segments as f32 * std::f32::consts::TAU;

            let point = Vec3::new(
                planet.orbit_radius * angle.cos(),
                -0.08,
                planet.orbit_radius * angle.sin(),
            );

            if let Some((x, y)) = camera.project(point, WIDTH, SCENE_HEIGHT) {
                draw_pixel(framebuffer, x, y, rgb(42, 52, 82));
            }
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

fn menu_color(index: usize, planets: &[Planet]) -> u32 {
    if index == 0 {
        // Sol
        rgb(255, 190, 40)
    } else {
        planets[index - 1].color
    }
}

fn draw_ui(framebuffer: &mut Framebuffer, planets: &[Planet], selected: Option<usize>) {
    draw_rect(
        framebuffer,
        0,
        SCENE_HEIGHT,
        WIDTH,
        UI_HEIGHT,
        rgb(8, 10, 20),
    );

    draw_rect(framebuffer, 0, SCENE_HEIGHT, WIDTH, 1, rgb(80, 100, 160));

    let button_width = WIDTH / MENU_ITEMS;

    for index in 0..MENU_ITEMS {
        let x = index * button_width;

        let background = if selected == Some(index) {
            rgb(55, 65, 100)
        } else {
            rgb(18, 22, 38)
        };

        draw_rect(
            framebuffer,
            x + 1,
            SCENE_HEIGHT + 3,
            button_width - 2,
            UI_HEIGHT - 6,
            background,
        );

        let cx = x + button_width / 2;

        let cy = SCENE_HEIGHT + UI_HEIGHT / 2;

        let size = match index {
            0 => 6, // Sol
            5 => 7, // Júpiter
            6 => 6, // Saturno
            7 | 8 => 5,
            _ => 4,
        };

        draw_rect(
            framebuffer,
            cx.saturating_sub(size),
            cy.saturating_sub(size),
            size * 2,
            size * 2,
            menu_color(index, planets),
        );

        // Tierra
        if index == 3 {
            draw_rect(framebuffer, cx - 1, cy - 2, 3, 3, rgb(45, 155, 70));
        }

        // Júpiter
        if index == 5 {
            draw_rect(
                framebuffer,
                cx - size,
                cy - 2,
                size * 2,
                2,
                rgb(235, 210, 175),
            );
        }

        // Saturno
        if index == 6 {
            draw_rect(
                framebuffer,
                cx.saturating_sub(size + 5),
                cy,
                (size + 5) * 2,
                2,
                rgb(225, 200, 140),
            );
        }
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    planets: &[Planet],
    moon_angle: f32,
    selected: Option<usize>,
) {
    let background = rgb(2, 3, 10);

    framebuffer.clear(background);

    /*
     * IMPORTANTE:
     *
     * Las órbitas se dibujan PRIMERO.
     *
     * Después el raytracing dibuja
     * Sol/planetas encima de ellas.
     *
     * Así ya no atraviesan visualmente
     * los cuerpos celestes.
     */
    draw_orbits(framebuffer, camera, planets);

    let sun_position = Vec3::new(0.0, 0.0, 0.0);

    let sun = Sphere::new(sun_position, 1.2);

    let moon_position = planets[2].moon_position(moon_angle);

    let moon = Sphere::new(moon_position, 0.14);

    /*
     * Los voxeles se generan UNA vez
     * por planeta por frame.
     */
    let planet_cubes: Vec<Vec<(Cube, u32)>> =
        planets.iter().map(|planet| planet.voxel_cubes()).collect();

    for y in 0..SCENE_HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, SCENE_HEIGHT);

            let mut closest_t = f32::INFINITY;

            /*
             * Conservamos el píxel de fondo.
             *
             * Si aquí había una órbita y
             * ningún objeto la tapa,
             * seguirá siendo visible.
             */
            let mut color = framebuffer.buffer[y * WIDTH + x];

            // SOL
            if let Some(t) = sun.intersect(&ray) {
                closest_t = t;

                color = rgb(255, 190, 40);
            }

            // PLANETAS VOXEL
            for cubes in &planet_cubes {
                for (cube, cube_color) in cubes {
                    if let Some(t) = cube.intersect(&ray) {
                        if t < closest_t {
                            closest_t = t;

                            color = shade_cube(cube, ray.at(t), *cube_color, sun_position);
                        }
                    }
                }
            }

            // LUNA
            if let Some(t) = moon.intersect(&ray) {
                if t < closest_t {
                    color = shade_sphere(&moon, ray.at(t), rgb(175, 175, 165), sun_position);
                }
            }

            framebuffer.set_pixel(x, y, WIDTH, color);
        }
    }

    draw_ui(framebuffer, planets, selected);
}

fn main() {
    let mut framebuffer = Framebuffer::new(WIDTH, HEIGHT);

    let mut planets = create_planets();

    let mut camera = Camera::new(Vec3::new(0.0, 0.0, 0.0), 22.0);

    let mut selected: Option<usize> = None;

    let mut moon_angle = 0.0_f32;

    let mut was_mouse_down = false;

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

        handle_keyboard_selection(&window, &mut selected, &mut camera);

        handle_mouse_selection(&window, &mut selected, &mut camera, &mut was_mouse_down);

        /*
         * selected:
         *
         * None    = sistema completo
         * Some(0) = Sol
         * Some(1) = Mercurio
         * ...
         * Some(8) = Neptuno
         */
        if let Some(index) = selected {
            if index == 0 {
                camera.target = Vec3::new(0.0, 0.0, 0.0);
            } else {
                camera.target = planets[index - 1].position;
            }
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
            camera.zoom(-0.12);
        }

        if window.is_key_down(Key::S) {
            camera.zoom(0.12);
        }

        let title = match selected {
            Some(0) => "SOLAR SYSTEM | Sun | 0: Sistema | W/S: Zoom".to_string(),

            Some(index) => {
                format!(
                    "SOLAR SYSTEM | {} | 0: Sistema | W/S: Zoom",
                    planets[index - 1].name
                )
            }

            None => "SOLAR SYSTEM | Click o 1-9 | 0: Sistema | W/S: Zoom".to_string(),
        };

        window.set_title(&title);

        render(&mut framebuffer, &camera, &planets, moon_angle, selected);

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
