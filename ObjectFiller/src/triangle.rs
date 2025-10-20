use crate::framebuffer::Framebuffer;
use nalgebra::Vector2;

fn edge(a: Vector2<i32>, b: Vector2<i32>, c: Vector2<i32>) -> i32 {
    (b.x - a.x) * (c.y - a.y) - (b.y - a.y) * (c.x - a.x)
}

pub fn fill_triangle_depth(
    fb: &mut Framebuffer,
    a: Vector2<i32>,
    b: Vector2<i32>,
    c: Vector2<i32>,
    za: f32,
    zb: f32,
    zc: f32,
    color: u32,
) {
    let min_x = a.x.min(b.x).min(c.x).max(0);
    let max_x = a.x.max(b.x).max(c.x).min((crate::framebuffer::WIDTH - 1) as i32);
    let min_y = a.y.min(b.y).min(c.y).max(0);
    let max_y = a.y.max(b.y).max(c.y).min((crate::framebuffer::HEIGHT - 1) as i32);

    let area = edge(a, b, c);
    if area == 0 {
        return;
    }
    let area_f = area as f32;

    for y in min_y..=max_y {
        for x in min_x..=max_x {
            let p = Vector2::new(x, y);
            let w0 = edge(b, c, p) as f32 / area_f;
            let w1 = edge(c, a, p) as f32 / area_f;
            let w2 = edge(a, b, p) as f32 / area_f;

            if (w0 >= 0.0 && w1 >= 0.0 && w2 >= 0.0)
                || (w0 <= 0.0 && w1 <= 0.0 && w2 <= 0.0)
            {
                let z = za * w0 + zb * w1 + zc * w2;
                fb.put_pixel_depth(x, y, z, color);
            }
        }
    }
}
