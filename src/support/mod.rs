//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
#![allow(dead_code)]

use {
    anyhow::Result,
    cgmath::Vector3,
    glium::implement_vertex,
    std::{
        ops::Sub,
        path::{Path, PathBuf},
        thread,
        time::{Duration, Instant},
    },
    thiserror::Error as ThisError,
};

pub mod brender;
pub mod camera;
pub mod car;
pub mod logger;
pub mod render_manager;
pub mod texture;
pub mod visitor;

// Vertex and VertexUV in resource.rs
#[derive(Copy, Clone, Default)]
pub struct Vertex {
    pub position: [f32; 3],   // x,y,z
    pub normal: [f32; 3],     // x,y,z
    pub tex_coords: [f32; 2], // u,v
}

implement_vertex!(Vertex, position, normal, tex_coords); // @fixme ❌ glium-specific - use bevy_render

// This is used only for vector math, using positions
// Not a general implementation - @todo replace with sub fun
impl Sub for Vertex {
    type Output = Vector3<f32>;

    fn sub(self, other: Vertex) -> Vector3<f32> {
        Vector3 {
            x: self.position[0] - other.position[0],
            y: self.position[1] - other.position[1],
            z: self.position[2] - other.position[2],
        }
    }
}

#[derive(Debug, ThisError)]
pub enum Error {
    #[error("i/o error {0:?}")]
    IO(#[from] std::io::Error),
    #[error("utf-8 conversion error {0:?}")]
    Utf8(#[from] std::str::Utf8Error),
    #[error("from utf-8 conversion error {0:?}")]
    FromUtf8(#[from] std::string::FromUtf8Error),
    #[error("image i/o error {0:?}")]
    ImageIO(#[from] image::ImageError),
    #[error("Resource type is invalid, expected {expected} but got {received}")]
    InvalidResourceType { expected: u32, received: u32 },
    #[error("Resource format is invalid")]
    InvalidResourceFormat,
    #[error("Resource stack is empty")]
    EmptyStack,
}

pub enum Action {
    Stop,
    Continue,
}

// @fixme ❌ drop this fn
pub fn start_loop<F>(mut callback: F)
where
    F: FnMut() -> Action,
{
    let mut accumulator = Duration::new(0, 0);
    let mut previous_clock = Instant::now();

    loop {
        match callback() {
            Action::Stop => break,
            Action::Continue => (),
        };

        let now = Instant::now();
        accumulator += now - previous_clock;
        previous_clock = now;

        let fixed_time_stamp = Duration::new(0, 16666667); // 16ms for 60 FPS
        while accumulator >= fixed_time_stamp {
            accumulator -= fixed_time_stamp;

            // if you have a game, update the state here
        }

        thread::sleep(fixed_time_stamp - accumulator);
    }
}

/*
 * Creates a pathname to filepath with the last directory replaced by newdir
 * and optionally changing extension to newext.
 */
pub fn path_subst<P: AsRef<Path>>(
    filepath: P,
    newdir: P,
    newext: Option<String>,
) -> Result<PathBuf> {
    fn inner<P: AsRef<Path>>(filepath: P, newdir: P, newext: Option<String>) -> Result<PathBuf> {
        let fname = filepath.as_ref().file_name();
        let mut dir = filepath.as_ref().to_path_buf();
        if fname.is_some() {
            dir.pop(); // remove file name
        }
        dir.pop(); // remove parent dir
        dir.push(newdir); // replace parent dir
        if let Some(fname) = fname {
            dir.push(fname); // add back file name
        }
        if let Some(ext) = newext {
            dir.set_extension(ext);
        }
        Ok(dir)
    }
    inner(&filepath, &newdir, newext)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_path_subst() {
        assert_eq!(
            PathBuf::from("/path/file.ext2"),
            path_subst(
                &Path::new("/old/file.ext"),
                &Path::new("path"),
                Some(String::from("ext2")),
            )
            .expect("Should be able to subst path")
        );
        assert_eq!(
            PathBuf::from("/path/file.ext"),
            path_subst(&Path::new("/old/file.ext"), &Path::new("path"), None)
                .expect("Should be able to subst path")
        );
    }
}
