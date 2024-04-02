//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    super::{
        model::Model,
        resource::{
            file_type, BoundsChunk, CameraChunk, Chunk, FileInfoChunk, FromStream, LightChunk,
            LoadMany, PlaneChunk, ResourceStack, ResourceTag, TransformEulerChunk,
            TransformLookUpChunk, TransformMatrix34Chunk, TransformQuatChunk,
            TransformTranslationChunk,
        },
    },
    crate::support::Error,
    byteorder::ReadBytesExt,
    carma_derive::ResourceTag,
    culpa::{throw, throws},
    std::{
        fs::File,
        io::{BufRead, BufReader},
    },
};

// Typical actor tree:
// Root
// +--Actor(NAME.ACT)
//    +--Transform()
//    +--MeshfileRef
//    +--Actor(SUBACT.ACT)
//       +--Transform()
//       +--MeshfileRef
//

#[derive(Default, ResourceTag)]
enum Transform {
    #[default]
    Identity,
    Matrix34(TransformMatrix34Chunk),
    Matrix34LP(TransformMatrix34Chunk),
    Euler(TransformEulerChunk),
    Quat(TransformQuatChunk),
    LookUp(TransformLookUpChunk),
    Translation(TransformTranslationChunk),
}

#[derive(Default)]
enum ActorData {
    #[default]
    None,
    Light(Box<LightChunk>),
    Camera(Box<CameraChunk>),
    Bounds(Box<BoundsChunk>),
    ClipPlane(Box<PlaneChunk>),
}

#[derive(Default, ResourceTag)]
pub struct Actor {
    model: Box<Model>,
    transform: Transform,
    material: (),
    data: ActorData,
    children: Vec<Box<Actor>>,
}

impl LoadMany for Actor {
    type Outputs = Box<Actor>;

    #[throws]
    fn load_many<P: AsRef<std::path::Path> + std::fmt::Debug>(filename: P) -> Vec<Self::Outputs> {
        let mut file = BufReader::new(File::open(filename)?);
        vec![<Self as FromStream>::from_stream(&mut file)?]
    }
}

impl FromStream for Actor {
    type Output = Box<Actor>;

    #[throws]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut stack = ResourceStack::new();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::ACTOR {
                        throw!(
                            Error::InvalidResourceType /*{
                                                       expected: file_type::ACTOR,
                                                       received: file_type,
                                                       }*/
                        );
                    }
                }

                Chunk::Actor(actor) => {
                    let mut act = Actor::default();
                    // @todo Type and render_style
                    stack.push(Box::new(act));
                }
                Chunk::ActorModel(_model) => {
                    let actor = stack.top::<Actor>()?;
                    // @todo ❌ Use [Direct ECS World Access](https://bevy-cheatbook.github.io/programming/world.html) here.
                    actor.model = Box::new(Model::default()); //Models::find(model.identifier); // World::query<Model>?
                }
                Chunk::ActorTransform(_) => {
                    // We should just pop transform and attach it to the actor on stack
                    let transform = stack.pop::<Transform>()?;
                    let actor = stack.top::<Actor>()?;
                    actor.transform = *transform;
                }
                Chunk::ActorMaterial(_material) => {
                    let actor = stack.top::<Actor>()?;
                    // @todo ❌ Use [Direct ECS World Access](https://bevy-cheatbook.github.io/programming/world.html) here.
                    actor.material = (); // Materials::find(material.identifier); // World::query<Material>?
                }
                Chunk::ActorLight(_) => {
                    // We should just pop light and attach it to the actor on stack
                    let light = stack.pop::<LightChunk>()?;
                    let actor = stack.top::<Actor>()?;
                    actor.data = ActorData::Light(light);
                }
                Chunk::ActorCamera(_) => {
                    // We should just pop camera and attach it to the actor on stack
                    let camera = stack.pop::<CameraChunk>()?;
                    let actor = stack.top::<Actor>()?;
                    actor.data = ActorData::Camera(camera);
                }
                Chunk::ActorBounds(_) => {
                    // We should just pop bounds and attach it to the actor on stack
                    let bounds = stack.pop::<BoundsChunk>()?;
                    let actor = stack.top::<Actor>()?;
                    actor.data = ActorData::Bounds(bounds);
                }
                Chunk::ActorClipPlane(_) => {
                    // We should just pop clip plane and attach it to the actor on stack
                    let plane = stack.pop::<PlaneChunk>()?;
                    let actor = stack.top::<Actor>()?;
                    actor.data = ActorData::ClipPlane(plane);
                }
                Chunk::ActorAddChild(_) => {
                    // @todo We should just pop actor and attach it to the actor on stack
                    let child = stack.pop::<Actor>()?;
                    let actor = stack.top::<Actor>()?;
                    actor.children.push(child);
                }

                Chunk::TransformMatrix34(transform) => {
                    stack.push(Box::new(Transform::Matrix34(transform)));
                }
                Chunk::TransformMatrix34LP(transform) => {
                    stack.push(Box::new(Transform::Matrix34LP(transform)));
                }
                Chunk::TransformQuat(transform) => {
                    stack.push(Box::new(Transform::Quat(transform)));
                }
                Chunk::TransformEuler(transform) => {
                    stack.push(Box::new(Transform::Euler(transform)));
                }
                Chunk::TransformLookUp(transform) => {
                    stack.push(Box::new(Transform::LookUp(transform)));
                }
                Chunk::TransformTranslation(transform) => {
                    stack.push(Box::new(Transform::Translation(transform)));
                }
                Chunk::TransformIdentity() => {
                    stack.push(Box::new(Transform::Identity));
                }

                Chunk::Bounds(bounds) => {
                    stack.push(Box::new(bounds));
                }
                Chunk::Light(light) => {
                    stack.push(Box::new(light));
                }
                Chunk::Camera(camera) => {
                    stack.push(Box::new(camera));
                }
                Chunk::Plane(plane) => {
                    stack.push(Box::new(plane));
                }

                _ => unimplemented!(), // unexpected type here
            }
        }

        let actor = stack.pop::<Actor>()?;
        actor
    }
}
