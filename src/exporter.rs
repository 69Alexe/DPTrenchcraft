use crate::optimizer::{Brush, BrushKind};
use crate::config::TEXTURE_SCALE;
use std::collections::BTreeSet;
use std::fs::File;
use std::io::Write;
use std::path::{Path, PathBuf};

const HULL_THICKNESS: i32 = 32;
const HULL_TOP_TEXTURE: &str = "sky1";
const HULL_SIDE_TEXTURE: &str = "sky1";
const HULL_BOTTOM_TEXTURE: &str = "sky1";
const NODRAW_TEXTURE: &str = "nodraw";

fn face_textures(base: &str) -> (String, String, String) {
    match base {
        "minecraft/grass_block" => (
            "minecraft/grass_block_top".to_string(),
            "minecraft/dirt".to_string(),
            "minecraft/grass_block_side".to_string(),
        ),
        "minecraft/dirt_path" => (
            "minecraft/dirt_path_top".to_string(),
            "minecraft/dirt".to_string(),
            "minecraft/dirt_path_side".to_string(),
        ),
        "minecraft/chest" | "minecraft/trapped_chest" => (
            "minecraft/chest_top".to_string(),
            "minecraft/chest_top".to_string(),
            "minecraft/chest_side".to_string(),
        ),
        _ => (
            base.to_string(),
            base.to_string(),
            base.to_string(),
        ),
    }
}

fn texture_list_path(output: &Path) -> PathBuf {
    let parent = output.parent().unwrap_or_else(|| Path::new("."));
    let stem = output
        .file_stem()
        .and_then(|s| s.to_str())
        .unwrap_or("map");
    parent.join(format!("{}_textures.txt", stem))
}

fn is_structural_texture(texture: &str) -> bool {
    matches!(
        texture,
        "minecraft/stone"
            | "minecraft/dirt"
            | "minecraft/grass_block"
            | "minecraft/grass_block_top"
            | "minecraft/grass_block_side"
            | "minecraft/dirt_path"
            | "minecraft/dirt_path_top"
            | "minecraft/dirt_path_side"
            | "minecraft/gravel"
            | "minecraft/deepslate"
            | "minecraft/cobbled_deepslate"
            | "minecraft/stone_bricks"
            | "minecraft/cobblestone"
            | "minecraft/andesite"
            | "minecraft/diorite"
            | "minecraft/granite"
            | "minecraft/sandstone"
            | "minecraft/red_sandstone"
            | "minecraft/end_stone"
            | "minecraft/end_stone_bricks"
            | "minecraft/netherrack"
            | "minecraft/blackstone"
            | "minecraft/polished_blackstone"
            | "minecraft/polished_blackstone_bricks"
            | "minecraft/mud_bricks"
            | "minecraft/bricks"
    )
}

fn is_structural_brush(brush: &Brush) -> bool {
    is_structural_texture(&brush.texture)
}

fn write_box_brush_with_textures(
    file: &mut File,
    brush: &Brush,
    top_tex: &str,
    bottom_tex: &str,
    side_tex: &str,
    used_textures: &mut BTreeSet<String>,
) -> std::io::Result<()> {
    used_textures.insert(top_tex.to_string());
    used_textures.insert(bottom_tex.to_string());
    used_textures.insert(side_tex.to_string());

    let x1 = brush.min.0;
    let y1 = brush.min.1;
    let z1 = brush.min.2;
    let x2 = brush.max.0;
    let y2 = brush.max.1;
    let z2 = brush.max.2;

    writeln!(file, "// Brush")?;
    writeln!(file, "{{")?;

    // Top (+Z)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 -1 0 0 ] 0 {} {}",
        x2, y2, z2, x2, y1, z2, x1, y1, z2, top_tex, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    // Bottom (-Z)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 -1 0 0 ] 0 {} {}",
        x2, y1, z1, x2, y2, z1, x1, y2, z1, bottom_tex, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    // Front (+X)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        x2, y2, z2, x2, y2, z1, x2, y1, z1, side_tex, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    // Back (-X)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        x1, y1, z2, x1, y1, z1, x1, y2, z1, side_tex, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    // Right (+Y)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        x2, y2, z2, x1, y2, z2, x1, y2, z1, side_tex, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    // Left (-Y)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        x1, y1, z2, x2, y1, z2, x2, y1, z1, side_tex, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    writeln!(file, "}}")?;
    Ok(())
}

