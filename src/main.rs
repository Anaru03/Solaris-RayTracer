mod camera;
mod cube;
mod framebuffer;
mod material;
mod planet;
mod ray;
mod sphere;
mod texture;
mod vector;

use camera::Camera;
use cube::Cube;
use framebuffer::{Framebuffer, rgb};
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};
use planet::{Planet, create_planets};
use sphere::Sphere;
use std::f32::consts::PI;
use std::time::Instant;
use texture::Texture;
use vector::Vec3;

const WIDTH: usize = 280;
const HEIGHT: usize = 210;

const UI_HEIGHT: usize = 40;
const SCENE_HEIGHT: usize = HEIGHT - UI_HEIGHT;

const MENU_ITEMS: usize = 9;

fn multiply_color(color: u32, brightness: f32) -> u32 {
    let r = ((color >> 16) & 255) as f32;

    let g = ((color >> 8) & 255) as f32;

    let b = (color & 255) as f32;

    rgb(
        (r * brightness).clamp(0.0, 255.0) as u32,
        (g * brightness).clamp(0.0, 255.0) as u32,
        (b * brightness).clamp(0.0, 255.0) as u32,
    )
}

fn shade_sphere(sphere: &Sphere, point: Vec3, base_color: u32, sun_position: Vec3) -> u32 {
    let normal = sphere.normal_at(point);

    let light_direction = (sun_position - point).normalize();

    let diffuse = normal.dot(&light_direction).max(0.0);

    let brightness = (0.12 + diffuse * 0.88).min(1.0);

    multiply_color(base_color, brightness)
}

fn shade_cube(cube: &Cube, point: Vec3, base_color: u32, sun_position: Vec3) -> u32 {
    let normal = cube.normal_at(point);

    let light_direction = (sun_position - point).normalize();

    let diffuse = normal.dot(&light_direction).max(0.0);

    let brightness = (0.18 + diffuse * 0.82).min(1.0);

    multiply_color(base_color, brightness)
}

fn sphere_texture_color(sphere: &Sphere, point: Vec3, texture: &Texture) -> u32 {
    let normal = sphere.normal_at(point);

    let u = 0.5 + normal.z.atan2(normal.x) / (2.0 * PI);

    let v = 0.5 - normal.y.asin() / PI;

    let sample = texture.sample(u, v);

    let r = ((sample >> 16) & 255) as f32;

    let g = ((sample >> 8) & 255) as f32;

    let b = (sample & 255) as f32;

    let luminance = (r + g + b) / (255.0 * 3.0);

    let factor = 0.75 + luminance * 0.35;

    rgb(
        (255.0 * factor).clamp(0.0, 255.0) as u32,
        (135.0 * factor).clamp(0.0, 255.0) as u32,
        (25.0 * factor).clamp(0.0, 255.0) as u32,
    )
}

