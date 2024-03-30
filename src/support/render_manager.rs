//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{camera::CameraState, car::Car, Vertex},
    cgmath::Matrix4,
    glium::{
        texture::{RawImage2d, SrgbTexture2d},
        uniform,
        uniforms::*,
        Display, IndexBuffer, Program, Surface, VertexBuffer,
    },
    log::*,
    std::{collections::HashMap, str},
};

/// Provide storage for in-memory level-data - models, meshes, textures etc.
/// @todo ❌ use bevy AssetLoader and Asset<> resources (esp for the shader program)
pub struct RenderManager {
    vertices: HashMap<String, VertexBuffer<Vertex>>,
    indices: HashMap<String, HashMap<u16, IndexBuffer<u16>>>, // MaterialId -> index buffer
    bound_textures: HashMap<String, HashMap<u16, SrgbTexture2d>>, // MaterialId -> texture
    program: Program,
}

fn debug_tree(name: &String, actor_name: &String, stack: &[Matrix4<f32>]) {
    debug!("{} for {}: stack depth {}", name, actor_name, stack.len());
    for x in stack.iter().rev() {
        debug!(".. {:?}", x);
    }
}

// Load VertexBuffer
// Load IndexBuffers for each material
// Set up renderlist (VertexBuffer, IndexBuffer)
impl RenderManager {
    pub fn new(display: &Display) -> Self {
        let vertex_shader_src = str::from_utf8(include_bytes!("../../shaders/first.vert")).unwrap();
        let fragment_shader_src =
            str::from_utf8(include_bytes!("../../shaders/first.frag")).unwrap();

        Self {
            vertices: HashMap::new(),
            indices: HashMap::new(),
            bound_textures: HashMap::new(),
            program: Program::from_source(display, vertex_shader_src, fragment_shader_src, None)
                .unwrap(),
        }
    }

    fn debug_indices(&self) {
        for name in self.indices.keys() {
            trace!("Indices for {}:", name);
            // for () in &indices {
            //     trace!("  ", )
            // }
        }
    }

    // @todo ❌ just map uv of a default black in megatexture
    fn bind_default_texture(
        textures: &mut HashMap<u16, SrgbTexture2d>,
        mat: u16,
        display: &Display,
    ) {
        let black_data = [0; 32 * 32 * 4];
        let black_image = RawImage2d::from_raw_rgba_reversed(&black_data, (32, 32));
        let black_texture = SrgbTexture2d::new(display, black_image).unwrap();
        textures.insert(mat, black_texture);
    }

    // @todo ❌ Prepare megatexture from all these small textures and keep a bevy::TextureAtlas
    // of texture ID to the rect region, scale u,v appropriately in vertices.
    // In theory, whole of the game could fit in 4096x4096 megatex.
    fn bind_textures(&mut self, actor_name: &String, car: &Car, display: &Display) {
        for &mat in self.indices[actor_name].keys() {
            let textures = self.bound_textures.entry(actor_name.clone()).or_default();
            if mat == 0 {
                RenderManager::bind_default_texture(textures, mat, display);
            } else {
                let material = &car.meshes[actor_name].material_names[(mat - 1) as usize];
                trace!("Referred material {} index {}", material, mat);
                if let Some(m) = car.materials.get(material) {
                    trace!("Found material {}", m);
                    let mut name = m.color_map_name.clone();
                    if name.is_empty() {
                        // @fixme hack
                        name = material.to_lowercase().replace(".mat", ".pix");
                    }
                    if let Some(tex) = car.textures.get(&name) {
                        trace!("Found texture {}", tex);
                        let image = RawImage2d::from_raw_rgba_reversed(
                            &tex.data,
                            (tex.width as u32, tex.height as u32),
                        );
                        let bound_texture = SrgbTexture2d::new(display, image).unwrap();
                        textures.insert(mat, bound_texture);
                    } else {
                        RenderManager::bind_default_texture(textures, mat, display);
                    }
                }
            }
        }
    }

    pub fn prepare_car(&mut self, _car: &Car, _display: &Display) {
        // for actor in car.actors.traverse() {
        //     if let ActorNode::MeshfileRef(name) = actor.data() {
        //         debug!("Actor meshfile {}", name);
        //         self.prepare_car_actor(name, car, display);
        //     }
        // }
    }

