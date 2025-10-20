mod framebuffer;
mod triangle;
mod obj_loader;
mod model_util;

use framebuffer::*;
use triangle::*;
use obj_loader::*;
use model_util::*;

use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use nalgebra::Vector3;
use std::time::Duration;

const USE_PERSPECTIVE: bool = true;
const CAMERA_DIST: f32 = 3.3;
const BG_COLOR: u32 = 0x339CFFFF; // fondo
const BASE_COLOR: (u8,u8,u8) = (255,150,255); // más claro p/ ver detalle
const LIGHT_DIR: [f32;3] = [0.5,0.4,1.0];     // luz suave

fn main() {
    let sdl = sdl2::init().unwrap();
    let mut fb = Framebuffer::new(&sdl);

    let mesh = load_obj("assets/nave_andres.obj");
    let fit = fit_to_screen(&mesh.vertices);
    let light_dir = Vector3::new(LIGHT_DIR[0], LIGHT_DIR[1], LIGHT_DIR[2]).normalize();

    let mut pump = sdl.event_pump().unwrap();
    let mut running = true;

    while running {
        for e in pump.poll_iter() {
            match e {
                Event::Quit { .. } => {
                    // Guarda al cerrar
                    std::fs::create_dir_all("docs").ok();
                    fb.save_png("docs/screenshot.png");
                    println!("✅ Guardado: docs/screenshot.png");
                    running = false;
                }
                Event::KeyDown { keycode: Some(Keycode::S), .. } => {
                    // Guarda al presionar S
                    std::fs::create_dir_all("docs").ok();
                    fb.save_png("docs/screenshot.png");
                    println!("✅ Guardado: docs/screenshot.png");
                }
                _ => {}
            }
        }

        fb.clear(BG_COLOR);

        for face in &mesh.faces {
            let v0 = mesh.vertices[face[0]];
            let v1 = mesh.vertices[face[1]];
            let v2 = mesh.vertices[face[2]];

            // sombreado MUY suave (solo para marcar volumen)
            let mut n = (v1 - v0).cross(&(v2 - v0));
            if n.magnitude_squared() < 1e-12 { continue; }
            n = n.normalize();
            let intensity = n.dot(&light_dir).clamp(0.4, 1.0);

            let r = (BASE_COLOR.0 as f32 * intensity) as u32;
            let g = (BASE_COLOR.1 as f32 * intensity) as u32;
            let b = (BASE_COLOR.2 as f32 * intensity) as u32;
            let color = 0xFF000000 | (r<<16) | (g<<8) | b;

            let (a, za) = if USE_PERSPECTIVE { project_persp_depth(&v0, &fit, CAMERA_DIST) }
                          else               { (project_ortho(&v0, &fit), 1.0) };
            let (b2, zb) = if USE_PERSPECTIVE { project_persp_depth(&v1, &fit, CAMERA_DIST) }
                           else               { (project_ortho(&v1, &fit), 1.0) };
            let (c, zc) = if USE_PERSPECTIVE { project_persp_depth(&v2, &fit, CAMERA_DIST) }
                          else               { (project_ortho(&v2, &fit), 1.0) };

            let area = (b2.x - a.x)*(c.y - a.y) - (b2.y - a.y)*(c.x - a.x);
            if area <= 0 { continue; }
            if za <= 0.0 || zb <= 0.0 || zc <= 0.0 { continue; }

            fill_triangle_depth(&mut fb, a, b2, c, za, zb, zc, color);
        }

        fb.present();
        std::thread::sleep(Duration::from_millis(16));
    }
}