fn target_camera_distance(index: usize) -> f32 {
    match index {
        0 => 6.0,
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
    if window.is_key_pressed(Key::Key0, KeyRepeat::No) {
        *selected = None;

        camera.target = Vec3::new(0.0, 0.0, 0.0);

        camera.distance = 22.0;

        return;
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

fn average_texture_color(texture: &Texture) -> u32 {
    let samples = [
        (0.15, 0.30),
        (0.35, 0.50),
        (0.55, 0.40),
        (0.75, 0.60),
        (0.90, 0.50),
    ];

    let mut r = 0u32;
    let mut g = 0u32;
    let mut b = 0u32;

    for (u, v) in samples {
        let color = texture.sample(u, v);

        r += (color >> 16) & 255;

        g += (color >> 8) & 255;

        b += color & 255;
    }

    let count = samples.len() as u32;

    rgb(r / count, g / count, b / count)
}

fn glyph(c: char) -> [&'static str; 5] {
    match c {
        'A' => ["010", "101", "111", "101", "101"],
        'E' => ["111", "100", "110", "100", "111"],
        'J' => ["111", "001", "001", "101", "010"],
        'M' => ["101", "111", "111", "101", "101"],
        'N' => ["101", "111", "111", "111", "101"],
        'P' => ["110", "101", "110", "100", "100"],
        'R' => ["110", "101", "110", "101", "101"],
        'S' => ["011", "100", "010", "001", "110"],
        'T' => ["111", "010", "010", "010", "010"],
        'U' => ["101", "101", "101", "101", "111"],
        'V' => ["101", "101", "101", "101", "010"],
        _ => ["000", "000", "000", "000", "000"],
    }
}

fn draw_char(framebuffer: &mut Framebuffer, x: usize, y: usize, character: char, color: u32) {
    let pattern = glyph(character);

    for (row, line) in pattern.iter().enumerate() {
        for (column, pixel) in line.chars().enumerate() {
            if pixel == '1' {
                framebuffer.set_pixel(x + column, y + row, WIDTH, color);
            }
        }
    }
}

fn draw_text(framebuffer: &mut Framebuffer, x: usize, y: usize, text: &str, color: u32) {
    let mut cursor = x;

    for character in text.chars() {
        draw_char(framebuffer, cursor, y, character, color);

        // 3 px de letra + 1 de separación.
        cursor += 4;
    }
}

fn draw_ui(framebuffer: &mut Framebuffer, textures: &[Texture], selected: Option<usize>) {
    draw_rect(
        framebuffer,
        0,
        SCENE_HEIGHT,
        WIDTH,
        UI_HEIGHT,
        rgb(8, 10, 20),
    );

    // Línea superior del panel.
    draw_rect(framebuffer, 0, SCENE_HEIGHT, WIDTH, 1, rgb(80, 100, 160));

    let names = [
        "SUN", "MER", "VEN", "EAR", "MAR", "JUP", "SAT", "URA", "NEP",
    ];

    let button_width = WIDTH / MENU_ITEMS;

    for index in 0..MENU_ITEMS {
        let x = index * button_width;

        let is_selected = selected == Some(index);

        let background = if is_selected {
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

        // Dejamos la parte inferior para texto.
        let planet_y = SCENE_HEIGHT + 15;

        let size = match index {
            0 => 5, // Sol
            5 => 6, // Júpiter
            6 => 5, // Saturno
            7 | 8 => 4,
            _ => 3,
        };

        let planet_color = average_texture_color(&textures[index]);

        draw_rect(
            framebuffer,
            cx.saturating_sub(size),
            planet_y.saturating_sub(size),
            size * 2,
            size * 2,
            planet_color,
        );

        // Tierra: detalle verde.
        if index == 3 {
            draw_rect(framebuffer, cx, planet_y - 2, 2, 3, rgb(45, 155, 70));
        }

        // Júpiter: banda.
        if index == 5 {
            draw_rect(
                framebuffer,
                cx.saturating_sub(size),
                planet_y - 1,
                size * 2,
                2,
                rgb(225, 190, 150),
            );
        }

        // Saturno: anillo.
        if index == 6 {
            draw_rect(
                framebuffer,
                cx.saturating_sub(size + 5),
                planet_y,
                (size + 5) * 2,
                1,
                rgb(225, 200, 140),
            );
        }

        // -------------------------
        // NOMBRE
        // -------------------------

        let name = names[index];

        // Cada letra ocupa 4 píxeles.
        // Tres letras = 11 píxeles reales.
        let text_width = name.len() * 4 - 1;

        let text_x = cx.saturating_sub(text_width / 2);

        let text_y = SCENE_HEIGHT + UI_HEIGHT - 10;

        let text_color = if is_selected {
            rgb(255, 255, 255)
        } else {
            rgb(165, 175, 195)
        };

        draw_text(framebuffer, text_x, text_y, name, text_color);
    }
}

fn render(
    framebuffer: &mut Framebuffer,
    camera: &Camera,
    planets: &[Planet],
    textures: &[Texture],
    moon_angle: f32,
    selected: Option<usize>,
) {
    let background = rgb(2, 3, 10);

    framebuffer.clear(background);

    draw_orbits(framebuffer, camera, planets);

    let sun_position = Vec3::new(0.0, 0.0, 0.0);

    let sun = Sphere::new(sun_position, 1.2);

    let moon_position = planets[2].moon_position(moon_angle);

    let moon = Sphere::new(moon_position, 0.14);

    // textures[0] = Sol
    // textures[1] = Mercurio
    // ...
    // textures[8] = Neptuno
    let planet_cubes: Vec<Vec<(Cube, u32)>> = planets
        .iter()
        .enumerate()
        .map(|(index, planet)| planet.voxel_cubes(&textures[index + 1]))
        .collect();

    for y in 0..SCENE_HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, SCENE_HEIGHT);

            let mut closest_t = f32::INFINITY;

            let mut color = framebuffer.buffer[y * WIDTH + x];

            // ==========================
            // SOL TEXTURIZADO
            // ==========================

            if let Some(t) = sun.intersect(&ray) {
                closest_t = t;

                let point = ray.at(t);

                color = sphere_texture_color(&sun, point, &textures[0]);
            }

            // ==========================
            // PLANETAS TEXTURIZADOS
            // ==========================

            for cubes in &planet_cubes {
                for (cube, texture_color) in cubes {
                    if let Some(t) = cube.intersect(&ray) {
                        if t < closest_t {
                            closest_t = t;

                            color = shade_cube(cube, ray.at(t), *texture_color, sun_position);
                        }
                    }
                }
            }

            // ==========================
            // LUNA
            // ==========================

            if let Some(t) = moon.intersect(&ray) {
                if t < closest_t {
                    color = shade_sphere(&moon, ray.at(t), rgb(175, 175, 165), sun_position);
                }
            }

            framebuffer.set_pixel(x, y, WIDTH, color);
        }
    }

    draw_ui(framebuffer, textures, selected);
}

fn main() {
    // ==========================
    // CARGA DE TEXTURAS
    // ==========================

    let texture_paths = [
        "assets/textures/sun.bmp",
        "assets/textures/mercury.bmp",
        "assets/textures/venus.bmp",
        "assets/textures/earth.bmp",
        "assets/textures/mars.bmp",
        "assets/textures/jupiter.bmp",
        "assets/textures/saturn.bmp",
        "assets/textures/uranus.bmp",
        "assets/textures/neptune.bmp",
    ];

    let textures: Vec<Texture> = texture_paths
        .iter()
        .map(|path| {
            Texture::load_bmp(path)
                .unwrap_or_else(|error| panic!("Error cargando {}: {}", path, error))
        })
        .collect();

    println!("{} texturas cargadas correctamente.", textures.len());

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

        render(
            &mut framebuffer,
            &camera,
            &planets,
            &textures,
            moon_angle,
            selected,
        );

        window
            .update_with_buffer(&framebuffer.buffer, WIDTH, HEIGHT)
            .expect("No se pudo actualizar la ventana");
    }
}
