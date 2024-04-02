//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{self, brender::read_c_string, Error},
    bevy::prelude::*,
    byteorder::{BigEndian, ReadBytesExt},
    carma_derive::ResourceTag,
    core::any::Any,
    culpa::{throw, throws},
    std::io::BufRead,
};

//------------------------------------------------------------------
/// Read resource from a stream.
pub trait FromStream {
    type Output;
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Result<Self::Output, Error>;
}

//------------------------------------------------------------------
// Read an array resource with a specified count from a stream.
// pub trait FromStreamExt {
//     type Output;
//     fn from_stream_ext<S: ReadBytesExt + BufRead>(
//         source: &mut S,
//         count: usize,
//     ) -> Result<Self::Output, Error>;
// }

//------------------------------------------------------------------
/// Read an array of resources from a specified file.
/// Resources must be named to distinguish them from each other.
pub trait LoadMany {
    type Outputs: NamedResource;
    fn load_many<P: AsRef<std::path::Path> + std::fmt::Debug>(
        filename: P,
    ) -> Result<Vec<Self::Outputs>, Error>;
}

//------------------------------------------------------------------
/// Chunk type wrapper for debugging.
struct ChunkType(u32);

impl std::fmt::Display for ChunkType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self.0 {
                chunk::END => "END",
                chunk::PIXELMAP => "PIXELMAP",
                chunk::MATERIAL => "MATERIAL",
                chunk::ANIM => "ANIM",
                chunk::ANIM_TRANSFORM => "ANIM_TRANSFORM",
                chunk::ANIM_RATE => "ANIM_RATE",
                chunk::FILE_INFO => "FILE_INFO",
                chunk::PIVOT => "PIVOT",
                chunk::MATERIAL_INDEX => "MATERIAL_INDEX",
                chunk::VERTICES => "VERTICES",
                chunk::VERTEX_UV => "VERTEX_UV",
                chunk::FACE_MATERIAL => "FACE_MATERIAL",
                chunk::COLOUR_MAP_REF => "COLOUR_MAP_REF",
                chunk::INDEX_BLEND_REF => "INDEX_BLEND_REF",
                chunk::INDEX_SHADE_REF => "INDEX_SHADE_REF",
                chunk::SCREENDOOR_REF => "SCREENDOOR_REF",
                chunk::PIXELS => "PIXELS",
                chunk::ADD_MAP => "ADD_MAP",
                chunk::ACTOR => "ACTOR",
                chunk::ACTOR_MODEL => "ACTOR_MODEL",
                chunk::ACTOR_TRANSFORM => "ACTOR_TRANSFORM",
                chunk::ACTOR_MATERIAL => "ACTOR_MATERIAL",
                chunk::ACTOR_LIGHT => "ACTOR_LIGHT",
                chunk::ACTOR_CAMERA => "ACTOR_CAMERA",
                chunk::ACTOR_BOUNDS => "ACTOR_BOUNDS",
                chunk::ACTOR_ADD_CHILD => "ACTOR_ADD_CHILD",
                chunk::TRANSFORM_MATRIX34 => "TRANSFORM_MATRIX34",
                chunk::TRANSFORM_MATRIX34_LP => "TRANSFORM_MATRIX34_LP",
                chunk::TRANSFORM_QUAT => "TRANSFORM_QUAT",
                chunk::TRANSFORM_EULER => "TRANSFORM_EULER",
                chunk::TRANSFORM_LOOK_UP => "TRANSFORM_LOOK_UP",
                chunk::TRANSFORM_TRANSLATION => "TRANSFORM_TRANSLATION",
                chunk::TRANSFORM_IDENTITY => "TRANSFORM_IDENTITY",
                chunk::BOUNDS => "BOUNDS",
                chunk::LIGHT => "LIGHT",
                chunk::CAMERA => "CAMERA",
                chunk::FACES => "FACES",
                chunk::MODEL => "MODEL",
                chunk::ACTOR_CLIP_PLANE => "ACTOR_CLIP_PLANE",
                chunk::PLANE => "PLANE",
                _ => "UNKNOWN",
            }
        )
    }
}

//------------------------------------------------------------------
/// A binary resource file consisting of chunks with specific size.
/// Reading from such file yields array of chunk results, some of
/// these chunks are service, some are useful to the client.
#[derive(Default)]
struct ChunkHeader {
    chunk_type: u32,
    /// size of chunk without the header
    size: u32,
}

impl FromStream for ChunkHeader {
    type Output = ChunkHeader;
    #[throws(support::Error)]
    fn from_stream<R: ReadBytesExt>(source: &mut R) -> Self::Output {
        let chunk_type = source.read_u32::<BigEndian>()?;
        let size = source.read_u32::<BigEndian>()?;
        trace!(
            "Loaded chunk type {chunk_type}/0x{:x} {} size {}",
            chunk_type,
            ChunkType(chunk_type),
            size
        );
        Self::Output { chunk_type, size }
    }
}

//------------------------------------------------------------------
/// A by-name reference.
pub struct NameRefChunk {
    pub identifier: String,
}

impl FromStream for NameRefChunk {
    type Output = NameRefChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let identifier = read_c_string(source)?;
        trace!("... {}", identifier);
        Self::Output { identifier }
    }
}

