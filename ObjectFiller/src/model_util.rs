use nalgebra::{Vector2, Vector3};
use crate::framebuffer::{WIDTH, HEIGHT};

#[derive(Clone)]
pub struct Fit {
    pub min: Vector3<f32>,
    pub max: Vector3<f32>,
    pub scale: f32,
    pub center: Vector2<f32>,
}

pub fn fit_to_screen(vertices: &[Vector3<f32>]) -> Fit {
    let min = vertices.iter().fold(Vector3::new(f32::INFINITY, f32::INFINITY, f32::INFINITY),
        |acc, v| acc.zip_map(v, |a, b| a.min(b)));
    let max = vertices.iter().fold(Vector3::new(f32::NEG_INFINITY, f32::NEG_INFINITY, f32::NEG_INFINITY),
        |acc, v| acc.zip_map(v, |a, b| a.max(b)));

    let extent = max - min;
    let scale = 0.9 * (WIDTH.min(HEIGHT) as f32) / extent.x.max(extent.y);
    let center = Vector2::new(WIDTH as f32 / 2.0, HEIGHT as f32 / 2.0);
    Fit { min, max, scale, center }
}

pub fn project_ortho(v: &Vector3<f32>, fit: &Fit) -> Vector2<i32> {
    let center3 = (fit.min + fit.max) / 2.0;
    let p = (*v - center3) * fit.scale + Vector3::new(fit.center.x, fit.center.y, 0.0);
    Vector2::new(p.x as i32, p.y as i32)
}

pub fn project_persp_depth(v: &Vector3<f32>, fit: &Fit, cam_dist: f32) -> (Vector2<i32>, f32) {
    let center3 = (fit.min + fit.max) / 2.0;
    let mut p = *v - center3;
    let z_cam = p.z + cam_dist;
    let z = z_cam.max(0.0001);
    p.x /= z;
    p.y /= z;
    p *= fit.scale * 2.0;
    p.y = -p.y;
    let sx = (fit.center.x + p.x) as i32;
    let sy = (fit.center.y + p.y) as i32;
    (Vector2::new(sx, sy), z_cam)
}
