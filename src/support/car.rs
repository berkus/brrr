//
// Part of Roadkill Project.
//
// Copyright 2010, 2017, Stanislav Karchebnyy <berkus@madfire.net>
//
// Distributed under the Boost Software License, Version 1.0.
// (See file LICENSE_1_0.txt or a copy at http://www.boost.org/LICENSE_1_0.txt)
//
use {
    crate::support::{
        brender::{actor::Actor, material::Material, model::Model, pixelmap::PixelMap},
        path_subst,
    },
    anyhow::{anyhow, Error, Result},
    bevy::prelude::*,
    cgmath::Vector3,
    culpa::throws,
    // log::*,
    std::{
        collections::{HashMap, HashSet},
        fs::File,
        io::{BufRead, BufReader},
        path::Path,
    },
};

// Car assembles the gameplay object (a car in this case) from various model and texture files.
#[derive(Component)]
pub struct Car {
    pub name: String,
    pub actors: Actor,
    pub meshes: HashMap<String, Model>,
    pub materials: HashMap<String, Material>,
    pub textures: HashMap<String, PixelMap>,
    pub base_translation: Vector3<f32>,
}

/// Expect next line to match provided text exactly.
fn expect_match(input: &mut impl Iterator<Item = String>, text: &str) -> Result<()> {
    if let Some(line) = input.next() {
        if line == text {
            return Ok(());
        }
        return Err(anyhow!("Expected {:?} but got {:?}", text, line));
    }
    Err(anyhow!("Expected {:?} but got empty line", text))
}

/// Parse a three-component vector from a comma-separated string.
#[throws]
fn parse_vector(line: &str) -> Vector3<f32> {
    let line: Result<Vec<f32>, _> = line.split(',').map(|i| i.trim().parse::<f32>()).collect();
    let line = line?;
    Vector3::from((line[0], line[1], line[2]))
}

fn consume_line(input: &mut impl Iterator<Item = String>) -> Result<String> {
    input.next().ok_or(anyhow!("Bad input data"))
}

/// Read systems in a single damage spec clause.
fn read_systems(input: &mut impl Iterator<Item = String>) -> Result<()> {
    // read condition flag for this clause
    /*let condition =*/
    consume_line(input)?;
    // read systems count, read this many systems
    let systems_count = consume_line(input)?.parse()?;
    for _ in 0..systems_count {
        consume_line(input)?;
    }
    Ok(())
}

/// Read all damage spec clauses.
fn read_clauses(input: &mut impl Iterator<Item = String>) -> Result<()> {
    // read clause count, read this many systems
    let clause_count = consume_line(input)?.parse()?;
    for _ in 0..clause_count {
        read_systems(input)?;
    }
    Ok(())
}

/// Read a vector of strings.
fn read_vector(input: &mut impl Iterator<Item = String>) -> Result<Vec<String>> {
    // read vector size, read this many strings
    let size = consume_line(input)?.parse()?;
    let mut vec = Vec::<String>::with_capacity(size);
    for _ in 0..size {
        vec.push(consume_line(input)?);
    }
    Ok(vec)
}

fn read_funk(input: &mut impl Iterator<Item = String>) -> Result<()> {
    expect_match(input, "START OF FUNK")?;
    // for now just ignore everything here, read until END OF FUNK
    loop {
        // @todo ❌ read funk loop with NEXT FUNK as trigger
        // read_funk();
        // NEXT FUNK
        let line = consume_line(input)?;
        if line == "END OF FUNK" {
            return Ok(());
        }
    }
}

struct Groove {}

// Read a single groove
// fn read_groove(input: &mut impl Iterator<Item = String>) -> Groove {
// }

fn read_grooves(input: &mut impl Iterator<Item = String>) -> Result<()> {
    expect_match(input, "START OF GROOVE")?;
    // for now just ignore everything here, read until END OF GROOVE
    loop {
        // @todo ❌ read groove loop with NEXT GROOVE as trigger
        // read_groove();
        // NEXT GROOVE
        let line = consume_line(input)?;
        if line == "END OF GROOVE" {
            return Ok(());
        }
    }
}

