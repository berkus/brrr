//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    super::resource::{
        file_type, Chunk, FileInfoChunk, FromStream, MaterialChunk, ResourceStack, ResourceTag,
    },
    crate::support::{self, Error},
    bevy::prelude::*,
    byteorder::ReadBytesExt,
    carma_derive::ResourceTag,
    culpa::{throw, throws},
    std::{
        fs::File,
        io::{BufRead, BufReader},
    },
};

// MAT file is an index of: material internal name, PIX file name and TAB file name.
// @todo ❌ keep material properties and internal name, replace pix and tab with megatexture reference
#[derive(Default, Debug, ResourceTag)]
pub struct Material {
    pub material: MaterialChunk,
    // We will remap names to resources when converting to bevy...
    pub color_map_name: String,
    pub index_shade_name: String, // Palette file used to convert u8 indexed color to RGBA
    pub index_blend_name: String,
    pub screendoor_name: String,
}

impl std::fmt::Display for Material {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}, material {:?}, color map {}, shade {}, blend {}, screendoor {}",
            self.material.identifier,
            self.material,
            self.color_map_name,
            self.index_shade_name,
            self.index_blend_name,
            self.screendoor_name,
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

/*
 * Table of chunk id's versus offsets in material structure
 */
// STATIC struct {
// 	br_uint_32 id;
// 	size_t offset;
// 	int table_or_map;
// } MaterialMaps[] = {
// 	{FID_COLOUR_MAP_REF,	offsetof(struct br_material,colour_map),		MAP},
// 	{FID_INDEX_BLEND_REF,	offsetof(struct br_material,index_blend),		TABLE},
// 	{FID_INDEX_SHADE_REF,	offsetof(struct br_material,index_shade),		TABLE},
// 	{FID_SCREENDOOR_REF,	offsetof(struct br_material,screendoor),		TABLE},
// };

/*
 * Various lists of registered items in private framework state. (INC/FW.H)
 */
// br_registry reg_models;
// br_registry reg_materials;
// br_registry reg_textures; // MAP looks up here
// br_registry reg_tables; // TABLE looks up here
// br_registry reg_resource_classes;

// Lookup uses name match and find-first strategy:
//                int NamePatternMatch(char *p, char *s)
//                {
// char *cp;

// /*
//  * A NULL pattern matches everything
//  */
// if(p == NULL)
// 	return 1;

// /*
//  * A NULL string never matches
//  */
// if(s == NULL)
// 	return 0;

// for(;;) switch(*p) {

// case '/':
// case '\0':
// 	/*
// 	 * Empty pattern only matches empty string
// 	 */
// 	return *s == '\0';

// case '*':
// 	/*
// 	 * Match p+1 in any position from s to end of s
// 	 */
// 	cp = s;
// 	do
// 		if(NamePatternMatch(p+1,cp))
// 			return 1;
// 	while (*cp++);

// 	return 0;

// case '?':
// 	/*
// 	 * Match any character followed by matching(p+1, s+1)
// 	 */
// 	if(*s == '\0')
// 		return 0;

// 	p++, s++;	/* Tail recurse */
// 	continue;

// default:
// 	/*
// 	 * Match this character followed by matching(p+1, s+1)
// 	 */
// 	if(!MATCH_CHAR(*s,*p))
// 		return 0;

// 	p++, s++;	/* Tail recurse */
// 	continue;
// }
//                }

impl FromStream for Material {
    type Output = Box<Material>;

    /// Read chunks until last chunk is encountered.
    /// Certain chunks initialize certain properties.
    #[throws(Error)]
    fn from_stream<R: ReadBytesExt + BufRead>(source: &mut R) -> Self::Output {
        let mut stack = ResourceStack::new();

        loop {
            match Chunk::from_stream(source)? {
                Chunk::End() => break,
                Chunk::FileInfo(FileInfoChunk { file_type, .. }) => {
                    if file_type != file_type::MATERIAL {
                        throw!(
                            Error::InvalidResourceType /*{
                                                       expected: file_type::MATERIAL,
                                                       received: file_type,
                                                       }*/
                        );
                    }
                }
                Chunk::Material(material) => {
                    let mut mat = Material::default();
                    mat.material = material;
                    stack.push(Box::new(mat));
                }
                Chunk::ColorMapRef(color_map) => {
                    stack.top::<Material>()?.color_map_name = color_map.identifier;
                    //MapLookup(color_map);
                }
                Chunk::IndexShadeRef(index_shade) => {
                    stack.top::<Material>()?.index_shade_name = index_shade.identifier;
                    //TableLookup(index_shade);
                }
                Chunk::IndexBlendRef(index_blend) => {
                    stack.top::<Material>()?.index_blend_name = index_blend.identifier;
                    //TableLookup(index_blend);
                }
                Chunk::ScreendoorRef(screendoor) => {
                    stack.top::<Material>()?.screendoor_name = screendoor.identifier;
                    //TableLookup(screendoor);
                }

                _ => unimplemented!(), // unexpected type here
            }
        }

        stack.pop::<Material>()?
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
    pub fn load_many<P: AsRef<std::path::Path> + std::fmt::Debug>(
        filename: P,
    ) -> Vec<Box<Material>> {
        debug!("Loading many Materials from {:?}", filename);
        let mut file = BufReader::new(File::open(filename)?);
        let mut materials = Vec::<_>::new();
        loop {
            let mat = Material::from_stream(&mut file);
            match mat {
                Err(_) => break, // @fixme allow only Eof here
                Ok(mat) => {
                    debug!(".. Loaded {}", mat.material.identifier);
                    materials.push(mat)
                }
            }
        }
        materials
    }
}
