use {
    super::resource::pixelmap_type,
    crate::support::{
        Error,
        brender::resource::{
            Chunk, FileInfoChunk, FromStream, LoadMany, NamedResource, PixelMapChunk, PixelsChunk,
            ResourceStack, ResourceTag, file_type,
        },
    },
    bevy::{
        log::debug,
        prelude::*,
        render::{
            render_asset::RenderAssetUsages,
            render_resource::{Extent3d, TextureDimension, TextureFormat},
        },
        utils::HashMap,
    },
    brrr_derive::ResourceTag,
    byteorder::ReadBytesExt,
    culpa::{throw, throws},
    log::trace,
    std::{
        fs::File,
        io::{BufReader, prelude::BufRead},
    },
};

// Pixmap consists of two chunks: name and data
// @todo ❌ use SharedData for pixmap contents to avoid copying.
#[derive(Default, Clone, ResourceTag)]
pub struct PixelMap {
    pub identifier: String,
    pub r#type: u8, // pixelmap_type::
    pub width: u16,
    pub height: u16,
    pub origin_x: u16,
    pub origin_y: u16,
    pub row_bytes: u16,
    pub units: u32,
    pub unit_bytes: u32,
    pub data: Vec<u8>, // temp pub
}

impl NamedResource for PixelMap {
    fn resource_name(&self) -> String {
        self.identifier.clone()
    }
}

impl From<PixelMap> for bevy::prelude::Image {
    fn from(value: PixelMap) -> Self {
        Self::new(
            Extent3d {
                width: value.width.into(),
                height: value.height.into(),
                depth_or_array_layers: 1,
            },
            TextureDimension::D2,
            value.data,
            PixelMapType(value.r#type).into(),
            RenderAssetUsages::default(),
        )
    }
}

impl FromStream for PixelMap {
    type Output = Box<PixelMap>;

    #[throws(Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut stack = ResourceStack::new();

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::PIXELMAP {
                        throw!(Error::InvalidFileType {
                            expected: file_type::PIXELMAP,
                            received: file_type,
                        });
                    }
                }
                Chunk::PixelMap(PixelMapChunk {
                    identifier,
                    width,
                    height,
                    origin_x,
                    origin_y,
                    r#type,
                    row_bytes,
                }) => {
                    let pixelmap = PixelMap {
                        identifier,
                        r#type,
                        row_bytes,
                        width,
                        height,
                        origin_x,
                        origin_y,
                        ..default()
                    };
                    trace!("Pixelmap {}", &pixelmap);
                    stack.push(Box::new(pixelmap));
                }
                Chunk::Pixels(PixelsChunk {
                    units,
                    unit_bytes,
                    data,
                }) => {
                    let pixelmap = stack.top::<PixelMap>()?;
                    pixelmap.units = units;
                    pixelmap.unit_bytes = unit_bytes;
                    pixelmap.data = data;

                    trace!(
                        "Pixelmap data in {} units, {} bytes each",
                        units, unit_bytes
                    );
                }
                Chunk::AddMap() => {
                    todo!()
                }
                _ => unimplemented!(), // unexpected type for a pixelmap file
            }
        }

        stack.pop::<PixelMap>()?
    }
}

/// Load one or more named textures from a single file
impl LoadMany for PixelMap {
    type Outputs = Box<PixelMap>;

    #[throws(Error)]
    fn load_many<P: AsRef<std::path::Path> + std::fmt::Debug>(filename: P) -> Vec<Self::Outputs> {
        debug!("Loading many PixelMaps from {:?}", filename);
        let mut file = BufReader::new(File::open(filename)?);
        let mut maps = Vec::<_>::new();
        loop {
            let m = PixelMap::from_stream(&mut file);
            match m {
                Err(_) => break, // fixme: allow only Eof here
                Ok(m) => {
                    trace!(".. Loaded {}", m.identifier);
                    maps.push(m)
                }
            }
        }
        maps
    }
}

impl std::fmt::Display for PixelMap {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{} type {} ({}x{}, stride {} bytes, origin {}x{}) in {} units of {} bytes each",
            self.identifier,
            self.r#type,
            self.width,
            self.height,
            self.origin_x,
            self.origin_y,
            self.row_bytes,
            self.units,
            self.unit_bytes
        )
    }
}

