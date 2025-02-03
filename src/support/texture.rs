//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, 2020 Berkus Karchebnyy <berkus+cargo@metta.systems>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{brender::pixelmap::PixelMap, Error},
    bevy::prelude::*,
    std::io::Write,
};

impl PixelMap {
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
            image::ExtendedColorType::Rgb8,
        )?;
        Ok(())
    }

    fn dump(&self) {
        info!("Pixelmap {}", self);
    }
}
