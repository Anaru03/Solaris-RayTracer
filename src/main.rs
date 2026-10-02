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
use material::Material;
use minifb::{Key, KeyRepeat, MouseButton, MouseMode, Scale, Window, WindowOptions};
use planet::{Planet, create_planets};
use ray::Ray;
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
const EPSILON: f32 = 0.002;

fn multiply_color(color: u32, factor: f32) -> u32 {
    let r = ((color >> 16) & 255) as f32;
    let g = ((color >> 8) & 255) as f32;
    let b = (color & 255) as f32;

    rgb(
        (r * factor).clamp(0.0, 255.0) as u32,
        (g * factor).clamp(0.0, 255.0) as u32,
        (b * factor).clamp(0.0, 255.0) as u32,
    )
}

fn add_colors(a: u32, b: u32) -> u32 {
    let ar = (a >> 16) & 255;
    let ag = (a >> 8) & 255;
    let ab = a & 255;

    let br = (b >> 16) & 255;
    let bg = (b >> 8) & 255;
    let bb = b & 255;

    rgb((ar + br).min(255), (ag + bg).min(255), (ab + bb).min(255))
}

fn blend_colors(a: u32, b: u32, amount: f32) -> u32 {
    let amount = amount.clamp(0.0, 1.0);
    let inverse = 1.0 - amount;

    let ar = ((a >> 16) & 255) as f32;
    let ag = ((a >> 8) & 255) as f32;
    let ab = (a & 255) as f32;

    let br = ((b >> 16) & 255) as f32;
    let bg = ((b >> 8) & 255) as f32;
    let bb = (b & 255) as f32;

    rgb(
        (ar * inverse + br * amount).clamp(0.0, 255.0) as u32,
        (ag * inverse + bg * amount).clamp(0.0, 255.0) as u32,
        (ab * inverse + bb * amount).clamp(0.0, 255.0) as u32,
    )
}

fn hash3(x: i32, y: i32, z: i32) -> u32 {
    let mut n = x as u32;

    n = n.wrapping_mul(374761393);
    n = n.wrapping_add((y as u32).wrapping_mul(668265263));
    n = n.wrapping_add((z as u32).wrapping_mul(2246822519));

    n ^= n >> 13;
    n = n.wrapping_mul(1274126177);

    n ^ (n >> 16)
}

fn skybox_color(direction: Vec3) -> u32 {
    let d = direction.normalize();

    let vertical = ((d.y + 1.0) * 0.5).clamp(0.0, 1.0);

    let mut r = 2.0 + vertical * 3.0;
    let mut g = 4.0 + vertical * 5.0;
    let mut b = 12.0 + vertical * 14.0;

    let nebula_a = (d.x * 3.2 + d.z * 2.1).sin() * 0.5 + 0.5;

    let nebula_b = (d.y * 4.0 - d.z).cos() * 0.5 + 0.5;

    let nebula = nebula_a * nebula_b;

    if nebula > 0.70 {
        let strength = (nebula - 0.70) / 0.30;

        r += 8.0 * strength;
        g += 3.0 * strength;
        b += 18.0 * strength;
    }

    let sx = (d.x * 900.0).floor() as i32;

    let sy = (d.y * 900.0).floor() as i32;

    let sz = (d.z * 900.0).floor() as i32;

    let hash = hash3(sx, sy, sz);

    if hash % 997 < 3 {
        let brightness = 170 + (hash % 86);

        return rgb(brightness, brightness, (brightness + 15).min(255));
    }

    let hash_large = hash3(
        (d.x * 300.0).floor() as i32,
        (d.y * 300.0).floor() as i32,
        (d.z * 300.0).floor() as i32,
    );

    if hash_large % 4093 == 0 {
        return rgb(220, 230, 255);
    }

    rgb(
        r.clamp(0.0, 255.0) as u32,
        g.clamp(0.0, 255.0) as u32,
        b.clamp(0.0, 255.0) as u32,
    )
}

fn sphere_texture_color(sphere: &Sphere, point: Vec3, texture: &Texture) -> u32 {
    let normal = sphere.normal_at(point);

    let u = 0.5 + normal.z.atan2(normal.x) / (2.0 * PI);

    let v = 0.5 - normal.y.asin() / PI;

    texture.sample(u, v)
}

fn planet_texture_color(planet: &Planet, hit_point: Vec3, texture: &Texture) -> u32 {
    let local = hit_point - planet.position;

    let diameter = planet.radius * 2.45;

    let u = 0.5 + local.x / diameter;

    let v = 0.5 - local.y / diameter;

    let sampled = texture.sample(u, v);

    let r = (sampled >> 16) & 255;

    let g = (sampled >> 8) & 255;

    let b = sampled & 255;

    if r + g + b < 18 {
        planet.color
    } else {
        sampled
    }
}