/// A bunch of some matrices and mappings or vertex-pairs, ignore for now.
fn read_some_metadata(input: &mut impl Iterator<Item = String>) -> Result<()> {
    consume_line(input)?; // 0.700000
    consume_line(input)?; // 0.050000,0.300000
    consume_line(input)?; // 0.050000
    consume_line(input)?; // 0.050000
    consume_line(input)?; // 0.000000
    consume_line(input)?; // 0.000000
    let size = consume_line(input)?.parse()?;
    for _ in 0..size {
        consume_line(input)?; // 11
        consume_line(input)?; // -0.107444, -0.080211, 0.106640
        consume_line(input)?; // -0.057444, 0.054463, 0.206640
        consume_line(input)?; // 0.038245, 0.352418, 0.220975
        consume_line(input)?; // 0.111755, 0.051602, 0.079025
        let pair_count = consume_line(input)?.parse()?;
        for _ in 0..pair_count {
            consume_line(input)?;
            consume_line(input)?;
        }
    }
    Ok(())
}

// @fixme used to patch actors now
// @todo ❌ should support extra wheels
pub struct Mechanics {
    pub lrwheel_pos: Vector3<f32>,
    pub rrwheel_pos: Vector3<f32>,
    pub lfwheel_pos: Vector3<f32>,
    pub rfwheel_pos: Vector3<f32>,
}

fn read_mechanics_block_v1_1(input: &mut impl Iterator<Item = String>) -> Result<Mechanics> {
    let lrwheel_pos = parse_vector(&consume_line(input)?)?;
    trace!("Left rear wheel position: {:?}", lrwheel_pos);

    let rrwheel_pos = parse_vector(&consume_line(input)?)?;
    trace!("Right rear wheel position: {:?}", rrwheel_pos);

    let lfwheel_pos = parse_vector(&consume_line(input)?)?;
    trace!("Left front wheel position: {:?}", lfwheel_pos);

    let rfwheel_pos = parse_vector(&consume_line(input)?)?;
    trace!("Right front wheel position: {:?}", rfwheel_pos);

    let centre_of_mass_pos = parse_vector(&consume_line(input)?)?;
    trace!("Centre of mass position: {:?}", centre_of_mass_pos);

    Ok(Mechanics {
        lrwheel_pos,
        rrwheel_pos,
        lfwheel_pos,
        rfwheel_pos,
    })
}

fn read_mechanics_block_v1_1_v3(input: &mut impl Iterator<Item = String>) -> Result<()> {
    let min_bb = parse_vector(&consume_line(input)?);
    let max_bb = parse_vector(&consume_line(input)?);
    trace!("Bounding box: ({:?} - {:?})", min_bb, max_bb);
    Ok(())
}

// Version 2 contains count for bounding boxes (which is always 1, that's why it's removed in ver 3)
fn read_mechanics_block_v1_1_v2(input: &mut impl Iterator<Item = String>) -> Result<()> {
    expect_match(input, "1")?;
    read_mechanics_block_v1_1_v3(input)
}

fn read_mechanics_block_v1_2(input: &mut impl Iterator<Item = String>) -> Result<()> {
    // 0.5                                     // min turning circle radius
    consume_line(input)?;
    // 0.025,  0.025                           // suspension give (forward, back)
    consume_line(input)?;
    // 0.090                  // ride height (must be more than miny in bounding box )
    consume_line(input)?;
    // 0.5                                     // damping factor
    consume_line(input)?;
    // 1.5                                     // mass in tonnes
    consume_line(input)?;
    // 1                                       // fractional reduction in friction when slipping
    consume_line(input)?;
    // 79, 80                                  // friction angle ( front and rear )
    consume_line(input)?;
    // 0.4,    0.2,    0.816 // width, height, length(0.816, 1.216) for angular momentum calculation
    consume_line(input)?;
    Ok(())
}

fn read_mechanics_block_v1_3(input: &mut impl Iterator<Item = String>) -> Result<()> {
    // 0.05, 0.05                              // rolling resistance front and back
    consume_line(input)?;
    // 6                                       // number of gears
    consume_line(input)?;
    // 200                                     // speed at red line in highest gear
    consume_line(input)?;
    // 4                           // acceleration in highest gear m/s^2 (i.e. engine strength)
    consume_line(input)?;
    Ok(())
}

fn read_mechanics_block_v2(input: &mut impl Iterator<Item = String>) -> Result<()> {
    // 2.0                                     // traction fractional multiplier v. 2
    consume_line(input)?;
    // 50                                      // speed at which down force = weight v. 2
    consume_line(input)?;
    // 1.0                                     // brake multiplier, 1 = nomral brakes v. 2
    consume_line(input)?;
    // 1.0                                     // increase in brakes per second 1 = normal v. 2
    consume_line(input)?;
    Ok(())
}

