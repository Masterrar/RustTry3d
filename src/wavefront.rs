extern crate nalgebra;

use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;

use nalgebra::core::{Vector2, Vector3};


/// A wavefront object containing vertices and faces from a wavefront file.
///
/// An instance of Object will contain the geometric vertices, geometric faces,
/// texture vertices, and texture faces from a given wavefront object file.
///
/// # Examples
///
/// Create a new Object from a given file and iterate over its vertices
///
/// ```
/// let coordinates = wavefront::Object::new("file.obj");
///
/// for geometric_vertex in coordinates.geometric_vertices {
///     // do something with the vertices
/// }
/// ```
///
pub struct Object {
    pub geometric_vertices: Vec<Vector3<f64>>,
    pub geometric_faces: Vec<Vector3<i32>>,
    pub texture_vertices: Vec<Vector2<f64>>,
    pub texture_faces: Vec<Vector3<i32>>,
    pub normal_vertices: Vec<Vector3<f64>>,
    pub normal_faces: Vec<Vector3<i32>>
}


impl Object {
    /// Create an instance of Object from the given filename.
    ///
    /// The geometric vertices are parsed from all lines that begin with
    /// 'v' while the texture vertices are parsed from all lines that
    /// begin with 'vt'. The faces are parsed from lines that begin
    /// with 'f'. Faces are assumed to be in the form of a triangle triplet
    /// such as 1100/1200/1300 1000/900/800 100/200/300 where 1100, 1000,
    /// and 100 are the geometric faces and 1200, 900, and 200 are the
    /// texture faces.
    ///
    /// # Examples
    ///
    /// Create a new Object from a given file and iterate over its faces.
    ///
    /// ```
    /// let coordinates = wavefront::Object::new("file.obj");
    ///
    /// for geometric_face in coordinates.geometric_faces {
    ///     // do something with the faces
    /// }
    /// ```
    ///
    pub fn new(filename: &str) -> Object {
        let file = BufReader::new(File::open(filename).unwrap());
        let mut geometric_vertices: Vec<Vector3<f64>> = Vec::new();
        let mut geometric_faces: Vec<Vector3<i32>> = Vec::new();
        let mut texture_vertices: Vec<Vector2<f64>> = Vec::new();
        let mut texture_faces: Vec<Vector3<i32>> = Vec::new();
        let mut normal_vertices: Vec<Vector3<f64>> = Vec::new();
        let mut normal_faces: Vec<Vector3<i32>> = Vec::new();

        for line in file.lines().map(|l| l.unwrap()) {
            if line.starts_with("v ") {
                let v_coordinates = line.split_at(2).1
                                        .split_whitespace()
                                        .map(|n| n.parse().unwrap())
                                        .collect::<Vec<f64>>();

                geometric_vertices.push(Vector3::new(v_coordinates[0],
                                                     v_coordinates[1],
                                                     v_coordinates[2]));
            }
            else if line.starts_with("vt ") {
                let vt_coordinates = line.split_at(3).1
                                         .split_whitespace()
                                         .map(|n| n.parse().unwrap())
                                         .collect::<Vec<f64>>();

                texture_vertices.push(Vector2::new(vt_coordinates[0], vt_coordinates[1]));
            }
            else if line.starts_with("vn ") {
                let vn_coordinates = line.split_at(3).1
                                         .split_whitespace()
                                         .map(|n| n.parse().unwrap())
                                         .collect::<Vec<f64>>();

                normal_vertices.push(Vector3::new(vn_coordinates[0],
                                                  vn_coordinates[1],
                                                  vn_coordinates[2]));
            }
            else if line.starts_with("f ") {
                let f_coordinates = line.split_at(2).1
                                        .split(|c| c == '/' || c == ' ')
                                        .map(|n| n.parse().unwrap())
                                        .collect::<Vec<i32>>();

                geometric_faces.push(Vector3::new(f_coordinates[0] - 1,
                                                  f_coordinates[3] - 1,
                                                  f_coordinates[6] - 1));

                texture_faces.push(Vector3::new(f_coordinates[1] - 1,
                                                f_coordinates[4] - 1,
                                                f_coordinates[7] - 1));

                normal_faces.push(Vector3::new(f_coordinates[2] - 1,
                                               f_coordinates[5] - 1,
                                               f_coordinates[8] - 1));

                if f_coordinates.len() == 12 {
                    geometric_faces.push(Vector3::new(f_coordinates[0] - 1,
                                                      f_coordinates[6] - 1,
                                                      f_coordinates[9] - 1));

                    texture_faces.push(Vector3::new(f_coordinates[1] - 1,
                                                    f_coordinates[7] - 1,
                                                    f_coordinates[10] - 1));

                    normal_faces.push(Vector3::new(f_coordinates[2] - 1,
                                                   f_coordinates[8] - 1,
                                                   f_coordinates[11] - 1));
                }
            }
        }

        Object { geometric_vertices: geometric_vertices, geometric_faces: geometric_faces,
                 texture_vertices: texture_vertices, texture_faces: texture_faces,
                 normal_vertices: normal_vertices, normal_faces: normal_faces }
    }
}
