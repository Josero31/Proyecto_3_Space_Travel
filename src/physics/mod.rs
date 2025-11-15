use crate::math::Vec3;

pub const G: f64 = 6.674e-11; // Gravitational constant

pub fn calculate_gravitational_force(mass1: f64, mass2: f64, distance: f64) -> f64 {
    if distance == 0.0 {
        0.0
    } else {
        G * mass1 * mass2 / (distance * distance)
    }
}

pub fn calculate_orbital_velocity(mass_central: f64, orbital_radius: f64) -> f64 {
    (G * mass_central / orbital_radius).sqrt()
}

pub fn calculate_escape_velocity(mass: f64, radius: f64) -> f64 {
    (2.0 * G * mass / radius).sqrt()
}

pub fn is_collision(pos1: &Vec3, radius1: f64, pos2: &Vec3, radius2: f64) -> bool {
    let distance = pos1.distance_to(pos2);
    distance <= radius1 + radius2
}