//------------------------------------------------------------------
/// Chunk types.
pub mod chunk {
    pub const END: u32 = 0x0;
    pub const PIXELMAP: u32 = 0x3;
    pub const MATERIAL: u32 = 0x4;
    pub const ANIM: u32 = 0xf;
    pub const ANIM_TRANSFORM: u32 = 0x10;
    pub const ANIM_RATE: u32 = 0x11;
    pub const FILE_INFO: u32 = 0x12;
    pub const PIVOT: u32 = 0x15;
    pub const MATERIAL_INDEX: u32 = 0x16;
    pub const VERTICES: u32 = 0x17;
    pub const VERTEX_UV: u32 = 0x18;
    pub const FACE_MATERIAL: u32 = 0x1a;
    pub const COLOUR_MAP_REF: u32 = 0x1c;
    pub const INDEX_BLEND_REF: u32 = 0x1e;
    pub const INDEX_SHADE_REF: u32 = 0x1f; // RENDERTAB_REF
    pub const SCREENDOOR_REF: u32 = 0x20;
    pub const PIXELS: u32 = 0x21;
    pub const ADD_MAP: u32 = 0x22; // Connect a map to an indexed pixelmap
    pub const ACTOR: u32 = 0x23;
    pub const ACTOR_MODEL: u32 = 0x24;
    pub const ACTOR_TRANSFORM: u32 = 0x25;
    pub const ACTOR_MATERIAL: u32 = 0x26;
    pub const ACTOR_LIGHT: u32 = 0x27;
    pub const ACTOR_CAMERA: u32 = 0x28;
    pub const ACTOR_BOUNDS: u32 = 0x29;
    pub const ACTOR_ADD_CHILD: u32 = 0x2a;
    pub const TRANSFORM_MATRIX34: u32 = 0x2b;
    pub const TRANSFORM_MATRIX34_LP: u32 = 0x2c;
    pub const TRANSFORM_QUAT: u32 = 0x2d;
    pub const TRANSFORM_EULER: u32 = 0x2e;
    pub const TRANSFORM_LOOK_UP: u32 = 0x2f;
    pub const TRANSFORM_TRANSLATION: u32 = 0x30;
    pub const TRANSFORM_IDENTITY: u32 = 0x31;
    pub const BOUNDS: u32 = 0x32;
    pub const LIGHT: u32 = 0x33;
    pub const CAMERA: u32 = 0x34;
    pub const FACES: u32 = 0x35;
    pub const MODEL: u32 = 0x36;
    pub const ACTOR_CLIP_PLANE: u32 = 0x37;
    pub const PLANE: u32 = 0x38;
}

//------------------------------------------------------------------
/// Types for FILE_INFO chunk.
pub mod file_type {
    pub const NONE: u32 = 0x0;
    pub const ACTOR: u32 = 0x1;
    pub const PIXELMAP: u32 = 0x2;
    pub const LIGHT: u32 = 0x3;
    pub const CAMERA: u32 = 0x4;
    pub const MATERIAL: u32 = 0x5;
    pub const MODEL: u32 = 0xface;
    pub const ANIM: u32 = 0x0a11;
    pub const TREE: u32 = 0x5eed;
}

//------------------------------------------------------------------
/// Types for ACTOR chunk.
pub mod actor_type {
    pub const NONE: u32 = 0x0;
    pub const MODEL: u32 = 0x1;
    pub const LIGHT: u32 = 0x2;
    pub const CAMERA: u32 = 0x3;
    pub const BOUNDS: u32 = 0x5;
    pub const BOUNDS_CORRECT: u32 = 0x6;
    pub const CLIP_PLANE: u32 = 0x7;
}

//------------------------------------------------------------------
/// Rendering style for ACTOR chunk.
pub mod actor_render_style {
    pub const DEFAULT: u32 = 0x0;
    pub const NONE: u32 = 0x1;
    pub const POINTS: u32 = 0x2;
    pub const EDGES: u32 = 0x3;
    pub const FACES: u32 = 0x4;
    pub const BOUNDING_POINTS: u32 = 0x5;
    pub const BOUNDING_EDGES: u32 = 0x6;
    pub const BOUNDING_FACES: u32 = 0x7;
}

//------------------------------------------------------------------
/// Order of rotations in a Euler transform.
pub mod euler_angle_order {
    pub const XYZ_S: u32 = 0x0;
    pub const XYX_S: u32 = 0x1;
    pub const XZY_S: u32 = 0x2;
    pub const XZX_S: u32 = 0x3;
    pub const YZX_S: u32 = 0x4;
    pub const YZY_S: u32 = 0x5;
    pub const YXZ_S: u32 = 0x6;
    pub const YXY_S: u32 = 0x7;
    pub const ZXY_S: u32 = 0x8;
    pub const ZXZ_S: u32 = 0x9;
    pub const ZYX_S: u32 = 0xa;
    pub const ZYZ_S: u32 = 0xb;
    pub const ZYX_R: u32 = 0xc;
    pub const XYX_R: u32 = 0xd;
    pub const YZX_R: u32 = 0xe;
    pub const XZX_R: u32 = 0xf;
    pub const XZY_R: u32 = 0x10;
    pub const YZY_R: u32 = 0x11;
    pub const ZXY_R: u32 = 0x12;
    pub const YXY_R: u32 = 0x13;
    pub const YXZ_R: u32 = 0x14;
    pub const ZXZ_R: u32 = 0x15;
    pub const XYZ_R: u32 = 0x16;
    pub const ZYZ_R: u32 = 0x17;
}

//------------------------------------------------------------------
/// Type for LIGHT chunk.
pub mod light_type {
    pub const POINT: u32 = 0x0;
    pub const DIRECT: u32 = 0x1;
    pub const SPOT: u32 = 0x2;
    pub const VIEW_POINT: u32 = 0x4;
    pub const VIEW_DIRECT: u32 = 0x5;
    pub const VIEW_SPOT: u32 = 0x6;
}

