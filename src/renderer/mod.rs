use image::{ImageBuffer, Rgba};
use crate::math::{Vec3, Vec2};

const BUFFER_WIDTH: usize = 1280;
const BUFFER_HEIGHT: usize = 720;

pub struct Framebuffer {
    pub color_buffer: Vec<u32>,
    pub depth_buffer: Vec<f64>,
    pub width: usize,
    pub height: usize,
}

impl Framebuffer {
    pub fn new(width: usize, height: usize) -> Self {
        Framebuffer {
            color_buffer: vec![0; width * height],
            depth_buffer: vec![f64::MAX; width * height],
            width,
            height,
        }
    }

    pub fn clear(&mut self, color: u32) {
        for pixel in &mut self.color_buffer {
            *pixel = color;
        }
        for depth in &mut self.depth_buffer {
            *depth = f64::MAX;
        }
    }

    pub fn set_pixel(&mut self, x: usize, y: usize, color: u32, depth: f64) {
        if x < self.width && y < self.height {
            let index = y * self.width + x;
            if depth < self.depth_buffer[index] {
                self.color_buffer[index] = color;
                self.depth_buffer[index] = depth;
            }
        }
    }

    pub fn to_image(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(self.width as u32, self.height as u32);
        
        for (i, &pixel) in self.color_buffer.iter().enumerate() {
            let r = ((pixel >> 24) & 0xFF) as u8;
            let g = ((pixel >> 16) & 0xFF) as u8;
            let b = ((pixel >> 8) & 0xFF) as u8;
            let a = (pixel & 0xFF) as u8;
            
            let x = (i % self.width) as u32;
            let y = (i / self.width) as u32;
            
            img.put_pixel(x, y, Rgba([r, g, b, a]));
        }
        
        img
    }
}

pub struct SoftwareRenderer {
    pub framebuffer: Framebuffer,
}

impl SoftwareRenderer {
    pub fn new() -> Self {
        SoftwareRenderer {
            framebuffer: Framebuffer::new(BUFFER_WIDTH, BUFFER_HEIGHT),
        }
    }

    pub fn clear(&mut self, color: u32) {
        self.framebuffer.clear(color);
    }

    pub fn draw_line(&mut self, p1: Vec2, p2: Vec2, color: u32) {
        let dx = (p2.x - p1.x).abs();
        let dy = (p2.y - p1.y).abs();
        let sx = if p2.x > p1.x { 1 } else { -1 };
        let sy = if p2.y > p1.y { 1 } else { -1 };
        
        let mut err = if dx > dy { dx } else { -dy } / 2.0;
        let mut x = p1.x;
        let mut y = p1.y;
        
        loop {
            let xi = x.round() as usize;
            let yi = y.round() as usize;
            if xi < self.framebuffer.width && yi < self.framebuffer.height {
                self.framebuffer.set_pixel(xi, yi, color, 0.0);
            }
            
            if (x - p2.x).abs() < 1.0 && (y - p2.y).abs() < 1.0 {
                break;
            }
            
            let e2 = err;
            if e2 > -dx {
                err -= dy;
                x += sx as f64;
            }
            if e2 < dy {
                err += dx;
                y += sy as f64;
            }
        }
    }

    pub fn draw_circle(&mut self, center: Vec2, radius: f64, color: u32) {
        let mut x = radius;
        let mut y = 0.0;
        let mut decision = 3.0 - 2.0 * radius;
        
        while x >= y {
            // Draw 8 points
            self.draw_circle_point(center.x + x, center.y + y, color);
            self.draw_circle_point(center.x - x, center.y + y, color);
            self.draw_circle_point(center.x + x, center.y - y, color);
            self.draw_circle_point(center.x - x, center.y - y, color);
            self.draw_circle_point(center.x + y, center.y + x, color);
            self.draw_circle_point(center.x - y, center.y + x, color);
            self.draw_circle_point(center.x + y, center.y - x, color);
            self.draw_circle_point(center.x - y, center.y - x, color);
            
            if decision < 0.0 {
                decision += 4.0 * y + 6.0;
            } else {
                decision += 4.0 * (y - x) + 10.0;
                x -= 1.0;
            }
            y += 1.0;
        }
    }