    pub fn prepare_car_actor(&mut self, name: &String, _car: &Car, _display: &Display) {
        debug!("prepare_car_actor({}): loading vertices", name);
        // let vbo = VertexBuffer::<Vertex>::new(display, &car.meshes[name].vertices).unwrap();
        // self.vertices.insert(name.clone(), vbo);

        // debug!("prepare_car_actor({}): partitioning faces", name);

        // let faces = &car.meshes[name].faces;

        // let mut partitioned_by_material = HashMap::<u16, Vec<u16>>::new();

        // for face in faces {
        //     let indices = partitioned_by_material.entry(face.material_id).or_default();
        //     indices.push(face.v1);
        //     indices.push(face.v2);
        //     indices.push(face.v3);
        // }

        // for (mat, list) in &partitioned_by_material {
        //     debug!(
        //         "Material {}: {} vertices, {} faces",
        //         mat,
        //         list.len(),
        //         list.len() as f32 / 3f32
        //     );
        // }

        // self.indices.insert(
        //     name.clone(),
        //     partitioned_by_material
        //         .iter()
        //         .map(|(key, item)| {
        //             (
        //                 *key,
        //                 IndexBuffer::new(display, PrimitiveType::TrianglesList, item).unwrap(),
        //             )
        //         })
        //         .collect(),
        // );

        // // each material from partitioned_by_material - load and bind it in bound_textures
        // // then remap to a set of HashMap from String material name to Vec<u16> indices
        // self.bind_textures(name, car, display);
    }

    /// Draw all visible actors
    pub fn draw_car<T>(&self, _car: &Car, _target: &mut T, _camera: &CameraState)
    where
        T: Surface,
    {
        // let mut v = false;
        // let mut transform_stack = Vec::<Matrix4<f32>>::new();
        // transform_stack.push(Matrix4::from_translation(car.base_translation) * Matrix4::identity());

        // let mut actor_name = String::new();

        // @fixme split those actors into Entities?
        // for actor in car.actors.traverse() {
        //     match *actor.data() {
        //         ActorNode::Actor { ref name, visible } => {
        //             actor_name = name.clone();
        //             v = visible;

        //             // let depth = car.actors.get_node_depth(actor) - 1;
        //             let depth = 0;
        //             trace!("Actor {} depth {}", name, depth);
        //             if depth < transform_stack.len() {
        //                 let pop_count = transform_stack.len() - depth;
        //                 trace!("Restoring transform - {} times", pop_count);
        //                 for _ in 0..pop_count {
        //                     transform_stack.pop().unwrap();
        //                 }
        //             }

        //             debug_tree(&String::from("Actor"), &actor_name, &transform_stack);
        //         }
        //         ActorNode::MeshfileRef(ref name) => {
        //             debug_tree(&format!("Mesh {}", name), &actor_name, &transform_stack);
        //             if v {
        //                 trace!("Drawing actor {}", name);
        //                 self.draw_actor(name, transform_stack.last().unwrap(), target, camera);
        //             }
        //         }
        //         ActorNode::Transform(t) => {
        //             let transform = Matrix4::from_translation(Vector3 {
        //                 x: t[9],
        //                 y: t[10],
        //                 z: t[11],
        //             }) * Matrix4::from_nonuniform_scale(t[0], t[4], t[8]);

        //             let model = transform * transform_stack.last().unwrap();
        //             transform_stack.push(model);

        //             debug_tree(
        //                 &String::from("Transform(after)"),
        //                 &actor_name,
        //                 &transform_stack,
        //             );
        //         }
        //         _ => (),
        //     }
        // }
    }

    /// Uses single mesh, but specific indices to draw with each material.
    fn draw_actor<T>(
        &self,
        mesh_name: &String,
        model: &Matrix4<f32>,
        target: &mut T,
        camera: &CameraState,
    ) where
        T: Surface,
    {
        trace!("Rendering {} with model {:?}", mesh_name, model);

        // the direction of the light - @todo ❌ more light sources?
        let light = [-5.0, 5.0, 10.0f32];
        // Ambient lighting: 0.5, 0.5, 0.5, 1.0

        let params = glium::DrawParameters {
            depth: glium::Depth {
                test: glium::draw_parameters::DepthTest::IfLess,
                write: true,
                ..Default::default()
            },
            backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
            ..Default::default()
        };

        let model: [[f32; 4]; 4] = (*model).into();

        for (mat, indices) in &self.indices[mesh_name] {
            let uniforms = uniform! {
                model: model,
                view: camera.get_view(),
                perspective: camera.get_perspective(),
                u_light: light,
                u_specular_color: [1.0, 1.0, 1.0f32],
                diffuse_tex: Sampler::new(&self.bound_textures[mesh_name][mat])
                    .minify_filter(MinifySamplerFilter::Linear)
                    .magnify_filter(MagnifySamplerFilter::Linear)
                    .wrap_function(SamplerWrapFunction::Repeat),
                // normal_tex: &self.bound_textures[mat],
            };

            target
                .draw(
                    &self.vertices[mesh_name],
                    indices,
                    &self.program,
                    &uniforms,
                    &params,
                )
                .unwrap();
        }
    }
}