//------------------------------------------------------------------
/// Type for CAMERA chunk.
pub mod camera_type {
    pub const PARALLEL: u32 = 0x0;
    pub const PERSPECTIVE: u32 = 0x1;
}

//------------------------------------------------------------------
/// Type for payload of a PIXELMAP chunk.
pub mod pixelmap_type {
    pub const INDEX_1: u8 = 0x0;
    pub const INDEX_2: u8 = 0x1;
    pub const INDEX_4: u8 = 0x2;
    pub const INDEX_8: u8 = 0x3;
    pub const RGB_555: u8 = 0x4;
    pub const RGB_565: u8 = 0x5;
    pub const RGB_888: u8 = 0x6;
    pub const RGBX_888: u8 = 0x7;
    pub const RGBA_888: u8 = 0x8;
    pub const YUYV_8888: u8 = 0x9;
    pub const YUV_888: u8 = 0xa;
    pub const DEPTH_16: u8 = 0xb;
    pub const DEPTH_32: u8 = 0xc;
    pub const ALPHA_8: u8 = 0xd;
    pub const INDEXA_88: u8 = 0xe;
}

// =================
// Universal chunks:

//------------------------------------------------------------------
pub struct FileInfoChunk {
    pub file_type: u32,
    pub version: u32,
}

impl FromStream for FileInfoChunk {
    type Output = FileInfoChunk;
    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let file_type = source.read_u32::<BigEndian>()?;
        let version = source.read_u32::<BigEndian>()?;
        Self::Output { file_type, version }
    }
}

// =================
// Model chunks: (BrModelLoadMany)

//------------------------------------------------------------------
pub struct ModelChunk {
    pub flags: u16,
    pub identifier: String,
}

impl FromStream for ModelChunk {
    type Output = ModelChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let flags = source.read_u16::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        trace!(".. {identifier}");
        Self::Output { flags, identifier }
    }
}

//------------------------------------------------------------------
pub struct MaterialIndexChunk {
    pub materials: Vec<String>,
}

impl FromStream for MaterialIndexChunk {
    type Output = MaterialIndexChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()? as usize;
        let mut materials = Vec::<String>::with_capacity(entries_count);
        for _ in 0..entries_count {
            let v = read_c_string(source)?;
            trace!("... ref ({})", v);
            materials.push(v);
        }
        Self::Output { materials }
    }
}

//------------------------------------------------------------------
#[derive(Default, Debug)]
pub struct Vec2f {
    x: f32,
    y: f32,
}

impl FromStream for Vec2f {
    type Output = Vec2f;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let x = source.read_f32::<BigEndian>()?;
        let y = source.read_f32::<BigEndian>()?;
        Self::Output { x, y }
    }
}

//------------------------------------------------------------------
pub type Vertex = Vec3f;

#[derive(Default, Debug)]
pub struct Vec3f {
    pub x: f32,
    pub y: f32,
    pub z: f32,
}

impl FromStream for Vec3f {
    type Output = Vec3f;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let v = Vec2f::from_stream(source)?;
        let z = source.read_f32::<BigEndian>()?;
        Self::Output { x: v.x, y: v.y, z }
    }
}

//------------------------------------------------------------------
struct Vec4f {
    x: f32,
    y: f32,
    z: f32,
    w: f32,
}

impl FromStream for Vec4f {
    type Output = Vec4f;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let v = Vec3f::from_stream(source)?;
        let w = source.read_f32::<BigEndian>()?;
        Self::Output {
            x: v.x,
            y: v.y,
            z: v.z,
            w,
        }
    }
}

//------------------------------------------------------------------
pub struct VertexUV {
    pub u: f32,
    pub v: f32,
}

impl FromStream for VertexUV {
    type Output = VertexUV;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let uv = Vec2f::from_stream(source)?;
        Self::Output { u: uv.x, v: uv.y }
    }
}

//------------------------------------------------------------------
pub struct VerticesChunk {
    pub vertices: Vec<Vertex>,
}

impl FromStream for VerticesChunk {
    type Output = VerticesChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()?;
        let mut vertices = Vec::<Vertex>::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            let v = Vertex::from_stream(source)?; // IoVertex::..
            vertices.push(v); // v.0
        }
        Self::Output { vertices }
    }
}

//------------------------------------------------------------------
pub struct VertexUvChunk {
    pub uvs: Vec<VertexUV>,
}

impl FromStream for VertexUvChunk {
    type Output = VertexUvChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()?;
        let mut uvs = Vec::<VertexUV>::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            let v = VertexUV::from_stream(source)?;
            uvs.push(v);
        }
        Self::Output { uvs }
    }
}

//------------------------------------------------------------------
#[derive(Default)]
pub struct Face {
    pub v1: u16,
    pub v2: u16,
    pub v3: u16,
    pub smoothing: u16,
    pub flags: u8,
}

impl FromStream for Face {
    type Output = Face;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let v1 = source.read_u16::<BigEndian>()?;
        let v2 = source.read_u16::<BigEndian>()?;
        let v3 = source.read_u16::<BigEndian>()?;
        let smoothing = source.read_u16::<BigEndian>()?;
        let flags = source.read_u8()?;
        Self::Output {
            v1,
            v2,
            v3,
            smoothing,
            flags,
        }
    }
}

//------------------------------------------------------------------
#[derive(Default)]
pub struct FacesChunk {
    pub faces: Vec<Face>,
}

impl FromStream for FacesChunk {
    type Output = FacesChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entries_count = source.read_u32::<BigEndian>()?;
        trace!(".. {entries_count} entries");
        let mut faces = Vec::with_capacity(entries_count as usize);
        for _ in 0..entries_count {
            let f = Face::from_stream(source)?;
            faces.push(f);
        }
        Self::Output { faces }
    }
}