    fn draw_circle_point(&mut self, x: f64, y: f64, color: u32) {
        let xi = x.round() as usize;
        let yi = y.round() as usize;
        if xi < self.framebuffer.width && yi < self.framebuffer.height {
            self.framebuffer.set_pixel(xi, yi, color, 0.0);
        }
    }

    pub fn draw_filled_circle(&mut self, center: Vec2, radius: f64, color: u32) {
        for y in 0..self.framebuffer.height {
            for x in 0..self.framebuffer.width {
                let dx = x as f64 - center.x;
                let dy = y as f64 - center.y;
                let distance = (dx * dx + dy * dy).sqrt();
                
                if distance <= radius {
                    self.framebuffer.set_pixel(x, y, color, 0.0);
                }
            }
        }
    }

    pub fn draw_triangle(&mut self, p1: Vec3, p2: Vec3, p3: Vec3, color: u32) {
        // Project 3D points to 2D screen space
        let screen_p1 = self.project_point(&p1);
        let screen_p2 = self.project_point(&p2);
        let screen_p3 = self.project_point(&p3);
        
        self.rasterize_triangle(screen_p1, screen_p2, screen_p3, color);
    }

    fn project_point(&self, point: &Vec3) -> Vec2 {
        let fov = 45.0_f64.to_radians();
        let _aspect = self.framebuffer.width as f64 / self.framebuffer.height as f64;
        
        let z_distance = point.z;
        if z_distance <= 0.0 {
            return Vec2::zero();
        }
        
        let scale = 1.0 / (fov / 2.0).tan();
        
        let x = (point.x * scale / z_distance) * (self.framebuffer.width as f64 / 2.0) + (self.framebuffer.width as f64 / 2.0);
        let y = -(point.y * scale / z_distance) * (self.framebuffer.height as f64 / 2.0) + (self.framebuffer.height as f64 / 2.0);
        
        Vec2::new(x, y)
    }

    fn rasterize_triangle(&mut self, p1: Vec2, p2: Vec2, p3: Vec2, color: u32) {
        let min_x = (p1.x.min(p2.x).min(p3.x)) as usize;
        let max_x = (p1.x.max(p2.x).max(p3.x)) as usize + 1;
        let min_y = (p1.y.min(p2.y).min(p3.y)) as usize;
        let max_y = (p1.y.max(p2.y).max(p3.y)) as usize + 1;
        
        let min_x = min_x.max(0).min(self.framebuffer.width);
        let max_x = max_x.max(0).min(self.framebuffer.width);
        let min_y = min_y.max(0).min(self.framebuffer.height);
        let max_y = max_y.max(0).min(self.framebuffer.height);
        
        for y in min_y..max_y {
            for x in min_x..max_x {
                if self.is_point_in_triangle(Vec2::new(x as f64, y as f64), p1, p2, p3) {
                    self.framebuffer.set_pixel(x, y, color, 0.0);
                }
            }
        }
    }

    fn is_point_in_triangle(&self, p: Vec2, a: Vec2, b: Vec2, c: Vec2) -> bool {
        let sign = |p1: Vec2, p2: Vec2, p3: Vec2| -> f64 {
            (p1.x - p3.x) * (p2.y - p3.y) - (p2.x - p3.x) * (p1.y - p3.y)
        };
        
        let d1 = sign(p, a, b);
        let d2 = sign(p, b, c);
        let d3 = sign(p, c, a);
        
        let has_neg = (d1 < 0.0) || (d2 < 0.0) || (d3 < 0.0);
        let has_pos = (d1 > 0.0) || (d2 > 0.0) || (d3 > 0.0);
        
        !(has_neg && has_pos)
    }

    pub fn get_framebuffer(&self) -> &Framebuffer {
        &self.framebuffer
    }

    pub fn get_width(&self) -> usize {
        self.framebuffer.width
    }

    pub fn get_height(&self) -> usize {
        self.framebuffer.height
    }
}
