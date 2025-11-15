use std::ops::{Add, Sub, Mul, Div};

#[derive(Debug, Clone, Copy)]
pub struct Vec3 {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

impl Vec3 {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vec3 { x, y, z }
    }

    pub fn zero() -> Self {
        Vec3 { x: 0.0, y: 0.0, z: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vec3::zero()
        } else {
            Vec3 {
                x: self.x / mag,
                y: self.y / mag,
                z: self.z / mag,
            }
        }
    }

    pub fn dot(&self, other: &Vec3) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z
    }

    pub fn cross(&self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
        }
    }

    pub fn distance_to(&self, other: &Vec3) -> f64 {
        (self - other).magnitude()
    }
}

impl Add for Vec3 {
    type Output = Vec3;

    fn add(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Add<&Vec3> for Vec3 {
    type Output = Vec3;

    fn add(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x + other.x,
            y: self.y + other.y,
            z: self.z + other.z,
        }
    }
}

impl Sub for Vec3 {
    type Output = Vec3;

    fn sub(self, other: Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Sub for &Vec3 {
    type Output = Vec3;

    fn sub(self, other: &Vec3) -> Vec3 {
        Vec3 {
            x: self.x - other.x,
            y: self.y - other.y,
            z: self.z - other.z,
        }
    }
}

impl Mul<f64> for Vec3 {
    type Output = Vec3;

    fn mul(self, scalar: f64) -> Vec3 {
        Vec3 {
            x: self.x * scalar,
            y: self.y * scalar,
            z: self.z * scalar,
        }
    }
}

impl Mul<Vec3> for f64 {
    type Output = Vec3;

    fn mul(self, vec: Vec3) -> Vec3 {
        Vec3 {
            x: vec.x * self,
            y: vec.y * self,
            z: vec.z * self,
        }
    }
}

impl Div<f64> for Vec3 {
    type Output = Vec3;

    fn div(self, scalar: f64) -> Vec3 {
        if scalar == 0.0 {
            panic!("Division by zero");
        }
        Vec3 {
            x: self.x / scalar,
            y: self.y / scalar,
            z: self.z / scalar,
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Vec2 { x, y }
    }

    pub fn zero() -> Self {
        Vec2 { x: 0.0, y: 0.0 }
    }

    pub fn magnitude(&self) -> f64 {
        (self.x * self.x + self.y * self.y).sqrt()
    }

    pub fn normalize(&self) -> Self {
        let mag = self.magnitude();
        if mag == 0.0 {
            Vec2::zero()
        } else {
            Vec2 {
                x: self.x / mag,
                y: self.y / mag,
            }
        }
    }
}

impl Add for Vec2 {
    type Output = Vec2;

    fn add(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl Sub for Vec2 {
    type Output = Vec2;

    fn sub(self, other: Vec2) -> Vec2 {
        Vec2 {
            x: self.x - other.x,
            y: self.y - other.y,
        }
    }
}

impl Mul<f64> for Vec2 {
    type Output = Vec2;

    fn mul(self, scalar: f64) -> Vec2 {
        Vec2 {
            x: self.x * scalar,
            y: self.y * scalar,
        }
    }
}

// Matrix 4x4 para transformaciones 3D
#[derive(Debug, Clone, Copy)]
pub struct Mat4 {
    pub data: [[f64; 4]; 4],
}

impl Mat4 {
    pub fn identity() -> Self {
        Mat4 {
            data: [
                [1.0, 0.0, 0.0, 0.0],
                [0.0, 1.0, 0.0, 0.0],
                [0.0, 0.0, 1.0, 0.0],
                [0.0, 0.0, 0.0, 1.0],
            ],
        }
    }

    pub fn translation(x: f64, y: f64, z: f64) -> Self {
        let mut mat = Mat4::identity();
        mat.data[0][3] = x;
        mat.data[1][3] = y;
        mat.data[2][3] = z;
        mat
    }

    pub fn scale(sx: f64, sy: f64, sz: f64) -> Self {
        let mut mat = Mat4::identity();
        mat.data[0][0] = sx;
        mat.data[1][1] = sy;
        mat.data[2][2] = sz;
        mat
    }

    pub fn rotation_x(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut mat = Mat4::identity();
        mat.data[1][1] = cos;
        mat.data[1][2] = -sin;
        mat.data[2][1] = sin;
        mat.data[2][2] = cos;
        mat
    }

    pub fn rotation_y(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut mat = Mat4::identity();
        mat.data[0][0] = cos;
        mat.data[0][2] = sin;
        mat.data[2][0] = -sin;
        mat.data[2][2] = cos;
        mat
    }

    pub fn rotation_z(angle: f64) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        let mut mat = Mat4::identity();
        mat.data[0][0] = cos;
        mat.data[0][1] = -sin;
        mat.data[1][0] = sin;
        mat.data[1][1] = cos;
        mat
    }

    pub fn multiply(&self, other: &Mat4) -> Self {
        let mut result = Mat4::identity();
        for i in 0..4 {
            for j in 0..4 {
                result.data[i][j] = 0.0;
                for k in 0..4 {
                    result.data[i][j] += self.data[i][k] * other.data[k][j];
                }
            }
        }
        result
    }

    pub fn transform_point(&self, point: &Vec3) -> Vec3 {
        let x = self.data[0][0] * point.x + self.data[0][1] * point.y + self.data[0][2] * point.z + self.data[0][3];
        let y = self.data[1][0] * point.x + self.data[1][1] * point.y + self.data[1][2] * point.z + self.data[1][3];
        let z = self.data[2][0] * point.x + self.data[2][1] * point.y + self.data[2][2] * point.z + self.data[2][3];
        let w = self.data[3][0] * point.x + self.data[3][1] * point.y + self.data[3][2] * point.z + self.data[3][3];

        if w != 0.0 {
            Vec3::new(x / w, y / w, z / w)
        } else {
            Vec3::new(x, y, z)
        }
    }
}

pub struct Quaternion {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Quaternion {
    pub fn identity() -> Self {
        Quaternion { x: 0.0, y: 0.0, z: 0.0, w: 1.0 }
    }

    pub fn from_axis_angle(axis: &Vec3, angle: f64) -> Self {
        let half_angle = angle / 2.0;
        let sin_half = half_angle.sin();
        let normalized = axis.normalize();
        
        Quaternion {
            x: normalized.x * sin_half,
            y: normalized.y * sin_half,
            z: normalized.z * sin_half,
            w: half_angle.cos(),
        }
    }

    pub fn to_matrix(&self) -> Mat4 {
        let mut mat = Mat4::identity();
        
        let x2 = self.x + self.x;
        let y2 = self.y + self.y;
        let z2 = self.z + self.z;
        
        let xx2 = self.x * x2;
        let xy2 = self.x * y2;
        let xz2 = self.x * z2;
        let yy2 = self.y * y2;
        let yz2 = self.y * z2;
        let zz2 = self.z * z2;
        let wx2 = self.w * x2;
        let wy2 = self.w * y2;
        let wz2 = self.w * z2;
        
        mat.data[0][0] = 1.0 - (yy2 + zz2);
        mat.data[0][1] = xy2 - wz2;
        mat.data[0][2] = xz2 + wy2;
        
        mat.data[1][0] = xy2 + wz2;
        mat.data[1][1] = 1.0 - (xx2 + zz2);
        mat.data[1][2] = yz2 - wx2;
        
        mat.data[2][0] = xz2 - wy2;
        mat.data[2][1] = yz2 + wx2;
        mat.data[2][2] = 1.0 - (xx2 + yy2);
        
        mat
    }
}
