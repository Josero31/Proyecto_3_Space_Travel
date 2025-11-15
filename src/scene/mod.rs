use crate::math::{Vec3, Mat4};

#[derive(Debug, Clone)]
pub struct CelestialBody {
    pub name: String,
    pub position: Vec3,
    pub velocity: Vec3,
    pub radius: f64,
    pub mass: f64,
    pub color: u32,
    pub rotation_angle: f64,
    pub rotation_speed: f64, // radians per second
    pub orbital_radius: f64,
    pub orbital_speed: f64, // radians per second
    pub body_type: BodyType,
    pub tilt: f64, // axial tilt in radians
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BodyType {
    Star,
    Planet,
    Moon,
}

impl CelestialBody {
    pub fn new_star(name: &str, position: Vec3, radius: f64, color: u32) -> Self {
        CelestialBody {
            name: name.to_string(),
            position,
            velocity: Vec3::zero(),
            radius,
            mass: 1e30,
            color,
            rotation_angle: 0.0,
            rotation_speed: 0.5,
            orbital_radius: 0.0,
            orbital_speed: 0.0,
            body_type: BodyType::Star,
            tilt: 0.0,
        }
    }

    pub fn new_planet(
        name: &str,
        orbital_radius: f64,
        orbital_speed: f64,
        radius: f64,
        color: u32,
    ) -> Self {
        CelestialBody {
            name: name.to_string(),
            position: Vec3::new(orbital_radius, 0.0, 0.0),
            velocity: Vec3::zero(),
            radius,
            mass: 1e24,
            color,
            rotation_angle: 0.0,
            rotation_speed: 1.0,
            orbital_radius,
            orbital_speed,
            body_type: BodyType::Planet,
            tilt: 0.0,
        }
    }

    pub fn new_moon(
        name: &str,
        orbital_radius: f64,
        orbital_speed: f64,
        radius: f64,
        color: u32,
    ) -> Self {
        CelestialBody {
            name: name.to_string(),
            position: Vec3::new(orbital_radius, 0.0, 0.0),
            velocity: Vec3::zero(),
            radius,
            mass: 1e20,
            color,
            rotation_angle: 0.0,
            rotation_speed: 0.5,
            orbital_radius,
            orbital_speed,
            body_type: BodyType::Moon,
            tilt: 0.0,
        }
    }

    pub fn update(&mut self, dt: f64, parent_pos: Vec3) {
        // Update rotation
        self.rotation_angle += self.rotation_speed * dt;
        self.rotation_angle = self.rotation_angle % (std::f64::consts::PI * 2.0);

        // Update orbital position if this body orbits something
        if self.orbital_radius > 0.0 {
            let angle = self.orbital_speed * self.get_age();
            let x = parent_pos.x + self.orbital_radius * angle.cos();
            let y = parent_pos.y;
            let z = parent_pos.z + self.orbital_radius * angle.sin();
            self.position = Vec3::new(x, y, z);
        }
    }

    pub fn get_age(&self) -> f64 {
        // This would normally be tracked separately, for now return rotation angle
        self.rotation_angle / self.rotation_speed
    }

    pub fn get_transform_matrix(&self) -> Mat4 {
        let translation = Mat4::translation(self.position.x, self.position.y, self.position.z);
        
        let rotation_y = Mat4::rotation_y(self.rotation_angle);
        let rotation_x = Mat4::rotation_x(self.tilt);
        
        let rotation = rotation_y.multiply(&rotation_x);
        
        translation.multiply(&rotation)
    }

    pub fn contains_point(&self, point: &Vec3) -> bool {
        let distance = self.position.distance_to(point);
        distance <= self.radius
    }
}

// Helper to generate color from HSV
pub fn hsv_to_rgb(h: f64, s: f64, v: f64) -> u32 {
    let c = v * s;
    let x = c * (1.0 - ((h / 60.0) % 2.0 - 1.0).abs());
    let m = v - c;

    let (r, g, b) = if h < 60.0 {
        (c, x, 0.0)
    } else if h < 120.0 {
        (x, c, 0.0)
    } else if h < 180.0 {
        (0.0, c, x)
    } else if h < 240.0 {
        (0.0, x, c)
    } else if h < 300.0 {
        (x, 0.0, c)
    } else {
        (c, 0.0, x)
    };

    let r = ((r + m) * 255.0) as u32;
    let g = ((g + m) * 255.0) as u32;
    let b = ((b + m) * 255.0) as u32;
    let a = 255u32;

    (r << 24) | (g << 16) | (b << 8) | a
}
