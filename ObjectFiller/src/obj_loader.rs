use nalgebra::Vector3;
use std::fs::File;
use std::io::{BufRead, BufReader};

pub struct Mesh {
    pub vertices: Vec<Vector3<f32>>,
    pub faces: Vec<[usize; 3]>,
}

pub fn load_obj(path: &str) -> Mesh {
    let file = File::open(path).expect("No se pudo abrir el archivo OBJ");
    let reader = BufReader::new(file);
    let mut vertices = Vec::new();
    let mut faces = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        if line.starts_with("v ") {
            let parts: Vec<_> = line.split_whitespace().collect();
            let x: f32 = parts[1].parse().unwrap();
            let y: f32 = parts[2].parse().unwrap();
            let z: f32 = parts[3].parse().unwrap();
            vertices.push(Vector3::new(x, y, z));
        } else if line.starts_with("f ") {
            let parts: Vec<_> = line.split_whitespace().collect();
            let mut idx = [0usize; 3];
            for i in 0..3 {
                idx[i] = parts[i + 1].split('/').next().unwrap().parse::<usize>().unwrap() - 1;
            }
            faces.push(idx);
        }
    }

    Mesh { vertices, faces }
}
