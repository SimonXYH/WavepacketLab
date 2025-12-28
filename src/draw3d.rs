use crate::screen_config::correct_coord;
use crate::vec2::Vec2;
use crate::vec3::Vec3;

pub struct Scene3d {
    pub center: Vec2<f32>,
    pub zoom: f32,
    pub perspective: f32,
    pub angles: (f32, f32),
}

pub fn rotate_x(pos: Vec3, angles: (f32, f32)) -> f32 {
    angles.0.sin() * pos.x + angles.1.cos() * pos.y
}
pub fn rotate_y(pos: Vec3, angles: (f32, f32)) -> f32 {
    let (a, b) = angles;
    a.cos() * b.sin() * pos.x - a.sin() * b.sin() * pos.y + b.cos() * -pos.z
}

pub fn rotate_z(pos: Vec3, angles: (f32, f32)) -> f32 {
    let (a, b) = angles;
    -a.cos() * b.cos() * pos.x + b.cos() * a.sin() * pos.y + b.sin() * -pos.z
}

pub fn rotate(pos: Vec3, angles: (f32, f32)) -> (f32, f32, f32) {
    let (a, b) = angles;
    let (a_cos, a_sin, b_cos, b_sin) = (a.cos(), a.sin(), b.cos(), b.sin());
    (
        a_sin * pos.x + a_cos * pos.y,
        a_cos * b_sin * pos.x - a_sin * b_sin * pos.y + b_cos * -pos.z,
        -a_cos * b_cos * pos.x + b.cos() * a_sin * pos.y + b_sin * -pos.z,
    )
}

impl Scene3d {
    pub fn new(center: Vec2<f32>, zoom: f32, perspective: f32, angles: (f32, f32)) -> Scene3d {
        Scene3d {
            center,
            zoom,
            perspective,
            angles,
        }
    }
    pub fn project(&self, pos: Vec3) -> Vec2<f32> {
        let (rx, ry, rz) = rotate(pos, self.angles);
        let perspective_factor = self.perspective / (rz + self.perspective);
        Vec2::new(rx, ry) * perspective_factor * self.zoom + self.center
    }

    pub fn project_correct(
        &self,
        pos: Vec3,
        center_outer: (i32, i32),
        zoom_outer: f32,
    ) -> Vec2<i32> {
        correct_coord(self.project(pos), center_outer, zoom_outer)
    }
}