fn shade_material(
    cube: &Cube,
    point: Vec3,
    texture_color: u32,
    material: Material,
    camera_position: Vec3,
    sun_position: Vec3,
) -> u32 {
    let normal = cube.normal_at(point);

    let light_direction = (sun_position - point).normalize();

    let view_direction = (camera_position - point).normalize();

    let diffuse = normal.dot(&light_direction).max(0.0);

    let ambient = 0.16;

    let diffuse_light = ambient + diffuse * material.albedo * 0.84;

    let diffuse_color = multiply_color(texture_color, diffuse_light);

    let light_dot_normal = light_direction.dot(&normal);

    let reflected_light = normal * (2.0 * light_dot_normal) - light_direction;

    let specular_angle = reflected_light.normalize().dot(&view_direction).max(0.0);

    let specular_strength = specular_angle.powf(24.0) * material.specular;

    let specular_color = multiply_color(rgb(255, 245, 220), specular_strength);

    add_colors(diffuse_color, specular_color)
}

fn shade_moon(sphere: &Sphere, point: Vec3, base_color: u32, sun_position: Vec3) -> u32 {
    let normal = sphere.normal_at(point);

    let light_direction = (sun_position - point).normalize();

    let diffuse = normal.dot(&light_direction).max(0.0);

    let brightness = 0.15 + diffuse * 0.85;

    multiply_color(base_color, brightness)
}

fn reflected_direction(incoming: Vec3, normal: Vec3) -> Vec3 {
    (incoming - normal * (2.0 * incoming.dot(&normal))).normalize()
}

fn refracted_direction(incoming: Vec3, normal: Vec3, refractive_index: f32) -> Option<Vec3> {
    let mut n = normal;

    let mut eta_i = 1.0;

    let mut eta_t = refractive_index.max(1.0001);

    let mut cos_i = incoming.dot(&n).clamp(-1.0, 1.0);

    if cos_i > 0.0 {
        std::mem::swap(&mut eta_i, &mut eta_t);

        n = n * -1.0;
    } else {
        cos_i = -cos_i;
    }

    let eta = eta_i / eta_t;

    let k = 1.0 - eta * eta * (1.0 - cos_i * cos_i);

    if k < 0.0 {
        None
    } else {
        Some((incoming * eta + n * (eta * cos_i - k.sqrt())).normalize())
    }
}

fn ray_hits_planet_bounds(ray: &Ray, planet: &Planet) -> bool {
    let oc = ray.origin - planet.position;

    let radius = if planet.name == "Saturn" {
        planet.radius * 2.55
    } else {
        planet.radius * 1.30
    };

    let a = ray.direction.dot(&ray.direction);

    let b = 2.0 * oc.dot(&ray.direction);

    let c = oc.dot(&oc) - radius * radius;

    let discriminant = b * b - 4.0 * a * c;

    discriminant >= 0.0
}

fn trace_secondary(
    origin: Vec3,
    direction: Vec3,
    planets: &[Planet],
    planet_cubes: &[Vec<Cube>],
    textures: &[Texture],
    sun: &Sphere,
) -> u32 {
    let ray = Ray::new(origin, direction);

    let mut closest_t = f32::INFINITY;

    let mut color = skybox_color(direction);

    if let Some(t) = sun.intersect(&ray) {
        if t > EPSILON && t < closest_t {
            closest_t = t;

            color = sphere_texture_color(sun, ray.at(t), &textures[0]);
        }
    }

    for (planet_index, cubes) in planet_cubes.iter().enumerate() {
        let planet = &planets[planet_index];

        if !ray_hits_planet_bounds(&ray, planet) {
            continue;
        }

        for cube in cubes {
            if let Some(t) = cube.intersect(&ray) {
                if t > EPSILON && t < closest_t {
                    closest_t = t;

                    let point = ray.at(t);

                    color = planet_texture_color(planet, point, &textures[planet_index + 1]);
                }
            }
        }
    }

    color
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

        cursor += 4;
    }
}

