//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, 2020 Berkus Karchebnyy <berkus+cargo@metta.systems>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{
        self,
        brender::{pixelmap::PixelMap, resource::FromStream},
        Error,
    },
    bevy::prelude::*,
    culpa::throws,
    std::{
        fs::File,
        io::{BufReader, Write},
    },
};

// @todo ❌ impl From<PixelMap> for bevy::Texture {}

/**
* Megatexture for storing all loaded textures.
* Usually 1024x1024 or 4096x4096 texture with multiple smaller textures inside.

@todo ❌ drop local Texture and use bevy::TextureAtlas
*/
#[derive(Default)]
pub struct Texture {
    // @todo ❌ use bevy::prelude::TextureAtlas
    pub w: u16,
    pub h: u16,
    pub data: Vec<u8>,
}

/**
* Named reference into the megatexture.

@todo ❌ use bevy Handle<Texture> or sth like this to get the texture out of the atlas
We need only u and v for the texture because it's all already part of the megatexture.
*/
pub struct TextureReference {
    pub id: i32,
    pub x0: f32,
    pub y0: f32,
    pub x1: f32,
    pub y1: f32,
    pub name: String,
}

impl PixelMap {
    /// Convert indexed-color image to RGBA using provided palette.
    ///
    /// `Palette = shade tab` in BRender parlance.
    #[throws(support::Error)]
    pub fn remap_via_palette(&self, palette: &PixelMap) -> PixelMap {
        let mut pm = self.clone();
        pm.data = Vec::<u8>::with_capacity(self.data.len() * 4);
        pm.unit_bytes = 4;

        for i in 0..pm.units {
            // @fixme use color index 0 as transparency
            if self.data[i as usize] == 0 {
                pm.data.push(0); // R
                pm.data.push(0); // G
                pm.data.push(0); // B
                pm.data.push(255); // A = transparent
            } else {
                pm.data.push(
                    palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 1) as usize],
                ); // R
                pm.data.push(
                    palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 2) as usize],
                ); // G
                pm.data.push(
                    palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 3) as usize],
                ); // B
                pm.data.push(
                    255 - palette.data
                        [(self.data[i as usize] as u32 * palette.unit_bytes/* + 0*/) as usize],
                ); // A
                if self.name == "BGLSPIKE.PIX" {
                    trace!("spike alpha {}", pm.data.last().unwrap());
                }
            }
        }

        pm
    }

    #[allow(clippy::identity_op)] // keep +0 in formulas
    pub fn write_png_remapped_via<W: Write>(
        &self,
        palette: &PixelMap,
        w: &mut W,
    ) -> Result<(), Error> {
        self.dump();

        let mut data = Vec::<u8>::with_capacity(self.data.len() * 4);

        match self.unit_bytes {
            1 => {
                for i in 0..self.units {
                    data.push(
                        palette.data
                            [(self.data[i as usize] as u32 * palette.unit_bytes + 1) as usize],
                    ); // R
                    data.push(
                        palette.data
                            [(self.data[i as usize] as u32 * palette.unit_bytes + 2) as usize],
                    ); // G
                    data.push(
                        palette.data
                            [(self.data[i as usize] as u32 * palette.unit_bytes + 3) as usize],
                    ); // B
                       // data.push(
                       // 255-palette.data[(self.data[i as usize] as u32 * palette.unit_bytes + 0) as
                       // usize],
                       // ); // A
                }
            }
            3 => {
                for i in 0..self.units {
                    data.push(self.data[(i * self.unit_bytes + 0) as usize]); // R
                    data.push(self.data[(i * self.unit_bytes + 1) as usize]); // G
                    data.push(self.data[(i * self.unit_bytes + 2) as usize]); // B
                                                                              // data.push(255); // A
                }
            }
            4 => {
                for i in 0..self.units {
                    data.push(self.data[(i * self.unit_bytes + 0) as usize]); // R
                    data.push(self.data[(i * self.unit_bytes + 1) as usize]); // G
                    data.push(self.data[(i * self.unit_bytes + 2) as usize]); // B
                                                                              // data.push(self.data[(i * self.unit_bytes + 3) as usize]); // A
                }
            }
            _ => unimplemented!(),
        }

        use image::ImageEncoder;
        let png = image::codecs::png::PngEncoder::new(w);
        png.write_image(
            &data,
            self.width.into(),
            self.height.into(),
            image::ColorType::Rgb8,
        )?;
        Ok(())
    }

    fn dump(&self) {
        info!("Pixelmap {}", self);
    }
}

// @todo ❌ Use bevy::Texture directly?
impl Texture {
    pub fn new() -> Texture {
        Texture {
            w: 1024,
            h: 1024,
            data: Vec::new(),
        }
    }
}
