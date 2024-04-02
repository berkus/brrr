use {
    anyhow::Result,
    carma::support::{
        brender::{pixelmap::PixelMap, resource::LoadMany},
        visitor::visit_files,
    },
    log::info,
    std::{
        fs::File,
        io::BufWriter,
        path::{Path, PathBuf},
    },
};

fn main() {
    convert_all_pixmaps().expect("Listing failed");
    convert_game_pixmap(String::from("DecodedData/DATA/PIXELMAP/EAGYELE.PIX"))
        .expect("Conversion failed");
}

// /// Load palette once and then apply to a bunch of pixmap data
fn convert_all_pixmaps() -> Result<()> {
    let palette = &PixelMap::load_many(String::from(
        "assets/DecodedData/DATA/REG/PALETTES/DRRENDER.PAL",
    ))?[0];
    visit_files(Path::new("assets/DecodedData"), &mut |dir_entry| {
        if let Ok(file_type) = dir_entry.file_type() {
            let fname = String::from(dir_entry.path().to_str().unwrap());
            if file_type.is_file() && fname.ends_with(".PIX") {
                convert_pixmap(fname, palette)?;
            }
        }
        Ok(())
    })
}

fn convert_game_pixmap(fname: String) -> Result<()> {
    let palette = &PixelMap::load_many(String::from(
        "assets/DecodedData/DATA/REG/PALETTES/DRRENDER.PAL",
    ))?[0];
    convert_pixmap(fname, palette)
}

fn convert_pixmap(fname: String, palette: &PixelMap) -> Result<()> {
    let pmap = PixelMap::load_many(fname.clone())
        .expect(format!("Couldnt open pix file {:?}", fname).as_ref());
    // let mut counter = 0;
    for pix in pmap {
        // counter += 1;
        let mut pngname = PathBuf::from(&fname);
        // let name = String::from(pngname.file_name().unwrap().to_str().unwrap());
        pngname.set_file_name(&pix.identifier);
        pngname.set_extension("png");

        info!("Creating file {:?}", pngname);
        let file = File::create(&pngname)
            .expect(format!("Couldnt create png file {:?}", pngname).as_ref());
        let w = &mut BufWriter::new(file);

        pix.write_png_remapped_via(palette, w)
            .expect("Failed to write PNG");
    }
    Ok(())
}

/// Uses different palette for race-selection part
#[allow(unused)]
fn convert_menu_pixmap(fname: String) -> Result<()> {
    let palette = &PixelMap::load_many(String::from(
        "assets/DecodedData/DATA/REG/PALETTES/DRACEFLC.PAL",
    ))?[0];
    convert_pixmap(fname, palette)
}
