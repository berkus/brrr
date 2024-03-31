use {
    super::resource::{
        Chunk, FileInfoChunk, FromStream, LoadMany, PixelMapChunk, PixelsChunk, ResourceTag,
    },
    crate::support,
    bevy::log::debug,
    byteorder::ReadBytesExt,
    carma_derive::ResourceTag,
    culpa::{throw, throws},
    log::trace,
    std::{
        fs::File,
        io::{prelude::BufRead, BufReader},
    },
    support::brender::resource::file_type,
};

// Pixmap consists of two chunks: name and data
// @todo ❌ use SharedData for pixmap contents to avoid copying.
#[derive(Default, Clone, ResourceTag)]
pub struct PixelMap {
    pub name: String,
    pub width: u16,
    pub height: u16,
    pub origin_x: u16,
    pub origin_y: u16,
    // pub r#type: u8, // pixelmap_type::
    // pub row_bytes: u16,
    pub units: u32,
    pub unit_bytes: u32,
    pub data: Vec<u8>, // temp pub
}

impl FromStream for PixelMap {
    type Output = Box<PixelMap>;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut pixelmap = Box::new(PixelMap::default());

        // Read chunks until last chunk is encountered.
        // Certain chunks initialize certain properties.
        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::PIXELMAP {
                        throw!(
                            support::Error::InvalidResourceType /*{
                                                                expected: file_type::PIXELMAP,
                                                                received: file_type,
                                                                }*/
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
                    pixelmap.name = identifier.clone();
                    pixelmap.width = width;
                    pixelmap.height = height;
                    pixelmap.origin_x = origin_x;
                    pixelmap.origin_y = origin_y;

                    trace!(
                        "Pixelmap {} (type {}, row_bytes {}, {}x{} origin {}x{})",
                        identifier,
                        r#type,
                        row_bytes,
                        width,
                        height,
                        origin_x,
                        origin_y
                    );
                }
                Chunk::Pixels(PixelsChunk {
                    units,
                    unit_bytes,
                    data,
                }) => {
                    pixelmap.units = units;
                    pixelmap.unit_bytes = unit_bytes;
                    pixelmap.data = data;

                    trace!(
                        "Pixelmap data in {} units, {} bytes each",
                        units,
                        unit_bytes
                    );
                }
                Chunk::AddMap() => {}
                _ => unimplemented!(), // unexpected type for a pixelmap file
            }
        }

        pixelmap
    }
}

/// Load one or more named textures from a single file
impl LoadMany for PixelMap {
    type Outputs = Vec<Box<PixelMap>>;

    #[throws(support::Error)]
    fn load_many<P: AsRef<std::path::Path> + std::fmt::Debug>(filename: P) -> Self::Outputs {
        debug!("Loading many PixelMaps from {:?}", filename);
        let mut file = BufReader::new(File::open(filename)?);
        let mut maps = Vec::<_>::new();
        loop {
            let m = PixelMap::from_stream(&mut file);
            match m {
                Err(_) => break, // fixme: allow only Eof here
                Ok(m) => {
                    trace!(".. Loaded {}", m.name);
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
            "{} ({}x{}, origin {}x{}) in {} units of {} bytes each",
            self.name,
            self.width,
            self.height,
            self.origin_x,
            self.origin_y,
            self.units,
            self.unit_bytes
        )
    }
}
