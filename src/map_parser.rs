//! This contains a parser for `.obj` files produced for RecastDemo from the
//! mmap generation tools from mangos

use std::io;
use std::path::Path;
use std::convert::TryInto;
use std::collections::{BTreeSet, BTreeMap};

use cgmath::{InnerSpace, Vector3};

/// A vertex
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vertex(pub f32, pub f32, pub f32, pub f32);

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(&other).expect("Compare failed for Vertex")
    }
}

/// A representation of a `.obj` file, which contains vertex data and triangle
/// data
#[derive(Default)]
pub struct ObjFile {
    /// A list of all triangles and their verticies that compose them 
    triangles: BTreeSet<(Vertex, Vertex, Vertex)>,
}

impl ObjFile {
    /// Loads an object file from `path` into the object file data
    pub fn load<P: AsRef<Path>>(&mut self, path: P) -> io::Result<()> {
        // Load the file contents
        let data = std::fs::read_to_string(path)?;

        // All verticies as they are loaded from the file
        let mut verticies = Vec::new();

        // Go through each line
        for line in data.lines() {
            if line.starts_with("v ") {
                // Line contains vector data, parse it
                // Currently we only support: v <x> <y> <z>
                // We do not support `w` data
                let mut parts = line[2..].split(" ");
                let x: f32 = parts.next().unwrap().parse().unwrap();
                let y: f32 = parts.next().unwrap().parse().unwrap();
                let z: f32 = parts.next().unwrap().parse().unwrap();
                assert!(parts.next().is_none(),
                    "Whoa, unexpected vector data");

                // Record the vertex
                verticies.push(Vertex(x, y, z, 0.5));
            } else if line.starts_with("f ") {
                // Line contains polygon face index data, parse it
                // Currently we only support: p <a> <b> <c>
                // We also only support positive indicies, technically negative
                // indicies are possible and they are relative to the end of
                // the vertex list, but we don't deal with those, no reason to
                // parse them.
                let mut parts = line[2..].split(" ");
                let a: usize = parts.next().unwrap().parse().unwrap();
                let b: usize = parts.next().unwrap().parse().unwrap();
                let c: usize = parts.next().unwrap().parse().unwrap();
                assert!(parts.next().is_none(),
                    "Whoa, unexpected polygon face data");

                // Resolve the triangle verticies
                let a = verticies[a - 1];
                let b = verticies[b - 1];
                let c = verticies[c - 1];
                self.triangles.insert((a, b, c));
            } else {
                unimplemented!("Unexpected obj file line: {}", line);
            }
        }

        Ok(())
    }

    /// Create a list of VBO data (vertex data)
    pub fn vbo(&self) -> Vec<(f32, f32, f32, f32)> {
        // List of triangles
        let mut triangles = Vec::new();

        // Go through each triangle in our data
        for &(a, b, c) in &self.triangles {
            // Convert the verticies into vectors
            let a = Vector3::new(a.0, a.1, a.2);
            let b = Vector3::new(b.0, b.1, b.2);
            let c = Vector3::new(c.0, c.1, c.2);

            // Compute vectors
            let ab = b - a;
            let bc = c - b;

            // Compute the triangle normal
            let normal = ab.cross(bc).normalize();

            // Compute the dot product against our light source
            let light_source = Vector3::new(0., 1., 0.);
            let brightness = normal.dot(light_source).abs();

            triangles.push((a.x, a.y, a.z, brightness));
            triangles.push((b.x, b.y, b.z, brightness));
            triangles.push((c.x, c.y, c.z, brightness));
        }

        triangles
    }
}