// impl PixelMap {
//     // Interpret pixelmap payload based on the r#type and return RGBA representation.
//     // @todo needs an iterator?
//     // Use imgref's iterators and rgb's transparent bytes interpretation.
//     pub fn remap(&self, palette: &PixelMap) -> PixelMap {
//         match self.r#type {
//             pixelmap_type::INDEX_1 => unimplemented!(),
//             pixelmap_type::INDEX_2 => unimplemented!(),
//             pixelmap_type::INDEX_4 => unimplemented!(),
//             pixelmap_type::INDEX_8 => (), // palette indexed bytes? the rest can be ignored for now
//             pixelmap_type::RGB_555 => unimplemented!(),
//             pixelmap_type::RGB_565 => unimplemented!(),
//             pixelmap_type::RGB_888 => unimplemented!(),
//             pixelmap_type::RGBX_888 => unimplemented!(),
//             pixelmap_type::RGBA_888 => unimplemented!(),
//             pixelmap_type::YUYV_8888 => unimplemented!(),
//             pixelmap_type::YUV_888 => unimplemented!(),
//             pixelmap_type::DEPTH_16 => unimplemented!(),
//             pixelmap_type::DEPTH_32 => unimplemented!(),
//             pixelmap_type::ALPHA_8 => unimplemented!(),
//             pixelmap_type::INDEXA_88 => (), // what's that, indexed + alpha?
//             _ => unimplemented!(),
//         }
//     }
// }

struct PixelMapType(u8);

impl From<PixelMapType> for TextureFormat {
    fn from(value: PixelMapType) -> Self {
        match value.0 {
            pixelmap_type::INDEX_1 => unimplemented!(),
            pixelmap_type::INDEX_2 => unimplemented!(),
            pixelmap_type::INDEX_4 => unimplemented!(),
            pixelmap_type::INDEX_8 => unimplemented!(),
            pixelmap_type::RGB_555 => unimplemented!(),
            pixelmap_type::RGB_565 => unimplemented!(),
            pixelmap_type::RGB_888 => unimplemented!(),
            pixelmap_type::RGBX_888 => unimplemented!(),
            pixelmap_type::RGBA_888 => TextureFormat::Rgba8Uint,
            pixelmap_type::YUYV_8888 => unimplemented!(),
            pixelmap_type::YUV_888 => unimplemented!(),
            pixelmap_type::DEPTH_16 => TextureFormat::Depth16Unorm,
            pixelmap_type::DEPTH_32 => TextureFormat::Depth32Float,
            pixelmap_type::ALPHA_8 => unimplemented!(),
            pixelmap_type::INDEXA_88 => unimplemented!(),
            _ => unimplemented!(),
        }
    }
}

/// Convert indexed-color image to RGBA using provided palette.
///
/// `Palette = shade tab` in BRender parlance.
#[throws(Error)]
pub fn remap_via_palette(
    color_map_name: &str,
    index_shade_name: &str,
    imgs: &HashMap<String, Box<PixelMap>>,
) -> PixelMap {
    let Some(color_map) = imgs.get(color_map_name.into()) else {
        throw!(Error::MissingPixelMap {
            pm_name: color_map_name.into()
        })
    };
    let Some(palette) = imgs.get(index_shade_name.into()) else {
        throw!(Error::MissingPixelMap {
            pm_name: index_shade_name.into()
        })
    };
    let mut output = PixelMap::default();
    output.unit_bytes = 4;
    output.r#type = pixelmap_type::RGBA_888;
    output.data = Vec::<_>::with_capacity(color_map.data.len() * 4);

    for i in 0..color_map.units {
        // @fixme use color index 0 as transparency
        if color_map.data[i as usize] == 0 {
            output.data.push(0); // R
            output.data.push(0); // G
            output.data.push(0); // B
            output.data.push(255); // A = transparent
        } else {
            output.data.push(
                palette.data[(color_map.data[i as usize] as u32 * palette.unit_bytes + 1) as usize],
            ); // R
            output.data.push(
                palette.data[(color_map.data[i as usize] as u32 * palette.unit_bytes + 2) as usize],
            ); // G
            output.data.push(
                palette.data[(color_map.data[i as usize] as u32 * palette.unit_bytes + 3) as usize],
            ); // B
            output.data.push(
                255 - palette.data
                    [(color_map.data[i as usize] as u32 * palette.unit_bytes/* + 0*/) as usize],
            ); // A
            if color_map.identifier == "BGLSPIKE.PIX" {
                trace!("spike alpha {}", output.data.last().unwrap());
            }
        }
    }

    output
}