//------------------------------------------------------------------
pub struct FaceMaterialChunk {
    pub face_material_indices: Vec<u16>,
}

impl FromStream for FaceMaterialChunk {
    type Output = FaceMaterialChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let entry_count = source.read_u32::<BigEndian>()? as usize;
        let entry_size = source.read_u32::<BigEndian>()?;
        trace!(".. {entry_count} entries, {entry_size} bytes each");
        assert!(entry_size == 2); // We don't support reading other variants
        if entry_size != 2 {
            throw!(Error::InvalidResourceFormat);
        }

        let mut face_material_indices = Vec::with_capacity(entry_count);
        for _ in 0..entry_count {
            let index = source.read_u16::<BigEndian>()?;
            face_material_indices.push(index);
        }
        Self::Output {
            face_material_indices,
        }
    }
}

//------------------------------------------------------------------
pub struct PivotChunk {
    pub pivot: Vec3f,
}

impl FromStream for PivotChunk {
    type Output = PivotChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let pivot = Vec3f::from_stream(source)?;
        Self::Output { pivot }
    }
}

//------------------------------------------------------------------
#[derive(Default, Debug)]
pub struct Colour {
    pub r: f32,
    pub g: f32,
    pub b: f32,
}

impl FromStream for Colour {
    type Output = Colour;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let color = Vec3f::from_stream(source)?;
        Self::Output {
            r: color.x,
            g: color.y,
            b: color.z,
        }
    }
}

impl From<Rgb> for Colour {
    fn from(value: Rgb) -> Self {
        Self {
            r: value.r as f32 / 255.0,
            g: value.g as f32 / 255.0,
            b: value.b as f32 / 255.0,
        }
    }
}

#[derive(Default, Debug)]
pub struct Rgb {
    pub r: u8,
    pub g: u8,
    pub b: u8,
}

impl FromStream for Rgb {
    type Output = Rgb;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let r = source.read_u8()?;
        let g = source.read_u8()?;
        let b = source.read_u8()?;
        Self::Output { r, g, b }
    }
}

// =================
// Material chunks: (BrMaterialLoadMany)

pub mod material_flags {
    pub const LIGHT: u16 = 0x0001;
    pub const PRELIT: u16 = 0x0002;

    pub const SMOOTH: u16 = 0x0004;

    pub const ENVIRONMENT_I: u16 = 0x0008;
    pub const ENVIRONMENT_L: u16 = 0x0010;
    pub const PERSPECTIVE: u16 = 0x0020;
    pub const DECAL: u16 = 0x0040;

    pub const I_FROM_U: u16 = 0x0080;
    pub const I_FROM_V: u16 = 0x0100;
    pub const U_FROM_I: u16 = 0x0200;
    pub const V_FROM_I: u16 = 0x0400;

    pub const ALWAYS_VISIBLE: u16 = 0x0800;
    pub const TWO_SIDED: u16 = 0x1000;

    pub const FORCE_Z_0: u16 = 0x2000;

    pub const DITHER: u16 = 0x4000;
    pub const CUSTOM: u16 = 0x8000;
}

#[derive(Default)]
pub struct MaterialFlags(u16);

impl std::fmt::Debug for MaterialFlags {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}]",
            if self.0 & material_flags::LIGHT != 0 {
                "LIGHT "
            } else {
                ""
            },
            if self.0 & material_flags::PRELIT != 0 {
                "PRELIT "
            } else {
                ""
            },
            if self.0 & material_flags::SMOOTH != 0 {
                "SMOOTH "
            } else {
                ""
            },
            if self.0 & material_flags::ENVIRONMENT_I != 0 {
                "ENVIRONMENT_I "
            } else {
                ""
            },
            if self.0 & material_flags::ENVIRONMENT_L != 0 {
                "ENVIRONMENT_L "
            } else {
                ""
            },
            if self.0 & material_flags::PERSPECTIVE != 0 {
                "PERSPECTIVE "
            } else {
                ""
            },
            if self.0 & material_flags::DECAL != 0 {
                "DECAL "
            } else {
                ""
            },
            if self.0 & material_flags::I_FROM_U != 0 {
                "I_FROM_U "
            } else {
                ""
            },
            if self.0 & material_flags::I_FROM_V != 0 {
                "I_FROM_V "
            } else {
                ""
            },
            if self.0 & material_flags::U_FROM_I != 0 {
                "U_FROM_I "
            } else {
                ""
            },
            if self.0 & material_flags::V_FROM_I != 0 {
                "V_FROM_I "
            } else {
                ""
            },
            if self.0 & material_flags::ALWAYS_VISIBLE != 0 {
                "ALWAYS_VISIBLE "
            } else {
                ""
            },
            if self.0 & material_flags::TWO_SIDED != 0 {
                "TWO_SIDED "
            } else {
                ""
            },
            if self.0 & material_flags::FORCE_Z_0 != 0 {
                "FORCE_Z_0 "
            } else {
                ""
            },
            if self.0 & material_flags::DITHER != 0 {
                "DITHER "
            } else {
                ""
            },
            if self.0 & material_flags::CUSTOM != 0 {
                "CUSTOM "
            } else {
                ""
            },
        )
    }
}

