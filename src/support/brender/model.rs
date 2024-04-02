use {
    super::resource::{
        file_type, Chunk, FaceMaterialChunk, FacesChunk, FileInfoChunk, FromStream, LoadMany,
        MaterialIndexChunk, PivotChunk, ResourceStack, ResourceTag, Vec3f, VertexUV, VertexUvChunk,
        VerticesChunk,
    },
    crate::support::Error,
    bevy::{prelude::*, render::mesh::VertexAttributeValues},
    byteorder::ReadBytesExt,
    carma_derive::ResourceTag,
    culpa::{throw, throws},
    std::{
        fs::File,
        io::{prelude::BufRead, BufReader},
    },
};

// @todo ❌ convert meshes to bevy_render::mesh::Mesh

#[derive(Default, ResourceTag)]
pub struct Model {
    pub identifier: String,
    pub vertices: Vec<Vec3f>,
    // pub vertex_normals: Vec<Vec3f>,
    pub vertex_uvs: Vec<VertexUV>,
    pub faces: FacesChunk,
    pub material_names: Vec<String>, // vvv @todo ❌ convert to material refs
    // pub materials: Vec<Material>,
    pub face_material_indices: Vec<u16>,
    pub pivot: Vec3f,
}

impl std::fmt::Debug for Model {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "{}: {} vertices, {} uvs, {} faces, {} material names, {} face materials, pivot {},{},{}",//{} normals,
            self.identifier, self.vertices.len(), self.vertex_uvs.len(), self.faces.faces.len(),//self.vertex_normals.len(),
            self.material_names.len(), self.face_material_indices.len(), self.pivot.x, self.pivot.y, self.pivot.z)?;
        for name in &self.material_names {
            writeln!(f, " MAT: {name}")?;
        }
        Ok(())
    }
}

impl Model {
    fn bevy_vertices(&self) -> VertexAttributeValues {
        VertexAttributeValues::Float32x3(self.vertices.iter().map(|v| [v.x, v.y, v.z]).collect())
    }

    // fn bevy_vertex_normals(&self) -> VertexAttributeValues {
    //     VertexAttributeValues::Float32x3(
    //         self.vertex_normals
    //             .iter()
    //             .map(|n| [n.x, n.y, n.z])
    //             .collect(),
    //     )
    // }

    fn bevy_vertex_uvs(&self) -> VertexAttributeValues {
        VertexAttributeValues::Float32x2(self.vertex_uvs.iter().map(|t| [t.u, t.v]).collect())
    }

    fn bevy_faces(&self) -> bevy::render::mesh::Indices {
        bevy::render::mesh::Indices::U16(
            self.faces
                .faces
                .iter()
                .map(|f| [f.v1, f.v2, f.v3])
                .flatten()
                .collect(),
        )
    }

    // @todo: Use face_materials_index and material_names to create multiple meshes with
    // different materials.
    // Simple version: duplicate all vertices always, use face_materials_index to pull in faces with the same material.
    // for mat in materials {
    //   // one mesh per material
    //   mesh = face_materials
    //      .enumerate()
    //      .select(|(i, m)| m.index == mat.index)
    //      .map(|(i, m)| faces[i])
    //   material = materials[mat];
    //   colormap = material.pixelmap.remap(material.index_shade_tab)
    //   spawn(PbrBundle {
    //      mesh: mesh,
    //      material: StandardMaterial { material.props, texture: images.add(colormap) }
    //   })
    // }
    //
    pub fn bevy_mesh(&self) -> bevy::render::mesh::Mesh {
        use bevy::render::mesh::PrimitiveTopology;

        let mut mesh = Mesh::new(PrimitiveTopology::TriangleList);

        // Positions of the vertices
        // See https://bevy-cheatbook.github.io/features/coords.html
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_POSITION,
            self.bevy_vertices(),
            // vec![[0., 0., 0.], [1., 2., 1.], [2., 0., 0.]],
        );

        // mesh.insert_attribute(
        //     Mesh::ATTRIBUTE_NORMAL,
        //     self.bevy_vertex_normals(),
        //     //vec![[0., 1., 0.]; 3]
        // );
        mesh.insert_attribute(
            Mesh::ATTRIBUTE_UV_0,
            self.bevy_vertex_uvs(),
            //vec![[0., 0.]; 3]
        );

        // A triangle using vertices 0, 2, and 1.
        // Note: order matters. [0, 1, 2] will be flipped upside down, and you won't see it from behind!
        mesh.set_indices(Some(
            self.bevy_faces(),
            // vec![0, 2, 1])
        ));

        // @todo Materials!

        // Optionally, for more complicated geometry, instead of setting normals manually with
        // mesh.insert_attribute(Mesh::ATTRIBUTE_NORMAL,...), you can do:
        mesh.duplicate_vertices();
        //      It's because by default the normals are interpolated. You do have the option
        //      in WGSL to disable this using `flat` interpolation, but then all meshes rendered
        //      using that shader would loose the interpolated normals as discussed here. https://stackoverflow.com/questions/60022613/how-to-implement-flat-shading-in-opengl-without-duplicate-vertices
        //      So if you want to mix flat and smooth shaded meshes using the same shader,
        //      duplicating vertices is the only option you have.
        mesh.compute_flat_normals();
        // after mesh.set_indices(...).
        // Note the warnings about increasing the vertex count.
        mesh
    }
}

impl FromStream for Model {
    type Output = Box<Model>;