fn write_box_brush(
    file: &mut File,
    brush: &Brush,
    used_textures: &mut BTreeSet<String>,
    global_min_z: i32,
) -> std::io::Result<()> {
    let (top_tex, bottom_tex, side_tex) = face_textures(&brush.texture);

    let bottom_tex_final = if brush.min.2 == global_min_z {
        NODRAW_TEXTURE.to_string()
    } else {
        bottom_tex
    };

    write_box_brush_with_textures(
        file,
        brush,
        &top_tex,
        &bottom_tex_final,
        &side_tex,
        used_textures,
    )
}

fn write_rotated_prism(
    file: &mut File,
    p0: (i32, i32),
    p1: (i32, i32),
    p2: (i32, i32),
    p3: (i32, i32),
    z1: i32,
    z2: i32,
    texture: &str,
) -> std::io::Result<()> {
    let t0 = (p0.0, p0.1, z2);
    let t1 = (p1.0, p1.1, z2);
    let t2 = (p2.0, p2.1, z2);
    let t3 = (p3.0, p3.1, z2);

    let b0 = (p0.0, p0.1, z1);
    let b1 = (p1.0, p1.1, z1);
    let b2 = (p2.0, p2.1, z1);
    let b3 = (p3.0, p3.1, z1);

    writeln!(file, "// Brush")?;
    writeln!(file, "{{")?;

    // Top (+Z)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 -1 0 0 ] 0 {} {}",
        t0.0, t0.1, t0.2, t1.0, t1.1, t1.2, t2.0, t2.1, t2.2, texture, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    // Bottom (-Z)
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 1 0 0 0 ] [ 0 -1 0 0 ] 0 {} {}",
        b0.0, b0.1, b0.2, b2.0, b2.1, b2.2, b1.0, b1.1, b1.2, texture, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    // Sides
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        t1.0, t1.1, t1.2, t0.0, t0.1, t0.2, b0.0, b0.1, b0.2, texture, TEXTURE_SCALE, TEXTURE_SCALE
    )?;
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        t2.0, t2.1, t2.2, t1.0, t1.1, t1.2, b1.0, b1.1, b1.2, texture, TEXTURE_SCALE, TEXTURE_SCALE
    )?;
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        t3.0, t3.1, t3.2, t2.0, t2.1, t2.2, b2.0, b2.1, b2.2, texture, TEXTURE_SCALE, TEXTURE_SCALE
    )?;
    writeln!(
        file,
        "( {} {} {} ) ( {} {} {} ) ( {} {} {} ) {} [ 0 1 0 0 ] [ 0 0 -1 0 ] 0 {} {}",
        t0.0, t0.1, t0.2, t3.0, t3.1, t3.2, b3.0, b3.1, b3.2, texture, TEXTURE_SCALE, TEXTURE_SCALE
    )?;

    writeln!(file, "}}")?;
    Ok(())
}

fn write_plant45_brush(
    file: &mut File,
    brush: &Brush,
    used_textures: &mut BTreeSet<String>,
) -> std::io::Result<()> {
    used_textures.insert(brush.texture.clone());

    let x1 = brush.min.0;
    let y1 = brush.min.1;
    let z1 = brush.min.2;
    let x2 = brush.max.0;
    let y2 = brush.max.1;

    let cx = (x1 + x2) / 2;
    let cy = (y1 + y2) / 2;

    let z2 = z1 + 20;

    let a0 = (cx - 11, cy - 13);
    let a1 = (cx - 13, cy - 11);
    let a2 = (cx + 11, cy + 13);
    let a3 = (cx + 13, cy + 11);

    let b0 = (cx - 13, cy + 11);
    let b1 = (cx - 11, cy + 13);
    let b2 = (cx + 13, cy - 11);
    let b3 = (cx + 11, cy - 13);

    write_rotated_prism(file, a0, a1, a2, a3, z1, z2, &brush.texture)?;
    write_rotated_prism(file, b0, b1, b2, b3, z1, z2, &brush.texture)?;
    Ok(())
}

fn write_brush(
    file: &mut File,
    brush: &Brush,
    used_textures: &mut BTreeSet<String>,
    global_min_z: i32,
) -> std::io::Result<()> {
    match brush.kind {
        BrushKind::Box => write_box_brush(file, brush, used_textures, global_min_z),
        BrushKind::Plant45 => write_plant45_brush(file, brush, used_textures),
    }
}

