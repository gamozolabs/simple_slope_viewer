//! This contains a parser for `.obj` files produced for RecastDemo from the
//! mmap generation tools from mangos

use std::io;
use std::fs::File;
use std::io::{Write, BufWriter};
use std::path::Path;
use std::convert::TryInto;
use std::collections::{BTreeSet, BTreeMap};

/// A vertex
#[repr(C)]
#[derive(Default, Clone, Copy, Debug, PartialEq, PartialOrd)]
pub struct Vertex(pub f32, pub f32, pub f32);

impl Eq for Vertex {}

impl Ord for Vertex {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).expect("Compare failed for Vertex")
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
                verticies.push(Vertex(x, y, z));
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

    /// Create a list of unique verticies and indicies into them creating
    /// triangles
    pub fn write_vbo_index<P: AsRef<Path>>(&self, path: P) -> io::Result<()> {
        // List of verticies
        let mut verticies: Vec<Vertex> = Vec::new();

        // Lookup table from vertex to index
        let mut vertex_index: BTreeMap<Vertex, usize> = BTreeMap::new();

        // List of triangles by index
        let mut triangles: Vec<(u32, u32, u32)> = Vec::new();

        // Go through each triangle in our data
        for &(a, b, c) in &self.triangles {
            for &x in &[a, b, c] {
                // Save all unique verticies into the `verticies` list and
                // maintain a lookup table from verticies to their indicies
                if !vertex_index.contains_key(&x) {
                    vertex_index.insert(x, verticies.len());
                    verticies.push(x);
                }
            }

            // Get the indicies into the verticies
            let vertex_indicies: (u32, u32, u32) = (
                vertex_index[&a].try_into().unwrap(),
                vertex_index[&b].try_into().unwrap(),
                vertex_index[&c].try_into().unwrap(),
            );

            // Save the triangle data based on index rather than actual vertex
            // data
            triangles.push(vertex_indicies);
        }

        // Create the output file
        let mut outfd = BufWriter::new(File::create(path)?);

        // Write the number of verticies
        outfd.write_all(&(verticies.len() as u64).to_le_bytes())?;

        // Write all verticies
        for &vertex in &verticies {
            // Write x, y, z data
            outfd.write_all(&vertex.0.to_le_bytes())?;
            outfd.write_all(&vertex.1.to_le_bytes())?;
            outfd.write_all(&vertex.2.to_le_bytes())?;
        }
        
        // Write the number of triangles
        outfd.write_all(&(triangles.len() as u64).to_le_bytes())?;

        // Write all triangles
        for &triangle in &triangles {
            // Write vertex index data
            outfd.write_all(&triangle.0.to_le_bytes())?;
            outfd.write_all(&triangle.1.to_le_bytes())?;
            outfd.write_all(&triangle.2.to_le_bytes())?;
        }

        Ok(())
    }
}

fn main() -> io::Result<()> {
    // Get the arguments
    let args: Vec<String> = std::env::args().collect();

    // For each file, load it!
    let mut obj = ObjFile::default();
    for filename in &args[1..] {
        print!("Loading {}\n", filename);
        obj.load(filename)?;
    }

    obj.write_vbo_index("foop.falkvbo")?;

    Ok(())
}