//------------------------------------------------------------------
#[derive(Default, Debug)]
pub struct MaterialChunk {
    pub color: Colour,
    pub opacity: u8,
    pub ka: f32,
    pub kd: f32,
    pub ks: f32,
    pub power: f32,
    pub flags: MaterialFlags,
    pub map_transform_x: Vec2f,
    pub map_transform_y: Vec2f,
    pub map_transform_z: Vec2f,
    pub index_base: u8,
    pub index_range: u8,
    pub identifier: String,
    // _COLOUR(colour), // 4 bytes in calc, 3 bytes actually written
    // _UINT_8(opacity), // 1 byte
    // _UFRACTION(ka), // 4 bytes
    // _UFRACTION(kd), // 4 bytes
    // _UFRACTION(ks), // 4 bytes
    // _SCALAR(power), // 4 bytes
    // _UINT_16(flags), // 2 bytes
    // _VECTOR2(map_transform.m[0]), // 8 bytes
    // _VECTOR2(map_transform.m[1]), // 8 bytes
    // _VECTOR2(map_transform.m[2]), // 8 bytes
    // _UINT_8(index_base), // 1 byte
    // _UINT_8(index_range), // 1 byte
    // _ASCIZ(identifier), // string length + 1 bytes
}

impl FromStream for MaterialChunk {
    type Output = MaterialChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let color = Rgb::from_stream(source)?.into();
        let opacity = source.read_u8()?;
        let ka = source.read_f32::<BigEndian>()?;
        let kd = source.read_f32::<BigEndian>()?;
        let ks = source.read_f32::<BigEndian>()?;
        let power = source.read_f32::<BigEndian>()?;
        let flags = MaterialFlags(source.read_u16::<BigEndian>()?);
        let map_transform_x = Vec2f::from_stream(source)?;
        let map_transform_y = Vec2f::from_stream(source)?;
        let map_transform_z = Vec2f::from_stream(source)?;
        let index_base = source.read_u8()?;
        let index_range = source.read_u8()?;
        let identifier = read_c_string(source)?;
        trace!("... {}", identifier);
        Self::Output {
            color,
            opacity,
            ka,
            kd,
            ks,
            power,
            flags,
            map_transform_x,
            map_transform_y,
            map_transform_z,
            index_base,
            index_range,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub type ColorMapRefChunk = NameRefChunk;

//------------------------------------------------------------------
pub type IndexShadeRefChunk = NameRefChunk;

//------------------------------------------------------------------
pub type IndexBlendRefChunk = NameRefChunk;

//------------------------------------------------------------------
pub type ScreenDoorRefChunk = NameRefChunk;

// =================
// PixelMap chunks: (BrPixelmapLoadMany)

//------------------------------------------------------------------
pub struct PixelMapChunk {
    pub r#type: u8, // pixelmap_type::
    pub row_bytes: u16,
    pub width: u16,
    pub height: u16,
    pub origin_x: u16,
    pub origin_y: u16,
    pub identifier: String,
}

impl FromStream for PixelMapChunk {
    type Output = PixelMapChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let r#type = source.read_u8()?;
        let row_bytes = source.read_u16::<BigEndian>()?;
        let width = source.read_u16::<BigEndian>()?;
        let height = source.read_u16::<BigEndian>()?;
        let origin_x = source.read_u16::<BigEndian>()?;
        let origin_y = source.read_u16::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        trace!("... {}", identifier);
        Self::Output {
            r#type,
            row_bytes,
            width,
            height,
            origin_x,
            origin_y,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub struct PixelsChunk {
    pub units: u32,
    pub unit_bytes: u32,
    pub data: Vec<u8>,
}

impl FromStream for PixelsChunk {
    type Output = PixelsChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let units = source.read_u32::<BigEndian>()?;
        let unit_bytes = source.read_u32::<BigEndian>()?;

        let payload_size = (units * unit_bytes) as usize;
        let mut data = vec![0u8; payload_size];
        source.read_exact(&mut data)?;

        Self::Output {
            units,
            unit_bytes,
            data,
        }
    }
}

// =================
// Actor chunks: (BrActorLoadMany)

//------------------------------------------------------------------
#[derive(Default)]
pub struct ActorChunk {
    pub r#type: u8,       // actor_type::
    pub render_style: u8, // actor_render_style::
    pub identifier: String,
}

impl FromStream for ActorChunk {
    type Output = ActorChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let r#type = source.read_u8()?;
        let render_style = source.read_u8()?;
        let identifier = read_c_string(source)?;
        Self::Output {
            r#type,
            render_style,
            identifier,
        }
    }
}

//------------------------------------------------------------------
pub type ActorModelChunk = NameRefChunk;

//------------------------------------------------------------------
pub type ActorMaterialChunk = NameRefChunk;

//------------------------------------------------------------------
pub struct ActorTransformActionChunk {} // empty, simply attach transform on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorLightActionChunk {} // empty, simply attach light on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorCameraActionChunk {} // empty, simply attach camera on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorBoundsActionChunk {} // empty, simply attach bounds on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorClipPlaneActionChunk {} // empty, simply attach clip plane on top of stack to the actor

//------------------------------------------------------------------
pub struct ActorAddChildActionChunk {} // empty, simply attach actor on top of stack to the actor

//------------------------------------------------------------------
pub struct TransformMatrix34Chunk {
    m: Vec<Vec3f>, // 4-element vector of Vec3f
}

impl FromStream for TransformMatrix34Chunk {
    type Output = TransformMatrix34Chunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let mut m = Vec::with_capacity(4);
        for _ in 0..4 {
            m.push(Vec3f::from_stream(source)?);
        }
        Self::Output { m }
    }
}

//------------------------------------------------------------------
pub struct TransformQuatChunk {
    q_x: f32,
    q_y: f32,
    q_z: f32,
    q_w: f32,
    t: Vec3f,
}

impl FromStream for TransformQuatChunk {
    type Output = TransformQuatChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let q = Vec4f::from_stream(source)?;
        let t = Vec3f::from_stream(source)?;
        Self::Output {
            q_x: q.x,
            q_y: q.y,
            q_z: q.z,
            q_w: q.w,
            t,
        }
    }
}