fn read_mechanics_block_v3(input: &mut impl Iterator<Item = String>) -> Result<()> {
    // 3
    // 0,-0.18,-0.52                               // extra point 1            v. 3
    // -0.07,0.07,0.18                         // extra point 2            v. 3
    // 0.07,0.07,0.18                          // extra point 3            v. 3
    read_vector(input)?; // @todo ❌ must be a vector of Vec3tors
    Ok(())
}

fn read_mechanics_v2(input: &mut impl Iterator<Item = String>) -> Result<Mechanics> {
    let mech = read_mechanics_block_v1_1(input)?;
    read_mechanics_block_v1_1_v2(input)?;
    read_mechanics_block_v1_2(input)?;
    read_mechanics_block_v2(input)?;
    read_mechanics_block_v1_3(input)?;
    Ok(mech)
}

fn read_mechanics_v3(input: &mut impl Iterator<Item = String>) -> Result<Mechanics> {
    let mech = read_mechanics_block_v1_1(input)?;
    read_mechanics_block_v1_1_v3(input)?;
    read_mechanics_block_v3(input)?;
    read_mechanics_block_v1_2(input)?;
    read_mechanics_block_v2(input)?;
    read_mechanics_block_v1_3(input)?;
    Ok(mech)
}

fn read_mechanics_v4(input: &mut impl Iterator<Item = String>) -> Result<Mechanics> {
    read_mechanics_v3(input)
}

fn read_meshes<P: AsRef<Path>>(
    fname: P,
    load_models: &[String],
    car_meshes: &mut HashMap<String, Model>,
) -> Result<()> {
    let mut load_models: Vec<String> = load_models.to_vec();
    load_models.sort();
    load_models.dedup();
    debug!("Models to load: {:?}", load_models);

    // Now iterate all meshes and load them.
    for mesh in load_models {
        let mut mesh_file_name = fname.as_ref().to_path_buf();
        mesh_file_name.set_file_name(mesh);
        let mesh_file_name = path_subst(mesh_file_name, "MODELS".into(), Some("DAT".into()))?;
        info!("### Opening model file {:?}", mesh_file_name);
        let meshes = Model::load_many(
            mesh_file_name
                .clone()
                .into_os_string()
                .into_string()
                .unwrap(),
        )?;
        for model in meshes {
            car_meshes.insert(model.name.clone(), *model); // @todo ❌ populate World resources
        }
    }
    Ok(())
}

// @todo ❌ this should be added to the TextureAtlas
fn read_materials<P: AsRef<Path>>(
    fname: P,
    load_materials: &HashSet<String>,
    _car_materials: &mut HashMap<String, Material>,
) -> Result<()> {
    for material in load_materials {
        let mut mat_file_name = fname.as_ref().to_path_buf();
        mat_file_name.set_file_name(material);
        let mat_file_name = path_subst(mat_file_name, "MATERIAL".into(), None)?;
        info!("### Opening material {:?}", mat_file_name);
        let _materials = Material::load_many(mat_file_name)?;
        // for mat in materials {
        //     car_materials.insert(mat.name.clone(), mat); // @todo ❌ make this Handle<Texture>
        // }
    }
    Ok(())
}

impl Car {
    pub fn dump(&self) {
        // self.actors.dump();
        // self.actors.dump_actor_points();
        for tex in &self.textures {
            println!("Texture {}: {}", tex.0, tex.1);
        }
        self.debug_meshes();
        for mat in &self.materials {
            println!("Material {}:", mat.0);
        }
    }

    pub fn debug_meshes(&self) {
        for mesh in &self.meshes {
            debug!("Mesh {}:", mesh.0);
            for mat in &mesh.1.material_names {
                debug!("... Material {}", mat);
            }
        }
    }

