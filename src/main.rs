use macroquad::prelude::*;

struct Planet {
    name: &'static str,
    x: f32,
    y: f32,
    radius: f32,
    color: Color,
    orbit_radius: f32,
    angle: f32,
    speed: f32,
    ring_color: Option<Color>,
    has_surface_details: bool,
}

impl Planet {
    fn new(name: &'static str, orbit_radius: f32, speed: f32, radius: f32, color: Color) -> Self {
        Planet {
            name,
            x: orbit_radius,
            y: 0.0,
            radius,
            color,
            orbit_radius,
            angle: 0.0,
            speed,
            ring_color: None,
            has_surface_details: false,
        }
    }

    fn with_rings(mut self, ring_color: Color) -> Self {
        self.ring_color = Some(ring_color);
        self
    }

    fn with_details(mut self) -> Self {
        self.has_surface_details = true;
        self
    }

    fn update(&mut self, dt: f32) {
        self.angle += self.speed * dt;
        self.x = self.orbit_radius * self.angle.cos();
        self.y = self.orbit_radius * self.angle.sin();
    }

    fn draw(&self, center_x: f32, center_y: f32) {
        let px = center_x + self.x;
        let py = center_y + self.y;

        // Dibujar órbita (línea tenue)
        draw_circle(center_x, center_y, self.orbit_radius, Color::new(0.3, 0.3, 0.3, 0.3));

        // Dibujar anillos si los tiene
        if let Some(ring_color) = self.ring_color {
            draw_circle_lines(px, py, self.radius * 1.8, 2.0, ring_color);
        }

        // Dibujar planeta principal
        draw_circle(px, py, self.radius, self.color);

        // Detalles de superficie
        if self.has_surface_details {
            match self.name {
                "Tierra" => {
                    // Continentes verdes en la Tierra
                    draw_circle(px - self.radius * 0.3, py - self.radius * 0.2, self.radius * 0.3, Color::new(0.0, 0.6, 0.0, 1.0));
                    draw_circle(px + self.radius * 0.4, py + self.radius * 0.1, self.radius * 0.25, Color::new(0.0, 0.6, 0.0, 1.0));
                }
                "Júpiter" => {
                    // Bandas en Júpiter
                    draw_line(px - self.radius, py - self.radius * 0.5, px + self.radius, py - self.radius * 0.5, 2.0, Color::new(0.8, 0.5, 0.0, 1.0));
                    draw_line(px - self.radius, py, px + self.radius, py, 2.0, Color::new(0.8, 0.5, 0.0, 1.0));
                    draw_line(px - self.radius, py + self.radius * 0.5, px + self.radius, py + self.radius * 0.5, 2.0, Color::new(0.8, 0.5, 0.0, 1.0));
                }
                "Marte" => {
                    // Cráteres en Marte
                    draw_circle(px - self.radius * 0.3, py - self.radius * 0.2, self.radius * 0.15, Color::new(0.7, 0.2, 0.0, 1.0));
                    draw_circle(px + self.radius * 0.2, py + self.radius * 0.3, self.radius * 0.12, Color::new(0.7, 0.2, 0.0, 1.0));
                }
                _ => {}
            }
        }

        // Etiqueta del planeta
        draw_text(self.name, px - self.radius, py - self.radius - 15.0, 14.0, WHITE);
    }
}

#[macroquad::main("Sistema Solar")]
async fn main() {
    let mut planets = vec![
        Planet::new("Mercurio", 80.0, 0.5, 5.0, Color::new(0.7, 0.7, 0.7, 1.0)),
        Planet::new("Venus", 120.0, 0.3, 8.0, Color::new(1.0, 0.9, 0.2, 1.0)),
        Planet::new("Tierra", 160.0, 0.2, 9.0, Color::new(0.1, 0.5, 1.0, 1.0)).with_details(),
        Planet::new("Marte", 200.0, 0.15, 6.0, Color::new(1.0, 0.3, 0.1, 1.0)).with_details(),
        Planet::new("Júpiter", 260.0, 0.08, 20.0, Color::new(1.0, 0.6, 0.2, 1.0)).with_details(),
        Planet::new("Saturno", 320.0, 0.05, 15.0, Color::new(1.0, 0.8, 0.3, 1.0)).with_rings(Color::new(0.9, 0.8, 0.5, 0.7)),
    ];

    let sun_color = Color::new(1.0, 0.8, 0.0, 1.0);

    loop {
        let dt = get_frame_time();

        // Clear screen
        clear_background(Color::new(0.01, 0.01, 0.02, 1.0));

        let center_x = screen_width() / 2.0;
        let center_y = screen_height() / 2.0;

        // Draw sun with glow effect
        draw_circle(center_x, center_y, 14.0, Color::new(1.0, 0.7, 0.0, 0.5));
        draw_circle(center_x, center_y, 12.0, sun_color);

        // Update and draw planets
        for planet in &mut planets {
            planet.update(dt);
            planet.draw(center_x, center_y);
        }

        // Draw title
        draw_text("SISTEMA SOLAR", 10.0, 20.0, 24.0, WHITE);
        
        // Draw FPS
        draw_text(&format!("FPS: {:.0}", get_fps()), 10.0, 50.0, 16.0, Color::new(0.5, 0.5, 0.5, 1.0));

        // Draw instructions
        draw_text("Presiona ESC para cerrar", screen_width() - 250.0, 20.0, 16.0, Color::new(0.5, 0.5, 0.5, 1.0));

        // Exit with ESC
        if is_key_pressed(KeyCode::Escape) {
            break;
        }

        next_frame().await;
    }
}
