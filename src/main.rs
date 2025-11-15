mod math;
mod renderer;
mod scene;
mod camera;
mod physics;

use math::Vec3;
use renderer::SoftwareRenderer;
use scene::CelestialBody;
use camera::Camera;
use std::fs;

fn main() {
    // Create output directory
    let _ = fs::create_dir_all("output");

    let mut renderer = SoftwareRenderer::new();
    let mut solar_system = create_solar_system();
    let mut camera = Camera::new(Vec3::new(0.0, 100.0, 600.0), Vec3::new(0.0, 0.0, 0.0));
    camera.look_at(Vec3::new(0.0, 0.0, 0.0));

    println!("Space Travel - Solar System Simulator");
    println!("Rendering {} frames...", 300);
    
    const TOTAL_FRAMES: usize = 300;
    const DT: f64 = 0.033; // ~30 FPS

    for frame in 0..TOTAL_FRAMES {
        // Render
        renderer.clear(0x0a0e27ff);

        // Draw celestial bodies
        for body in &solar_system {
            draw_celestial_body(&mut renderer, body, &camera);
        }

        // Save frame
        if frame % 10 == 0 {
            let img = renderer.framebuffer.to_image();
            let filename = format!("output/frame_{:04}.png", frame);
            img.save(&filename).expect("Failed to save frame");
            println!("Frame {} saved", frame);
        }

        // Update solar system
        for body in &mut solar_system {
            body.update(DT, Vec3::new(0.0, 0.0, 0.0));
        }

        // Animate camera
        if frame < 100 {
            // Move around the sun
            let angle = (frame as f64 / 100.0) * std::f64::consts::PI * 2.0;
            let radius = 600.0;
            camera.position = Vec3::new(
                angle.cos() * radius,
                100.0,
                angle.sin() * radius,
            );
            camera.look_at(Vec3::new(0.0, 0.0, 0.0));
        } else if frame < 200 {
            // Zoom to Earth
            let progress = (frame - 100) as f64 / 100.0;
            let from = Vec3::new(600.0, 100.0, 0.0);
            let to = Vec3::new(400.0, 50.0, 100.0);
            camera.position = Vec3::new(
                from.x + (to.x - from.x) * progress,
                from.y + (to.y - from.y) * progress,
                from.z + (to.z - from.z) * progress,
            );
            camera.look_at(Vec3::new(400.0, 0.0, 0.0));
        } else {
            // Circle Earth
            let progress = (frame - 200) as f64 / 100.0;
            let angle = progress * std::f64::consts::PI * 2.0;
            camera.position = Vec3::new(
                400.0 + angle.cos() * 200.0,
                50.0 + 150.0 * (angle / std::f64::consts::PI * 2.0).sin().abs(),
                100.0 + angle.sin() * 200.0,
            );
            camera.look_at(Vec3::new(400.0, 0.0, 0.0));
        }
    }

    println!("Rendering complete! Check the 'output' directory.");
}

fn create_solar_system() -> Vec<CelestialBody> {
    vec![
        CelestialBody::new_star("Sun", Vec3::new(0.0, 0.0, 0.0), 50.0, 0xFFD700FF),
        CelestialBody::new_planet("Mercury", 150.0, 2.0, 15.0, 0x8C7853FF),
        CelestialBody::new_planet("Venus", 250.0, 1.5, 30.0, 0xFFC649FF),
        CelestialBody::new_planet("Earth", 400.0, 1.0, 35.0, 0x4169E1FF),
        CelestialBody::new_planet("Mars", 550.0, 0.8, 25.0, 0xE27B58FF),
        CelestialBody::new_planet("Jupiter", 900.0, 0.5, 80.0, 0xC88B3AFF),
        CelestialBody::new_planet("Saturn", 1200.0, 0.35, 70.0, 0xE5C55BFF),
    ]
}

fn draw_celestial_body(renderer: &mut SoftwareRenderer, body: &CelestialBody, camera: &Camera) {
    let relative_pos = body.position - camera.position;
    let distance = relative_pos.magnitude();

    if distance > 0.1 {
        let forward = camera.get_forward();
        let projected_distance = relative_pos.dot(&forward);

        if projected_distance > 0.1 {
            let right = camera.get_right();
            let screen_x = relative_pos.dot(&right);
            let screen_y = relative_pos.dot(&camera.up);

            let screen_pos = math::Vec2::new(
                640.0 + screen_x * 100.0 / projected_distance,
                360.0 - screen_y * 100.0 / projected_distance,
            );

            let screen_radius = body.radius * 100.0 / projected_distance;

            if screen_radius > 0.5 {
                renderer.draw_filled_circle(screen_pos, screen_radius, body.color);
            }
        }
    }
}
