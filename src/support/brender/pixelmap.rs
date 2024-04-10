use {
    crate::support::{
        brender::resource::{
            file_type, Chunk, FileInfoChunk, FromStream, LoadMany, NamedResource, PixelMapChunk,
            PixelsChunk, ResourceStack, ResourceTag,
        },
        Error,
    },
    bevy::{log::debug, prelude::*},
    byteorder::ReadBytesExt,
    carma_derive::ResourceTag,
    culpa::{throw, throws},
    log::trace,
    std::{
        fs::File,
        io::{prelude::BufRead, BufReader},
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
                        throw!(
                            Error::InvalidResourceType // {
                                                       //     expected: file_type::PIXELMAP,
                                                       //     received: file_type,
                                                       // }
                        );
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
                        units,
                        unit_bytes
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