fn draw_texture_thumbnail(
    framebuffer: &mut Framebuffer,
    texture: &Texture,
    center_x: usize,
    center_y: usize,
    width: usize,
    height: usize,
) {
    let start_x = center_x.saturating_sub(width / 2);

    let start_y = center_y.saturating_sub(height / 2);

    for dy in 0..height {
        for dx in 0..width {
            let u = dx as f32 / width as f32;

            let v = dy as f32 / height as f32;

            let color = texture.sample(u, v);

            let r = (color >> 16) & 255;

            let g = (color >> 8) & 255;

            let b = color & 255;

            if r + g + b > 15 {
                let px = start_x + dx;

                let py = start_y + dy;

                if px < WIDTH && py < HEIGHT {
                    framebuffer.set_pixel(px, py, WIDTH, color);
                }
            }
        }
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

    draw_rect(framebuffer, 0, SCENE_HEIGHT, WIDTH, 1, rgb(80, 100, 160));

    let names = [
        "SUN", "MER", "VEN", "EAR", "MAR", "JUP", "SAT", "URA", "NEP",
    ];

    let button_width = WIDTH / MENU_ITEMS;

    for index in 0..MENU_ITEMS {
        let x = index * button_width;

        let selected_now = selected == Some(index);

        let background = if selected_now {
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

        let center_x = x + button_width / 2;

        let center_y = SCENE_HEIGHT + 14;

        let thumb_width = if index == 6 { 25 } else { 20 };

        draw_texture_thumbnail(
            framebuffer,
            &textures[index],
            center_x,
            center_y,
            thumb_width,
            20,
        );

        let name = names[index];

        let text_width = name.len() * 4 - 1;

        let text_x = center_x.saturating_sub(text_width / 2);

        let text_y = SCENE_HEIGHT + UI_HEIGHT - 9;

        let text_color = if selected_now {
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
    let sun_position = Vec3::new(0.0, 0.0, 0.0);

    let sun = Sphere::new(sun_position, 1.2);

    let moon_position = planets[2].moon_position(moon_angle);

    let moon = Sphere::new(moon_position, 0.14);

    let planet_cubes: Vec<Vec<Cube>> = planets.iter().map(|planet| planet.voxel_cubes()).collect();

    let camera_position = camera.position();

    for y in 0..SCENE_HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, SCENE_HEIGHT);

            framebuffer.set_pixel(x, y, WIDTH, skybox_color(ray.direction));
        }
    }

    draw_orbits(framebuffer, camera, planets);

    for y in 0..SCENE_HEIGHT {
        for x in 0..WIDTH {
            let ray = camera.get_ray(x, y, WIDTH, SCENE_HEIGHT);

            let mut closest_t = f32::INFINITY;

            let mut color = framebuffer.buffer[y * WIDTH + x];

            if let Some(t) = sun.intersect(&ray) {
                closest_t = t;

                color = sphere_texture_color(&sun, ray.at(t), &textures[0]);
            }

            for (planet_index, cubes) in planet_cubes.iter().enumerate() {
                let planet = &planets[planet_index];

                if !ray_hits_planet_bounds(&ray, planet) {
                    continue;
                }

                let texture = &textures[planet_index + 1];

                for cube in cubes {
                    if let Some(t) = cube.intersect(&ray) {
                        if t < closest_t {
                            closest_t = t;

                            let point = ray.at(t);

                            let texture_color = planet_texture_color(planet, point, texture);

                            let mut surface_color = shade_material(
                                cube,
                                point,
                                texture_color,
                                planet.material,
                                camera_position,
                                sun_position,
                            );

                            let normal = cube.normal_at(point);

                            let incoming = ray.direction.normalize();

                            if planet.material.reflectivity > 0.001 {
                                let reflection_direction = reflected_direction(incoming, normal);

                                let reflection_origin = point + normal * EPSILON;

                                let reflected_color = trace_secondary(
                                    reflection_origin,
                                    reflection_direction,
                                    planets,
                                    &planet_cubes,
                                    textures,
                                    &sun,
                                );

                                surface_color = blend_colors(
                                    surface_color,
                                    reflected_color,
                                    planet.material.reflectivity,
                                );
                            }

                            if planet.material.transparency > 0.001 {
                                if let Some(refraction_direction) = refracted_direction(
                                    incoming,
                                    normal,
                                    planet.material.refractive_index,
                                ) {
                                    let refraction_origin =
                                        point + refraction_direction * (EPSILON * 4.0);

                                    let refracted_color = trace_secondary(
                                        refraction_origin,
                                        refraction_direction,
                                        planets,
                                        &planet_cubes,
                                        textures,
                                        &sun,
                                    );

                                    surface_color = blend_colors(
                                        surface_color,
                                        refracted_color,
                                        planet.material.transparency,
                                    );
                                }
                            }

                            color = surface_color;
                        }
                    }
                }
            }

            if let Some(t) = moon.intersect(&ray) {
                if t < closest_t {
                    color = shade_moon(&moon, ray.at(t), rgb(180, 180, 175), sun_position);
                }
            }

            framebuffer.set_pixel(x, y, WIDTH, color);
        }
    }

    draw_ui(framebuffer, textures, selected);
}

fn main() {
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
