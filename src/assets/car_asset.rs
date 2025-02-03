use bevy::{
    asset::{io::Reader, Asset, AssetLoader, Handle, LoadContext},
    reflect::TypePath,
    render::mesh::Mesh,
    utils::BoxedFuture,
};

// Use support/car to load car asset with mesh and textures...
#[derive(Asset, TypePath)]
pub struct CarAsset {
    _body: Handle<Mesh>,
    // #[asset(key = "combined_image")]
    // combined_image: Handle<Image>,
    // #[asset(key = "tree_standard_material")]
    // tree_standard_material: Handle<StandardMaterial>,
    // #[asset(key = "player_standard_material")]
    // player_standard_material: Handle<StandardMaterial>,
}

pub struct CarAssetLoader;

impl AssetLoader for CarAssetLoader {
    type Asset = CarAsset;
    type Settings = ();
    type Error = anyhow::Error;

    // obsolete, move this to asset_io somehow?
    // fn from_bytes(&self, asset_path: &Path, _bytes: Vec<u8>) -> Result<Car> {
    //     info!("### Loading car {:?} via AssetLoader", asset_path);
    //     Car::load_from(asset_path)
    // }

    fn load<'a>(
        &'a self,
        _reader: &'a mut Reader,
        _settings: &'a Self::Settings,
        _load_context: &'a mut LoadContext,
    ) -> BoxedFuture<'a, Result<Self::Asset, anyhow::Error>> {
        todo!()
    }

    fn extensions(&self) -> &[&str] {
        &["ENC"]
    }
}