//------------------------------------------------------------------
pub type Angle = f32;

//------------------------------------------------------------------
pub struct TransformEulerChunk {
    e_order: u8,
    e_a: Angle,
    e_b: Angle,
    e_c: Angle,
    t: Vec3f,
}

impl FromStream for TransformEulerChunk {
    type Output = TransformEulerChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let e_order = source.read_u8()?;
        let e = Vec3f::from_stream(source)?;
        let t = Vec3f::from_stream(source)?;
        Self::Output {
            e_order,
            e_a: e.x,
            e_b: e.y,
            e_c: e.z,
            t,
        }
    }
}

//------------------------------------------------------------------
pub struct TransformLookUpChunk {
    look: Vec3f,
    up: Vec3f,
    t: Vec3f,
}

impl FromStream for TransformLookUpChunk {
    type Output = TransformLookUpChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let look = Vec3f::from_stream(source)?;
        let up = Vec3f::from_stream(source)?;
        let t = Vec3f::from_stream(source)?;
        Self::Output { look, up, t }
    }
}

//------------------------------------------------------------------
pub struct TransformTranslationChunk {
    t: Vec3f,
}

impl FromStream for TransformTranslationChunk {
    type Output = TransformTranslationChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let t = Vec3f::from_stream(source)?;
        Self::Output { t }
    }
}

//------------------------------------------------------------------
#[derive(ResourceTag)]
pub struct BoundsChunk {
    min: Vec3f,
    max: Vec3f,
}

impl FromStream for BoundsChunk {
    type Output = BoundsChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let min = Vec3f::from_stream(source)?;
        let max = Vec3f::from_stream(source)?;
        Self::Output { min, max }
    }
}

//------------------------------------------------------------------
#[derive(ResourceTag)]
pub struct LightChunk {
    light_type: u8,
    color: Colour,
    attn_c: f32,
    attn_l: f32,
    attn_q: f32,
    cone_inner: Angle,
    cone_outer: Angle,
    identifier: String,
}

impl FromStream for LightChunk {
    type Output = LightChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let light_type = source.read_u8()?;
        let color = Rgb::from_stream(source)?.into();
        let attn_c = source.read_f32::<BigEndian>()?;
        let attn_l = source.read_f32::<BigEndian>()?;
        let attn_q = source.read_f32::<BigEndian>()?;
        let cone_inner = source.read_f32::<BigEndian>()?;
        let cone_outer = source.read_f32::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        Self::Output {
            light_type,
            color,
            attn_c,
            attn_l,
            attn_q,
            cone_inner,
            cone_outer,
            identifier,
        }
    }
}

//------------------------------------------------------------------
#[derive(ResourceTag)]
pub struct CameraChunk {
    camera_type: u8,
    fov: Angle,
    hither_z: f32,
    yon_z: f32,
    aspect: f32,
    identifier: String,
}

impl FromStream for CameraChunk {
    type Output = CameraChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let camera_type = source.read_u8()?;
        let fov = source.read_f32::<BigEndian>()?;
        let hither_z = source.read_f32::<BigEndian>()?;
        let yon_z = source.read_f32::<BigEndian>()?;
        let aspect = source.read_f32::<BigEndian>()?;
        let identifier = read_c_string(source)?;
        Self::Output {
            camera_type,
            fov,
            hither_z,
            yon_z,
            aspect,
            identifier,
        }
    }
}

//------------------------------------------------------------------
#[derive(ResourceTag)]
pub struct PlaneChunk {
    equation: Vec4f,
}

impl FromStream for PlaneChunk {
    type Output = PlaneChunk;

    #[throws(support::Error)]
    fn from_stream<S: ReadBytesExt + BufRead>(source: &mut S) -> Self::Output {
        let equation = Vec4f::from_stream(source)?;
        Self::Output { equation }
    }
}

//------------------------------------------------------------------
/// All chunk types (probably useless, use ModelLoadChunks, ActorLoadChunks etc)
pub enum Chunk {
    // =================
    // Universal chunks:
    End(),
    FileInfo(FileInfoChunk),

    // =================
    // Model chunks: (BrModelLoadMany)
    Model(ModelChunk),
    MaterialIndex(MaterialIndexChunk),
    Vertices(VerticesChunk),
    VertexUV(VertexUvChunk),
    Faces(FacesChunk),
    FaceMaterial(FaceMaterialChunk), // FACE_MATERIAL - external size
    Pivot(PivotChunk),

    // =================
    // Material chunks: (BrMaterialLoadMany)
    Material(MaterialChunk),
    ColorMapRef(ColorMapRefChunk),
    IndexBlendRef(IndexBlendRefChunk), // INDEX_BLEND_REF
    IndexShadeRef(IndexShadeRefChunk),
    ScreendoorRef(ScreenDoorRefChunk), // SCREENDOOR_REF

    // =================
    // PixelMap chunks: (BrPixelmapLoadMany)
    PixelMap(PixelMapChunk),
    Pixels(PixelsChunk),
    AddMap(),