fn make_hull_brushes(brushes: &[Brush]) -> Vec<Brush> {
    if brushes.is_empty() {
        return Vec::new();
    }

    let mut min_x = brushes[0].min.0;
    let mut min_y = brushes[0].min.1;
    let mut min_z = brushes[0].min.2;
    let mut max_x = brushes[0].max.0;
    let mut max_y = brushes[0].max.1;
    let mut max_z = brushes[0].max.2;

    for b in brushes.iter().skip(1) {
        min_x = min_x.min(b.min.0);
        min_y = min_y.min(b.min.1);
        min_z = min_z.min(b.min.2);
        max_x = max_x.max(b.max.0);
        max_y = max_y.max(b.max.1);
        max_z = max_z.max(b.max.2);
    }

    let t = HULL_THICKNESS;

    vec![
        Brush {
            min: (min_x, min_y, min_z - t),
            max: (max_x, max_y, min_z),
            texture: HULL_BOTTOM_TEXTURE.to_string(),
            kind: BrushKind::Box,
        },
        Brush {
            min: (min_x, min_y, max_z),
            max: (max_x, max_y, max_z + t),
            texture: HULL_TOP_TEXTURE.to_string(),
            kind: BrushKind::Box,
        },
        Brush {
            min: (min_x - t, min_y, min_z),
            max: (min_x, max_y, max_z),
            texture: HULL_SIDE_TEXTURE.to_string(),
            kind: BrushKind::Box,
        },
        Brush {
            min: (max_x, min_y, min_z),
            max: (max_x + t, max_y, max_z),
            texture: HULL_SIDE_TEXTURE.to_string(),
            kind: BrushKind::Box,
        },
        Brush {
            min: (min_x, min_y - t, min_z),
            max: (max_x, min_y, max_z),
            texture: HULL_SIDE_TEXTURE.to_string(),
            kind: BrushKind::Box,
        },
        Brush {
            min: (min_x, max_y, min_z),
            max: (max_x, max_y + t, max_z),
            texture: HULL_SIDE_TEXTURE.to_string(),
            kind: BrushKind::Box,
        },
    ]
}

pub fn export_map(brushes: &[Brush], output: &Path) {
    println!("Exporting {} brushes to {}...", brushes.len(), output.display());

    let mut file = match File::create(output) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create output map file: {:?}", e);
            return;
        }
    };

    let mut used_textures: BTreeSet<String> = BTreeSet::new();

    let mut global_min_z = i32::MAX;
    for b in brushes {
        global_min_z = global_min_z.min(b.min.2);
    }

    let hull_brushes = make_hull_brushes(brushes);

    let mut worldspawn_brushes: Vec<&Brush> = Vec::new();
    let mut detail_brushes: Vec<&Brush> = Vec::new();

    for brush in brushes {
        if is_structural_brush(brush) {
            worldspawn_brushes.push(brush);
        } else {
            detail_brushes.push(brush);
        }
    }

    writeln!(file, "// Game: Digital Paint: Paintball 2").unwrap();
    writeln!(file, "// Format: Quake2 (Valve)").unwrap();

    // worldspawn
    writeln!(file, "// entity 0").unwrap();
    writeln!(file, "{{").unwrap();
    writeln!(file, "\"classname\" \"worldspawn\"").unwrap();
    writeln!(file, "\"mapversion\" \"220\"").unwrap();

    for brush in &worldspawn_brushes {
        write_brush(&mut file, brush, &mut used_textures, global_min_z).unwrap();
    }

    println!("Adding {} hull brushes.", hull_brushes.len());

    for (i, brush) in hull_brushes.iter().enumerate() {
        if i == 0 {
            write_box_brush_with_textures(
                &mut file,
                brush,
                NODRAW_TEXTURE,
                HULL_BOTTOM_TEXTURE,
                HULL_BOTTOM_TEXTURE,
                &mut used_textures,
            )
            .unwrap();
        } else {
            write_brush(&mut file, brush, &mut used_textures, global_min_z).unwrap();
        }
    }

    writeln!(file, "}}").unwrap();

    // func_detail
    if !detail_brushes.is_empty() {
        writeln!(file, "// entity 1").unwrap();
        writeln!(file, "{{").unwrap();
        writeln!(file, "\"classname\" \"func_detail\"").unwrap();

        for brush in &detail_brushes {
            write_brush(&mut file, brush, &mut used_textures, global_min_z).unwrap();
        }

        writeln!(file, "}}").unwrap();
    }

    println!(
        "Export complete. Structural brushes: {}, detail brushes: {}, hull brushes: {}.",
        worldspawn_brushes.len(),
        detail_brushes.len(),
        hull_brushes.len()
    );

    let texture_txt = texture_list_path(output);
    let mut tex_file = match File::create(&texture_txt) {
        Ok(f) => f,
        Err(e) => {
            eprintln!("Failed to create texture list file: {:?}", e);
            return;
        }
    };

    for tex in used_textures {
        writeln!(tex_file, "{}", tex).unwrap();
    }

    println!("Texture list written to {}.", texture_txt.display());
}