    #[throws]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut stack = ResourceStack::new();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::MODEL {
                        throw!(
                            Error::InvalidResourceType /*{
                                                       expected: file_type::MODEL,
                                                       received: file_type,
                                                       }*/
                        );
                    }
                }
                Chunk::Model(model_chunk) => {
                    let mut model = Model::default();
                    model.identifier = model_chunk.identifier;
                    stack.push(Box::new(model));
                }
                Chunk::Vertices(VerticesChunk { vertices }) => {
                    let model = stack.top::<Model>()?;
                    model.vertices = vertices;
                }
                Chunk::VertexUV(VertexUvChunk { uvs }) => {
                    let model = stack.top::<Model>()?;
                    model.vertex_uvs = uvs;
                }
                Chunk::Faces(faces) => {
                    let model = stack.top::<Model>()?;
                    model.faces = faces;
                }
                Chunk::MaterialIndex(MaterialIndexChunk { materials }) => {
                    let model = stack.top::<Model>()?;
                    let mut vec = vec!["".to_string()];
                    vec.extend(materials);
                    model.material_names = vec;
                    // @todo prepend with a default material entry (NULL material)
                    // stack.push(Box::new(materials_index));
                }
                Chunk::FaceMaterial(FaceMaterialChunk {
                    face_material_indices,
                }) => {
                    // let material_index = stack.pop::<MaterialIndex>()?;
                    let model = stack.top::<Model>()?;
                    // @todo use material_index to assign materials to faces...
                    model.face_material_indices = face_material_indices;
                }
                Chunk::Pivot(PivotChunk { pivot }) => {
                    let model = stack.top::<Model>()?;
                    model.pivot = pivot;
                }
                _ => unimplemented!(), // unexpected type for a model file
            }
        }

        stack.pop::<Model>()?
    }
}

// pub fn load<R: ReadBytesExt + BufRead>(_reader: &mut R) -> Result<Mesh> {
//     for (i, fm) in fmlist.iter().enumerate().take(m.faces.len()) {
//         m.faces[i].material_id = *fm;
//     }

//     for (n, uvcoord) in uvcoords.iter().enumerate() {
//         // Carma uses 0.0,0.0 for the top left corner, OpenGL for the bottom left.
//         m.vertices[n].tex_coords = [uvcoord.u, 1.0 - uvcoord.v];
//     }

//     m.calc_normals();
//     Ok(m)
// }

/// Single model file may contain multiple models.
impl LoadMany for Model {
    type Outputs = Box<Model>;

    #[throws]
    fn load_many<P: AsRef<std::path::Path> + std::fmt::Debug>(filename: P) -> Vec<Self::Outputs> {
        debug!("Loading many Models from {:?}", filename);
        let mut file = BufReader::new(File::open(filename)?);
        let mut models = Vec::<_>::new();
        loop {
            let m = Model::from_stream(&mut file);
            match m {
                Err(_) => break, // fixme: allow only Eof here
                Ok(m) => {
                    trace!(".. Loaded {}", m.identifier);
                    models.push(m)
                }
            }
        }
        models
    }
}

// impl Model {
//     pub fn calc_normals(&mut self) {
//         self.vertex_normals.clear();
//         self.vertex_normals.reserve(self.vertices.len());

//         let normals = HashSet::new();
//         for face in &self.faces.faces {
//             let normal: [f32; 3] = calc_plane_normal(
//                 self.vertices[face.v1 as usize],
//                 self.vertices[face.v2 as usize],
//                 self.vertices[face.v3 as usize],
//             )
//             .into();
//             normals.entry(face.v1).normal = normal;
//             normals.entry(face.v2).normal = normal;
//             normals.entry(face.v3).normal = normal;
//         }
//     }
// }

/// Calculate normal from three vertices in counter-clockwise order.
// pub fn calc_plane_normal(v1: Vector3<f32>, v2: Vector3<f32>, v3: Vector3<f32>) -> Vector3<f32> {
//     (v1 - v2).cross(v2 - v3).normalize()
// }

#[cfg(test)]
mod tests {
    use {super::* /* , cgmath::Zero*/, std::io::Cursor};

    #[test]
    fn test_load_model() {
        #[rustfmt::skip]
        let mut data = Cursor::new(vec![
            0x0, 0x0, 0x0, 0x36, // Chunk type - MODEL
            0x0, 0x0, 0x0, 0x8, // Chunk size
            0x0, 0x3, // flags u16
            b'h', b'e', b'l', b'l', b'o', 0, // identifier
            0x0, 0x0, 0x0, 0x0, // Chunk type - NULL_CHUNK
            0x0, 0x0, 0x0, 0x0, // Chunk size
        ]);
        let m = Model::from_stream(&mut data).unwrap();
        assert_eq!("hello", m.identifier);
        // assert_eq!(0xbeef, m.v2);
        // assert_eq!(0xcafe, m.v3);
        // assert_eq!(0xbabe, m.flags);
    }

    // test that normals to unit vectors will be the third unit vector
    // #[test]
    // fn test_calc_normal() {
    //     assert_eq!(
    //         calc_plane_normal(Vector3::unit_y(), Vector3::zero(), Vector3::unit_x()),
    //         Vector3::unit_z()
    //     );
    //     assert_eq!(
    //         calc_plane_normal(Vector3::unit_x(), Vector3::zero(), Vector3::unit_y()),
    //         -Vector3::unit_z()
    //     );
    // }
} // tests mod