    pub fn load_from<P: AsRef<Path> + std::fmt::Debug>(fname: P) -> Result<Car> {
        // Load description file.
        // let description_file_name = path_subst(fname, "CARS", Some("ENC".into()))?;
        info!("### Opening car {:?}", fname);

        let description_file = BufReader::new(File::open(fname.as_ref())?);

        let mut input_lines = description_file
            .lines()
            .map_while(Result::ok)
            .filter(|line| !line.starts_with("//")) // Skip whole-line comments
            .filter(|line| !line.is_empty()) // Skip empty lines
            // Separate in-line comments from data
            .map(|line| {
                line.split("//")
                    .next()
                    .map(|x| x.trim())
                    .map(|x| x.to_owned())
                    .unwrap_or("".into())
            })
            .filter(|line| !line.is_empty());

        let car_name = consume_line(&mut input_lines)?;
        debug!("Car name {}", car_name);

        expect_match(&mut input_lines, "START OF DRIVABLE STUFF")?;

        let driver_head_3d_offset = parse_vector(&consume_line(&mut input_lines)?);
        trace!(
            "Offset of driver's head in 3D space {:?}",
            driver_head_3d_offset
        );

        let head_turn_angles = consume_line(&mut input_lines)?;
        trace!(
            "Angles to turn to make head go left and right {}",
            head_turn_angles
        );

        let mirror_3d_offset_and_fov = consume_line(&mut input_lines)?;
        trace!(
            "Offset of 'mirror camera' in 3D space, viewing angle of mirror {}",
            mirror_3d_offset_and_fov
        );

        let pratcam_borders = consume_line(&mut input_lines)?;
        trace!(
            "Pratcam border names (left, top, right, bottom) {}",
            pratcam_borders
        );

        expect_match(&mut input_lines, "END OF DRIVABLE STUFF")?;

        let engine_noise = consume_line(&mut input_lines)?;
        trace!(
            "Engine noise (normal, enclosed space, underwater) {}",
            engine_noise
        );

        let stealworthy = consume_line(&mut input_lines)?;
        trace!("Cannot be stolen (without cheat): {}", stealworthy);

        // @todo ❌ who knows what...
        read_clauses(&mut input_lines)?;
        read_clauses(&mut input_lines)?;
        read_clauses(&mut input_lines)?;
        read_clauses(&mut input_lines)?;
        read_clauses(&mut input_lines)?;
        read_clauses(&mut input_lines)?;

        let grid_image = consume_line(&mut input_lines)?;
        trace!("Grid image (opponent, frank, annie): {}", grid_image);

        let mut load_pixmaps = read_vector(&mut input_lines)?;
        load_pixmaps.append(&mut read_vector(&mut input_lines)?);
        load_pixmaps.append(&mut read_vector(&mut input_lines)?);

        let load_shadetable = read_vector(&mut input_lines)?;
        debug!("Shadetable to load: {:?}", load_shadetable);

        let mut load_materials = read_vector(&mut input_lines)?;
        load_materials.append(&mut read_vector(&mut input_lines)?);
        load_materials.append(&mut read_vector(&mut input_lines)?);

        let mut load_models = read_vector(&mut input_lines)?;

        let load_actors: HashMap<isize, String> = read_vector(&mut input_lines)?
            .iter()
            .map(|act| act.split(','))
            .map(|mut split| {
                (
                    split.next().and_then(|id| id.parse().ok()).unwrap_or(0),
                    split.next().map(String::from).unwrap_or_else(|| "".into()),
                )
            })
            .collect();
        debug!("Actors to load: {:?}", load_actors);

        let reflective_material = consume_line(&mut input_lines)?;
        trace!(
            "Name of reflective screen material (or none if non-reflective): {}",
            reflective_material
        );

        // Number of steerable wheels
        // GroovyFunkRef of 1st steerable wheel -- this is index in the GROOVE array below
        // GroovyFunkRef of 2nd steerable wheel
        let steerable_wheels = read_vector(&mut input_lines)?;
        trace!("Steerable wheels GroovyFunkRefs: {:?}", steerable_wheels);

        let lfsus_gfref = consume_line(&mut input_lines)?;
        trace!("Left-front suspension parts GroovyFunkRef: {}", lfsus_gfref);

        let rfsus_gfref = consume_line(&mut input_lines)?;
        trace!(
            "Right-front suspension parts GroovyFunkRef: {}",
            rfsus_gfref
        );

        let lrsus_gfref = consume_line(&mut input_lines)?;
        trace!("Left-rear suspension parts GroovyFunkRef: {}", lrsus_gfref);

        let rrsus_gfref = consume_line(&mut input_lines)?;
        trace!("Right-rear suspension parts GroovyFunkRef: {}", rrsus_gfref);

        let driven_wheels_gfref = consume_line(&mut input_lines)?;
        trace!(
            "Driven wheels GroovyFunkRefs (for spinning) - MUST BE 4 ITEMS: {}",
            driven_wheels_gfref
        );

        let nondriven_wheels_gfref = consume_line(&mut input_lines)?;
        trace!(
            "Non-driven wheels GroovyFunkRefs (for spinning) - MUST BE 4 ITEMS: {}",
            nondriven_wheels_gfref
        );

        let driven_wheels_diameter = consume_line(&mut input_lines)?;
        trace!("Driven wheels diameter: {}", driven_wheels_diameter);

        let nondriven_wheels_diameter = consume_line(&mut input_lines)?;
        trace!("Non-driven wheels diameter: {}", nondriven_wheels_diameter);

        read_funk(&mut input_lines)?;
        read_grooves(&mut input_lines)?;

        // @todo ❌ who knows what...
        read_some_metadata(&mut input_lines)?;
        read_some_metadata(&mut input_lines)?;
        read_some_metadata(&mut input_lines)?;

        let mechanics = consume_line(&mut input_lines)?; // @fixme ❌ Can't use expect_match() here
        if !mechanics.starts_with("START OF MECHANICS STUFF") {
            return Err(anyhow!(
                "Expected START OF MECHANICS STUFF, got {:?} instead",
                mechanics
            ));
        }
        let version = mechanics
            .split(" version ")
            .nth(1)
            .map(|x| x.parse())
            .ok_or(anyhow!("Bad input data"))??;

        let _mech = match version {
            2 => read_mechanics_v2(&mut input_lines),
            3 => read_mechanics_v3(&mut input_lines),
            4 => read_mechanics_v4(&mut input_lines),
            x => return Err(anyhow!("Unsupported mechanics version {}", x)),
        }?;

        expect_match(&mut input_lines, "END OF MECHANICS STUFF")?;

        let some_materials = read_vector(&mut input_lines)?;
        debug!("Some other materials to use: {:?}", some_materials);

        // @todo ❌ More post-mechanics stuff

        //
        // Meshes
        //
        let mut car_meshes = HashMap::<String, Model>::new();

        debug!("Meshes to load: {:?}", load_models);

        // Read meshes referenced from text description
        read_meshes(&fname, &load_models, &mut car_meshes)?;

        // Load actor file.
        let mut actor_file_name = fname.as_ref().to_path_buf();
        let idx: isize = 0;
        actor_file_name.set_file_name(&load_actors[&idx]); // Read mipmap 0 actor
        let actor_file_name = path_subst(actor_file_name, "ACTORS".into(), Some("ACT".into()))?;
        info!("### Opening actor {:?}", actor_file_name);
        let car_actors = Actor::load_many(actor_file_name)?;

        // Read meshes referenced from actor file
        load_models.clear();
        // for actor in car_actors.traverse() {
        //     if let ActorNode::MeshfileRef(name) = actor.data() {
        //         if !car_meshes.contains_key(name) {
        //             load_models.push(name.clone())
        //         }
        //     }
        // }

        debug!("Extra meshes to load: {:?}", load_models);
        read_meshes(&fname, &load_models, &mut car_meshes)?;

        //
        // Materials
        //
        let mut load_materials: HashSet<String> = load_materials.iter().cloned().collect();
        debug!("Materials to load: {:?}", load_materials);

        let mut car_materials = HashMap::<String, Material>::new();

        read_materials(&fname, &load_materials, &mut car_materials)?;

        load_materials.clear();
        for mat in some_materials {
            if !car_materials.contains_key(&mat) {
                load_materials.insert(mat.clone());
            }
        }

        debug!("Extra materials to load: {:?}", load_materials);
        read_materials(&fname, &load_materials, &mut car_materials)?;

        // Load palette from PIX file.
        let mut pal_file_name = fname.as_ref().to_path_buf();
        pal_file_name.set_file_name("DRRENDER.PAL");
        let pal_file_name = path_subst(pal_file_name, "REG/PALETTES".into(), None)?;
        info!("### Opening palette {:?}", pal_file_name);
        let palette = &PixelMap::load_many(pal_file_name)?[0]; // @todo Insert into World resources

        for x in 0..palette.units {
            trace!(
                "Palette alpha {}",
                palette.data[(x * palette.unit_bytes/*+ 0*/) as usize]
            );
        }

        let load_pixmaps: HashSet<_> = load_pixmaps.iter().collect();
        debug!("Pixmaps to load: {:?}", load_pixmaps);

        let mut car_textures = HashMap::<String, PixelMap>::new();
        for pixmap in load_pixmaps {
            let mut pix_file_name = fname.as_ref().to_path_buf();
            pix_file_name.set_file_name(pixmap);
            let pix_file_name = path_subst(pix_file_name, "PIXELMAP".into(), None)?;
            info!("### Opening pixelmap {:?}", pix_file_name);
            let pix = PixelMap::load_many(pix_file_name)?;
            for pmap in pix {
                let pmap = pmap.remap_via_palette(palette)?;
                car_textures.insert(pmap.name.clone(), pmap);
            }
        }

        Ok(Car {
            name: car_name,
            actors: *car_actors,
            meshes: car_meshes,
            materials: car_materials,
            textures: car_textures,
            base_translation: Vector3::from([0f32, 0f32, 0f32]),
        })
    }
}
