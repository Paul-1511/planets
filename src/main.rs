mod math;
mod renderer;
mod shader;

use minifb::{Key, Window, WindowOptions};
use renderer::{Scene, WIDTH, HEIGHT};

fn main() {
    let mut window = Window::new(
        "Sistema Solar 3D",
        WIDTH,
        HEIGHT,
        WindowOptions {
            resize: true,
            scale: minifb::Scale::X1,
            ..WindowOptions::default()
        },
    )
    .unwrap();


    let mut scene = Scene::new();
    let mut time = 0.0;

    while window.is_open() && !window.is_key_down(Key::Escape) {
        // Camera controls: left/right rotate camera around sun, up/down zoom
        let mut moving = false;
        if window.is_key_down(Key::Left) {
            scene.camera_angle -= 0.08; // rotate left (faster responsiveness)
            moving = true;
        }
        if window.is_key_down(Key::Right) {
            scene.camera_angle += 0.08; // rotate right
            moving = true;
        }
        if window.is_key_down(Key::Up) {
            scene.camera_radius = (scene.camera_radius - 0.6).max(5.0); // zoom in clamp
            moving = true;
        }
        if window.is_key_down(Key::Down) {
            scene.camera_radius = (scene.camera_radius + 0.6).min(120.0); // zoom out clamp
            moving = true;
        }
        scene.moving = moving;

        let buffer = scene.render(time);
        window.update_with_buffer(&buffer, WIDTH, HEIGHT).unwrap();
        time += 0.016; // Aproximadamente 60 FPS
    }
}