    // =================
    // Actor chunks: (BrActorLoadMany)
    Actor(ActorChunk),
    ActorModel(ActorModelChunk),
    ActorTransform(ActorTransformActionChunk),
    ActorMaterial(ActorMaterialChunk),
    ActorLight(ActorLightActionChunk),
    ActorCamera(ActorCameraActionChunk),
    ActorBounds(ActorBoundsActionChunk),
    ActorClipPlane(ActorClipPlaneActionChunk),
    ActorAddChild(ActorAddChildActionChunk),

    TransformMatrix34(TransformMatrix34Chunk),
    TransformMatrix34LP(TransformMatrix34Chunk),
    TransformQuat(TransformQuatChunk),
    TransformEuler(TransformEulerChunk),
    TransformLookUp(TransformLookUpChunk),
    TransformTranslation(TransformTranslationChunk),
    TransformIdentity(),

    Bounds(BoundsChunk),
    Light(LightChunk),
    Camera(CameraChunk),
    Plane(PlaneChunk),
}

impl FromStream for Chunk {
    type Output = Chunk;

    /// General chunk reader, no logic, just i/o.
    #[throws(support::Error)]
    fn from_stream<R: ReadBytesExt + BufRead>(source: &mut R) -> Chunk {
        let header = ChunkHeader::from_stream(source)?;
        match header.chunk_type {
            // =================
            // Universal chunks:
            chunk::END => Chunk::End(),
            chunk::FILE_INFO => {
                trace!("Reading file info...");
                assert_eq!(header.size, 8);
                Chunk::FileInfo(FileInfoChunk::from_stream(source)?)
            }

            // =================
            // Model chunks: (BrModelLoadMany)
            chunk::MODEL => {
                trace!("Reading model...");
                Chunk::Model(ModelChunk::from_stream(source)?)
            }
            chunk::MATERIAL_INDEX => {
                trace!("Reading material index...");
                Chunk::MaterialIndex(MaterialIndexChunk::from_stream(source)?)
            }
            chunk::VERTICES => {
                trace!("Reading vertex list...");
                Chunk::Vertices(VerticesChunk::from_stream(source)?)
            }
            chunk::VERTEX_UV => {
                trace!("Reading uvmap list...");
                Chunk::VertexUV(VertexUvChunk::from_stream(source)?)
            }
            chunk::FACES => {
                trace!("Reading face list...");
                Chunk::Faces(FacesChunk::from_stream(source)?)
            }
            chunk::FACE_MATERIAL => {
                trace!("Reading face material list...");
                Chunk::FaceMaterial(FaceMaterialChunk::from_stream(source)?)
            }
            chunk::PIVOT => {
                trace!("Reading pivot...");
                Chunk::Pivot(PivotChunk::from_stream(source)?)
            }

            // =================
            // Material chunks: (BrMaterialLoadMany)
            chunk::MATERIAL => {
                trace!("Reading material descriptor...");
                Chunk::Material(MaterialChunk::from_stream(source)?)
            }
            chunk::COLOUR_MAP_REF => {
                trace!("Reading pixelmap ref...");
                Chunk::ColorMapRef(ColorMapRefChunk::from_stream(source)?)
            }
            // ❌ IndexBlendRef(),                      // INDEX_BLEND_REF
            chunk::INDEX_SHADE_REF => {
                trace!("Reading rendertab (shade) ref...");
                Chunk::IndexShadeRef(IndexShadeRefChunk::from_stream(source)?)
            }
            // ❌ ScreendoorRef(),                      // SCREENDOOR_REF

            // =================
            // PixelMap chunks: (BrPixelmapLoadMany)
            chunk::PIXELMAP => {
                trace!("Reading pixelmap header...");
                Chunk::PixelMap(PixelMapChunk::from_stream(source)?)
            }
            chunk::PIXELS => {
                trace!("Reading pixelmap data...");
                Chunk::Pixels(PixelsChunk::from_stream(source)?)
            }
            // ❌ AddMap(),

            // =================
            // Actor chunks: (BrActorLoadMany)
            chunk::ACTOR => {
                trace!("Reading actor...");
                Chunk::Actor(ActorChunk::from_stream(source)?)
            }
            chunk::ACTOR_MODEL => {
                trace!("Reading actor model ref...");
                Chunk::ActorModel(ActorModelChunk::from_stream(source)?)
            }
            chunk::ACTOR_TRANSFORM => {
                trace!("Attaching actor transform...");
                Chunk::ActorTransform(ActorTransformActionChunk {})
            }
            chunk::ACTOR_MATERIAL => {
                trace!("Reading actor material ref...");
                Chunk::ActorMaterial(ActorMaterialChunk::from_stream(source)?)
            }
            chunk::ACTOR_LIGHT => {
                trace!("Attaching actor light...");
                Chunk::ActorLight(ActorLightActionChunk {})
            }
            chunk::ACTOR_CAMERA => {
                trace!("Attaching actor camera...");
                Chunk::ActorCamera(ActorCameraActionChunk {})
            }
            chunk::ACTOR_BOUNDS => {
                trace!("Attaching actor bounds...");
                Chunk::ActorBounds(ActorBoundsActionChunk {})
            }
            chunk::ACTOR_CLIP_PLANE => {
                trace!("Attaching actor clip plane...");
                Chunk::ActorClipPlane(ActorClipPlaneActionChunk {})
            }
            chunk::ACTOR_ADD_CHILD => {
                trace!("Attaching sub-actor to actor...");
                Chunk::ActorAddChild(ActorAddChildActionChunk {})
            }

            chunk::TRANSFORM_MATRIX34 => {
                trace!("Reading transform 3x4...");
                Chunk::TransformMatrix34(TransformMatrix34Chunk::from_stream(source)?)
            }
            chunk::TRANSFORM_MATRIX34_LP => {
                trace!("Reading transform 3x4 LP...");
                Chunk::TransformMatrix34LP(TransformMatrix34Chunk::from_stream(source)?)
            }
            chunk::TRANSFORM_QUAT => {
                trace!("Reading transform quat...");
                Chunk::TransformQuat(TransformQuatChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_EULER => {
                trace!("Reading transform Euler...");
                Chunk::TransformEuler(TransformEulerChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_LOOK_UP => {
                trace!("Reading transform look up...");
                Chunk::TransformLookUp(TransformLookUpChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_TRANSLATION => {
                trace!("Reading transform translation...");
                Chunk::TransformTranslation(TransformTranslationChunk::from_stream(source)?)
            }
            chunk::TRANSFORM_IDENTITY => {
                trace!("Reading transform 3x4 LP...");
                Chunk::TransformIdentity()
            }

            chunk::BOUNDS => {
                trace!("Reading bounds...");
                Chunk::Bounds(BoundsChunk::from_stream(source)?)
            }
            chunk::LIGHT => {
                trace!("Reading light...");
                Chunk::Light(LightChunk::from_stream(source)?)
            }
            chunk::CAMERA => {
                trace!("Reading camera...");
                Chunk::Camera(CameraChunk::from_stream(source)?)
            }
            chunk::PLANE => {
                trace!("Reading plane...");
                Chunk::Plane(PlaneChunk::from_stream(source)?)
            }

            _ => unimplemented!(),
        }
    }
}

/// Values on the resource stack.
pub trait ResourceTag {
    // fn as_any(&self) -> &dyn Any;
    // fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// A Carma resource that has a name
pub trait NamedResource {
    fn resource_name(&self) -> String;
}

impl<T: NamedResource> NamedResource for Box<T> {
    fn resource_name(&self) -> String {
        use std::ops::Deref;
        self.deref().resource_name()
    }
}

// Named resources:
// Actor
// Model
// PixelMap
// Material
//
// ResourceTags:
// Actor
// Model
// enum Transform for set of transform chunks
//
// LightChunk
// CameraChunk
// BoundsChunk
// PlaneChunk

// pub enum ResourceTag2 {
//     // unused
//     ImagePlane(),
//     MaterialIndex(Box<MaterialIndexChunk>),
//     Vertices(Box<VerticesChunk>),
//     Faces(Box<FacesChunk>),
//     Anim,
//     AnimName,
//     AnimTransform,
//     AnimCount,
//     AnimRate,
//     FileInfo(Box<FileInfoChunk>),
//     Pivot(Box<PivotChunk>),
// }

//------------------------------------------------------------------
/// Loading stack for resource chunks.
/// The per-resource loaders use it to construct final Actor or Model object.
#[derive(Default)]
pub struct ResourceStack {
    stack: Vec<Box<dyn Any>>,
}

impl ResourceStack {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn push(&mut self, resource: Box<dyn Any>) {
        self.stack.push(resource);
    }

    #[throws]
    pub fn pop<T: ResourceTag + 'static>(&mut self) -> Box<T> {
        let box_ = self.stack.pop().ok_or(Error::EmptyStack)?;
        box_.downcast::<T>().or(Err(Error::InvalidResourceType))?
    }

    /// Give mutable access to the stack top.
    #[throws]
    pub fn top<T: ResourceTag + 'static>(&mut self) -> &mut T {
        let box_ = self.stack.last_mut().ok_or(Error::EmptyStack)?;
        (*box_)
            .downcast_mut::<T>()
            .ok_or(Error::InvalidResourceType)?
    }
}

//------------------------------------------------------------------
// Tests.
//------------------------------------------------------------------

#[cfg(test)]
mod tests {
    use {super::*, std::io::Cursor};

    #[test]
    fn test_load_face() {
        let mut data = Cursor::new(vec![0xde, 0xad, 0xbe, 0xef, 0xca, 0xfe, 0xba, 0xbe, 0xed]);
        let f = Face::from_stream(&mut data).unwrap();
        assert_eq!(0xdead, f.v1);
        assert_eq!(0xbeef, f.v2);
        assert_eq!(0xcafe, f.v3);
        assert_eq!(0xbabe, f.smoothing);
        assert_eq!(0xed, f.flags);
    }

    #[test]
    fn test_stack_hetero() {
        #[derive(ResourceTag)]
        struct Res1 {
            pub x: usize,
        }
        #[derive(ResourceTag)]
        struct Res2;
        let mut stack = ResourceStack::new();
        stack.push(Box::new(Res1 { x: 0 }));
        stack.push(Box::new(Res2));
        stack.pop::<Res2>().expect("Should have the right type");
        stack.top::<Res1>().expect("Should have the right type");
        assert_eq!(stack.top::<Res1>().unwrap().x, 0);
        stack.top::<Res1>().unwrap().x = 1;
        assert_eq!(stack.top::<Res1>().unwrap().x, 1);
    }
}

// struct A;
// struct B;

// impl A {
//     fn do_first_component_thing(&self) {
//         println!("First component thing");
//     }
// }
// impl B {
//     fn do_second_component_thing(&self) {
//         println!("Second component thing");
//     }
// }

// fn main() {
//     let mut components: Vec<Box<dyn Component>> = Vec::new();
//     components.push(Box::new(A {}));
//     components.push(Box::new(B {}));

//     if let Some(component) =
//             components[0].as_any().downcast_ref::<A>() {
//         component.do_first_component_thing();
//     }

//     if let Some(component) =
//             components[1].as_any().downcast_ref::<B>() {
//         component.do_second_component_thing();
//     }
// }
