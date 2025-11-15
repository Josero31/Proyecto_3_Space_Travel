use crate::math::{Vec3, Mat4};

pub struct Camera {
    pub position: Vec3,
    pub target: Vec3,
    pub up: Vec3,
    pub yaw: f64,
    pub pitch: f64,
    pub roll: f64,
    pub fov: f64,
    pub near_plane: f64,
    pub far_plane: f64,
}

impl Camera {
    pub fn new(position: Vec3, target: Vec3) -> Self {
        Camera {
            position,
            target,
            up: Vec3::new(0.0, 1.0, 0.0),
            yaw: 0.0,
            pitch: 0.0,
            roll: 0.0,
            fov: 45.0,
            near_plane: 0.1,
            far_plane: 10000.0,
        }
    }

    pub fn look_at(&mut self, target: Vec3) {
        self.target = target;
        
        // Calculate yaw and pitch from position and target
        let forward = self.target - self.position;
        
        self.yaw = forward.z.atan2(forward.x);
        self.pitch = (forward.y / forward.magnitude()).asin();
    }

    pub fn move_forward(&mut self, distance: f64) {
        let direction = self.get_forward();
        self.position = self.position + direction * distance;
        self.target = self.target + direction * distance;
    }

    pub fn move_right(&mut self, distance: f64) {
        let direction = self.get_right();
        self.position = self.position + direction * distance;
        self.target = self.target + direction * distance;
    }

    pub fn move_up(&mut self, distance: f64) {
        let direction = Vec3::new(0.0, 1.0, 0.0);
        self.position = self.position + direction * distance;
        self.target = self.target + direction * distance;
    }

    pub fn rotate_yaw(&mut self, angle: f64) {
        self.yaw += angle;
    }

    pub fn rotate_pitch(&mut self, angle: f64) {
        self.pitch += angle;
        // Clamp pitch to avoid gimbal lock
        if self.pitch > std::f64::consts::FRAC_PI_2 - 0.01 {
            self.pitch = std::f64::consts::FRAC_PI_2 - 0.01;
        }
        if self.pitch < -std::f64::consts::FRAC_PI_2 + 0.01 {
            self.pitch = -std::f64::consts::FRAC_PI_2 + 0.01;
        }
    }

    pub fn get_forward(&self) -> Vec3 {
        Vec3::new(
            self.yaw.cos() * self.pitch.cos(),
            self.pitch.sin(),
            self.yaw.sin() * self.pitch.cos(),
        )
        .normalize()
    }

    pub fn get_right(&self) -> Vec3 {
        let forward = self.get_forward();
        forward.cross(&self.up).normalize()
    }

    pub fn get_view_matrix(&self) -> Mat4 {
        let forward = self.get_forward();
        let right = self.get_right();
        let up = right.cross(&forward).normalize();

        let mut view = Mat4::identity();

        view.data[0][0] = right.x;
        view.data[0][1] = right.y;
        view.data[0][2] = right.z;

        view.data[1][0] = up.x;
        view.data[1][1] = up.y;
        view.data[1][2] = up.z;

        view.data[2][0] = -forward.x;
        view.data[2][1] = -forward.y;
        view.data[2][2] = -forward.z;

        view.data[0][3] = -right.dot(&self.position);
        view.data[1][3] = -up.dot(&self.position);
        view.data[2][3] = forward.dot(&self.position);

        view
    }

    pub fn get_projection_matrix(&self, width: usize, height: usize) -> Mat4 {
        let aspect = width as f64 / height as f64;
        let fov_rad = self.fov.to_radians();
        let f = 1.0 / (fov_rad / 2.0).tan();

        let mut proj = Mat4::identity();

        proj.data[0][0] = f / aspect;
        proj.data[1][1] = f;
        proj.data[2][2] = (self.far_plane + self.near_plane) / (self.near_plane - self.far_plane);
        proj.data[2][3] = (2.0 * self.far_plane * self.near_plane) / (self.near_plane - self.far_plane);
        proj.data[3][2] = -1.0;
        proj.data[3][3] = 0.0;

        proj
    }

    pub fn warp_to(&mut self, target: Vec3, distance: f64) {
        let direction = (target - self.position).normalize();
        self.position = target - direction * distance;
        self.target = target;
        self.look_at(target);
    }
}
