//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    super::resource::{Chunk, FromStream},
    crate::support,
    byteorder::ReadBytesExt,
    culpa::throws,
    std::{
        fs::File,
        io::{BufRead, BufReader},
    },
};

// MAT file is an index of: material internal name, PIX file name and TAB file name.
// @todo ❌ keep material properties and internal name, replace pix and tab with megatexture reference
#[derive(Default, Debug)]
pub struct Material {
    params: [f32; 12],
    pub name: String,
    pub pixelmap_name: String,
    rendertab_name: String, // Palette file used to convert u8 indexed color to RGBA
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, pixelmap {}, rendertab {}",
            self.name, self.pixelmap_name, self.rendertab_name,
        )
    }
}

#[derive(Default)]
pub struct MaterialLoader;

// impl AssetLoader for MaterialLoader {
// fn from_bytes(&self, asset_path: &Path, _bytes: Vec<u8>) -> Result<Material> {
//     info!("### Loading car {:?} via AssetLoader", asset_path);
//     Material::load_from(asset_path)
// }

//     fn extensions(&self) -> &[&str] {
//         static EXTENSIONS: &[&str] = &["ENC"];
//         EXTENSIONS
//     }
// }

impl FromStream for Material {
    type Output = Material;
    /// @todo material loader using stack
    /// Read chunks until last chunk is encountered.
    /// Certain chunks initialize certain properties.
    #[throws(support::Error)]
    fn from_stream<R: ReadBytesExt + BufRead>(source: &mut R) -> Self::Output {
        let /*mut*/ mat = Material::default();

        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::Material(_) => {}
                Chunk::ColorMapRef(_) => {}
                Chunk::IndexShadeRef(_) => {}
                Chunk::IndexBlendRef() => {}
                //        Chunk:ScreendoorRef => ,
                //         Chunk::FileHeader { file_type } => {
                //             if file_type != support::MATERIAL_FILE_TYPE {
                //                 return Err(anyhow!("Invalid material file type {}", file_type));
                //             }
                //         }
                //         Chunk::MaterialDesc { name, params } => {
                //             mat.params = params;
                //             mat.name = name;
                //         }
                //         Chunk::PixelmapRef(name) => mat.pixelmap_name = name,
                //         Chunk::RenderTabRef(name) => mat.rendertab_name = name,
                _ => unimplemented!(), // unexpected type here
            }
        }

        mat
    }
}

impl Material {
    /**
     * Load multiple materials from a file.
    //
    // currently theres no "really nice" way to do that. currently it would be something like
    //
    // impl AssetLoader<Vec<Mesh>> for VecMeshLoader {}
    //
    // Then consume AssetEvent<Vec<Mesh>> in a system. When new Vec<Mesh>es get loaded, insert each Mesh into Assets<Mesh>
    //
    // atelier-assets handles this scenario in a cleaner way (and we're considering a move to that)
     */
    // @sa brender's `BrMaterialLoadMany()`
    #[throws(support::Error)]
    pub fn load_many<P: AsRef<std::path::Path>>(filename: P) -> Vec<Material> {
        let mut file = BufReader::new(File::open(filename)?);
        let mut materials = Vec::<Material>::new();
        loop {
            let mat = Material::from_stream(&mut file);
            match mat {
                Err(_) => break, // @fixme allow only Eof here
                Ok(mat) => materials.push(mat),
            }
        }
        materials
    }
}
