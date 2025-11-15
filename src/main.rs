use macroquad::prelude::*;

struct Planet {
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    orbit_radius: f32,
    angle: f32,
    speed: f32,
}

impl Planet {
    fn new(orbit_radius: f32, speed: f32, radius: f32, color: Color) -> Self {
        Planet {
            x: orbit_radius,
            y: 0.0,
            radius,
            color,
            orbit_radius,
            angle: 0.0,
            speed,
        }
    }

    fn update(&mut self, dt: f32) {
        self.angle += self.speed * dt;
        self.x = self.orbit_radius * self.angle.cos();
        self.y = self.orbit_radius * self.angle.sin();
    }
}

#[macroquad::main("Sistema Solar")]
async fn main() {
    let mut planets = vec![
        Planet::new(80.0, 0.5, 5.0, Color::new(0.8, 0.4, 0.2, 1.0)),
        Planet::new(120.0, 0.3, 8.0, Color::new(1.0, 0.9, 0.2, 1.0)),
        Planet::new(160.0, 0.2, 9.0, Color::new(0.1, 0.5, 1.0, 1.0)),
        Planet::new(200.0, 0.15, 6.0, Color::new(1.0, 0.3, 0.1, 1.0)),
        Planet::new(260.0, 0.08, 20.0, Color::new(1.0, 0.6, 0.2, 1.0)),
        Planet::new(320.0, 0.05, 15.0, Color::new(1.0, 0.8, 0.3, 1.0)),
    ];

    let sun_color = Color::new(1.0, 0.8, 0.0, 1.0);

    loop {
        let dt = get_frame_time();

        // Clear screen
        clear_background(BLACK);

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Draw sun
        draw_circle(center_x, center_y, 12.0, sun_color);

        // Update and draw planets
        for planet in &mut planets {
            planet.update(dt);

            let px = center_x + planet.x;
            let py = center_y + planet.y;

            draw_circle(px, py, planet.radius, planet.color);
        }

        // Draw FPS
        draw_text(&format!("FPS: {:.0}", get_fps()), 10.0, 20.0, 20.0, WHITE);

        // Exit with ESC
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
