use crate::block_shapes::{self, BlockShape};
use crate::parser::{Block, VoxelMap};

fn map_block_to_texture(name: &str) -> String {
    let base = name.strip_prefix("minecraft:").unwrap_or(name);

    let mapped = match base {
        // Wood stairs/slabs -> plank textures
        "oak_stairs" | "oak_slab" => "oak_planks",
        "spruce_stairs" | "spruce_slab" => "spruce_planks",
        "birch_stairs" | "birch_slab" => "birch_planks",
        "jungle_stairs" | "jungle_slab" => "jungle_planks",
        "acacia_stairs" | "acacia_slab" => "acacia_planks",
        "dark_oak_stairs" | "dark_oak_slab" => "dark_oak_planks",
        "mangrove_stairs" | "mangrove_slab" => "mangrove_planks",
        "cherry_stairs" | "cherry_slab" => "cherry_planks",
        "bamboo_stairs" | "bamboo_slab" => "bamboo_planks",
        "bamboo_mosaic_stairs" | "bamboo_mosaic_slab" => "bamboo_mosaic",
        "crimson_stairs" | "crimson_slab" => "crimson_planks",
        "warped_stairs" | "warped_slab" => "warped_planks",

        // Stone-ish stairs/slabs
        "stone_stairs" | "stone_slab" => "stone",
        "cobblestone_stairs" | "cobblestone_slab" => "cobblestone",
        "mossy_cobblestone_stairs" | "mossy_cobblestone_slab" => "mossy_cobblestone",
        "stone_brick_stairs" | "stone_brick_slab" => "stone_bricks",
        "mossy_stone_brick_stairs" | "mossy_stone_brick_slab" => "mossy_stone_bricks",
        "granite_stairs" | "granite_slab" => "granite",
        "polished_granite_stairs" | "polished_granite_slab" => "polished_granite",
        "diorite_stairs" | "diorite_slab" => "diorite",
        "polished_diorite_stairs" | "polished_diorite_slab" => "polished_diorite",
        "andesite_stairs" | "andesite_slab" => "andesite",
        "polished_andesite_stairs" | "polished_andesite_slab" => "polished_andesite",
        "blackstone_stairs" | "blackstone_slab" => "blackstone",
        "polished_blackstone_stairs" | "polished_blackstone_slab" => "polished_blackstone",
        "polished_blackstone_brick_stairs" | "polished_blackstone_brick_slab" => "polished_blackstone_bricks",
        "brick_stairs" | "brick_slab" => "bricks",
        "mud_brick_stairs" | "mud_brick_slab" => "mud_bricks",
        "nether_brick_stairs" | "nether_brick_slab" => "nether_bricks",
        "red_nether_brick_stairs" | "red_nether_brick_slab" => "red_nether_bricks",
        "sandstone_stairs" | "sandstone_slab" => "sandstone",
        "smooth_sandstone_stairs" | "smooth_sandstone_slab" => "smooth_sandstone",
        "red_sandstone_stairs" | "red_sandstone_slab" => "red_sandstone",
        "smooth_red_sandstone_stairs" | "smooth_red_sandstone_slab" => "smooth_red_sandstone",
        "quartz_stairs" | "quartz_slab" => "quartz_block",
        "smooth_quartz_stairs" | "smooth_quartz_slab" => "smooth_quartz",
        "prismarine_stairs" | "prismarine_slab" => "prismarine",
        "prismarine_brick_stairs" | "prismarine_brick_slab" => "prismarine_bricks",
        "dark_prismarine_stairs" | "dark_prismarine_slab" => "dark_prismarine",
        "purpur_stairs" | "purpur_slab" => "purpur_block",
        "end_stone_brick_stairs" | "end_stone_brick_slab" => "end_stone_bricks",
        "deepslate_brick_stairs" | "deepslate_brick_slab" => "deepslate_bricks",
        "deepslate_tile_stairs" | "deepslate_tile_slab" => "deepslate_tiles",
        "cobbled_deepslate_stairs" | "cobbled_deepslate_slab" => "cobbled_deepslate",
        "polished_deepslate_stairs" | "polished_deepslate_slab" => "polished_deepslate",
        "cut_copper_stairs" | "cut_copper_slab" => "cut_copper",
        "exposed_cut_copper_stairs" | "exposed_cut_copper_slab" => "exposed_cut_copper",
        "weathered_cut_copper_stairs" | "weathered_cut_copper_slab" => "weathered_cut_copper",
        "oxidized_cut_copper_stairs" | "oxidized_cut_copper_slab" => "oxidized_cut_copper",
        "waxed_cut_copper_stairs" | "waxed_cut_copper_slab" => "waxed_cut_copper",
        "waxed_exposed_cut_copper_stairs" | "waxed_exposed_cut_copper_slab" => "waxed_exposed_cut_copper",
        "waxed_weathered_cut_copper_stairs" | "waxed_weathered_cut_copper_slab" => "waxed_weathered_cut_copper",
        "waxed_oxidized_cut_copper_stairs" | "waxed_oxidized_cut_copper_slab" => "waxed_oxidized_cut_copper",

        // Fences -> use plank textures
        "oak_fence" => "oak_planks",
        "spruce_fence" => "spruce_planks",
        "birch_fence" => "birch_planks",
        "jungle_fence" => "jungle_planks",
        "acacia_fence" => "acacia_planks",
        "dark_oak_fence" => "dark_oak_planks",
        "mangrove_fence" => "mangrove_planks",
        "cherry_fence" => "cherry_planks",
        "bamboo_fence" => "bamboo_planks",
        "crimson_fence" => "crimson_planks",
        "warped_fence" => "warped_planks",

        // Chests: all chest variants use normal chest textures
        "chest"
        | "trapped_chest"
        | "ender_chest"
        | "copper_chest"
        | "exposed_copper_chest"
        | "weathered_copper_chest"
        | "oxidized_copper_chest"
        | "waxed_copper_chest"
        | "waxed_exposed_copper_chest"
        | "waxed_weathered_copper_chest"
        | "waxed_oxidized_copper_chest" => "chest",

        // Beds: all bed variants use white wool texture
        "white_bed"
        | "orange_bed"
        | "magenta_bed"
        | "light_blue_bed"
        | "yellow_bed"
        | "lime_bed"
        | "pink_bed"
        | "gray_bed"
        | "light_gray_bed"
        | "cyan_bed"
        | "purple_bed"
        | "blue_bed"
        | "brown_bed"
        | "green_bed"
        | "red_bed"
        | "black_bed" => "white_wool",

        // Wood / hyphae blocks -> log/stem textures
        "oak_wood" | "stripped_oak_wood" => "oak_log",
        "spruce_wood" | "stripped_spruce_wood" => "spruce_log",
        "birch_wood" | "stripped_birch_wood" => "birch_log",
        "jungle_wood" | "stripped_jungle_wood" => "jungle_log",
        "acacia_wood" | "stripped_acacia_wood" => "acacia_log",
        "dark_oak_wood" | "stripped_dark_oak_wood" => "dark_oak_log",
        "mangrove_wood" | "stripped_mangrove_wood" => "mangrove_log",
        "cherry_wood" | "stripped_cherry_wood" => "cherry_log",
        "crimson_hyphae" | "stripped_crimson_hyphae" => "crimson_stem",
        "warped_hyphae" | "stripped_warped_hyphae" => "warped_stem",

	// Glass Panes all use the same glass texture (too lazy to make them all for now)
	"glass_pane" 
	| "gray_stained_glass_pane"
	| "gray_stained_glass"
	| "black_stained_glass_pane" 
	| "black_stained_glass"
	| "brown_stained_glass_pane"
	| "brown_stained_glass"
	| "red_stained_glass_pane"
	| "red_stained_glass"
	| "orange_stained_glass_pane" 
	| "orange_stained_glass"
	| "yellow_stained_glass_pane"
	| "yellow_stained_glass"
	| "lime_stained_glass_pane"
	| "lime_stained_glass"
	| "green_stained_glass_pane" 
	| "green_stained_glass"
	| "cyan_stained_glass_pane" 
	| "cyan_stained_glass"
	| "light_blue_stained_glass_pane" 
	| "light_blue_stained_glass"
	| "blue_stained_glass_pane"
	| "blue_stained_glass"
	| "purple_stained_glass_pane"
	| "purple_stained_glass"
	| "magenta_stained_glass_pane"
	| "magenta_stained_glass"
	| "pink_stained_glass_pane"
	| "pink_stained_glass" => "glass",

        _ => base,
    };

    format!("minecraft/{}", mapped)
}

fn is_ignored_plant(block: &Block) -> bool {
    let name = block.name.as_str();

    matches!(
        name,
        "minecraft:dandelion"
            | "minecraft:poppy"
            | "minecraft:blue_orchid"
            | "minecraft:allium"
            | "minecraft:azure_bluet"
            | "minecraft:red_tulip"
            | "minecraft:orange_tulip"
            | "minecraft:white_tulip"
            | "minecraft:pink_tulip"
            | "minecraft:oxeye_daisy"
            | "minecraft:cornflower"
            | "minecraft:lily_of_the_valley"
            | "minecraft:wither_rose"
            | "minecraft:sunflower"
            | "minecraft:lilac"
            | "minecraft:rose_bush"
            | "minecraft:peony"
            | "minecraft:oak_sapling"
            | "minecraft:spruce_sapling"
            | "minecraft:birch_sapling"
            | "minecraft:jungle_sapling"
            | "minecraft:acacia_sapling"
            | "minecraft:dark_oak_sapling"
            | "minecraft:mangrove_propagule"
            | "minecraft:brown_mushroom"
            | "minecraft:red_mushroom"
            | "minecraft:grass"
            | "minecraft:short_grass"
            | "minecraft:tall_grass"
            | "minecraft:fern"
            | "minecraft:large_fern"
            | "minecraft:dead_bush"
            | "minecraft:bush"
	    | "minecraft:wheat"
            | "minecraft:carrots"
            | "minecraft:potatoes"
            | "minecraft:beetroots"
            | "minecraft:nether_wart"
            | "minecraft:sweet_berry_bush"
            | "minecraft:cocoa"
            | "minecraft:melon_stem"
            | "minecraft:attached_melon_stem"
            | "minecraft:pumpkin_stem"
            | "minecraft:attached_pumpkin_stem"
    ) || name.contains("flower")
        || name.contains("sapling")
        || name.contains("mushroom")
}

fn is_empty_block(block: &Block) -> bool {
    block.name == "minecraft:air"
        || block.name == "air"
        || block.name == "minecraft:cave_air"
        || block.name == "cave_air"
        || block.name == "minecraft:void_air"
        || block.name == "void_air"
        || block.name == "minecraft:structure_void"
        || is_ignored_plant(block)
        || block.name == "minecraft:oak_door"
        || block.name == "minecraft:spruce_door"
        || block.name == "minecraft:birch_door"
        || block.name == "minecraft:jungle_door"
        || block.name == "minecraft:acacia_door"
        || block.name == "minecraft:dark_oak_door"
        || block.name == "minecraft:mangrove_door"
        || block.name == "minecraft:cherry_door"
        || block.name == "minecraft:bamboo_door"
        || block.name == "minecraft:crimson_door"
        || block.name == "minecraft:warped_door"
        || block.name == "minecraft:iron_door"
        || block.name == "minecraft:copper_door"
        || block.name == "minecraft:exposed_copper_door"
        || block.name == "minecraft:weathered_copper_door"
        || block.name == "minecraft:oxidized_copper_door"
        || block.name == "minecraft:waxed_copper_door"
        || block.name == "minecraft:waxed_exposed_copper_door"
        || block.name == "minecraft:waxed_weathered_copper_door"
        || block.name == "minecraft:waxed_oxidized_copper_door"
}

fn is_solid_for_boundary_cull(block: &Block) -> bool {
    !is_empty_block(block) && block_shapes::get_shape(block) == BlockShape::Full
}

fn cull_group(block: &Block) -> &str {
    match block.name.as_str() {
        "minecraft:stone"
        | "minecraft:dirt"
        | "minecraft:gravel"
        | "minecraft:coal_ore"
        | "minecraft:iron_ore"
        | "minecraft:copper_ore"
        | "minecraft:gold_ore"
        | "minecraft:redstone_ore"
        | "minecraft:lapis_ore"
        | "minecraft:diamond_ore"
        | "minecraft:emerald_ore" => "stone",

        "minecraft:deepslate"
        | "minecraft:deepslate_coal_ore"
        | "minecraft:deepslate_iron_ore"
        | "minecraft:deepslate_copper_ore"
        | "minecraft:deepslate_gold_ore"
        | "minecraft:deepslate_redstone_ore"
        | "minecraft:deepslate_lapis_ore"
        | "minecraft:deepslate_diamond_ore"
        | "minecraft:deepslate_emerald_ore" => "deepslate",

        _ => block.name.as_str(),
    }
}

fn same_block(a: &Block, b: &Block) -> bool {
    cull_group(a) == cull_group(b)
}

fn is_terrain_family(block: &Block) -> bool {
    matches!(cull_group(block), "stone" | "deepslate")
}

pub fn filter_blocks(mut map: VoxelMap) -> VoxelMap {
    println!("Filtering air blocks and applying textures...");

    for block_opt in &mut map.blocks {
        let is_empty = match block_opt {
            Some(block) => is_empty_block(block),
            None => true,
        };

        if is_empty {
            *block_opt = None;
        } else if let Some(block) = block_opt {
            block.texture = map_block_to_texture(&block.name);
        }
    }

    let solid_snapshot = map.blocks.clone();

    println!("Culling hidden interior full blocks...");

    let w = map.width;
    let h = map.height;
    let l = map.length;
    let original = map.blocks.clone();

    let idx = |x: i32, y: i32, z: i32| -> usize { (y * (w * l) + z * w + x) as usize };

    let mut culled = 0usize;

    for y in 1..(h - 1) {
        for z in 1..(l - 1) {
            for x in 1..(w - 1) {
                let i = idx(x, y, z);

                let center = match &original[i] {
                    Some(b) => b,
                    None => continue,
                };

                if block_shapes::get_shape(center) != BlockShape::Full {
                    continue;
                }

                let neighbors = [
                    &original[idx(x - 1, y, z)],
                    &original[idx(x + 1, y, z)],
                    &original[idx(x, y - 1, z)],
                    &original[idx(x, y + 1, z)],
                    &original[idx(x, y, z - 1)],
                    &original[idx(x, y, z + 1)],
                ];

                let mut surrounded = true;

                for n in neighbors {
                    match n {
                        Some(nb) if same_block(center, nb) => {}
                        _ => {
                            surrounded = false;
                            break;
                        }
                    }
                }

                if surrounded {
                    map.blocks[i] = None;
                    culled += 1;
                }
            }
        }
    }

    println!("Culled {} hidden interior blocks.", culled);

    println!("Culling buried outer-boundary full blocks...");

    let original2 = map.blocks.clone();
    let mut boundary_culled = 0usize;

    for y in 0..h {
        for z in 0..l {
            for x in 0..w {
                let i = idx(x, y, z);

                let center = match &original2[i] {
                    Some(b) => b,
                    None => continue,
                };

                if block_shapes::get_shape(center) != BlockShape::Full {
                    continue;
                }

                let on_boundary = x == 0 || x == w - 1 || y == 0 || y == h - 1 || z == 0 || z == l - 1;

                if !on_boundary {
                    continue;
                }

                let mut buried = true;

                let neighbor_coords = [
                    (x - 1, y, z, x > 0),
                    (x + 1, y, z, x < w - 1),
                    (x, y - 1, z, y > 0),
                    (x, y + 1, z, y < h - 1),
                    (x, y, z - 1, z > 0),
                    (x, y, z + 1, z < l - 1),
                ];

                for (nx, ny, nz, exists) in neighbor_coords {
                    if !exists {
                        continue;
                    }

                    match &original2[idx(nx, ny, nz)] {
                        Some(nb) if is_solid_for_boundary_cull(nb) => {}
                        _ => {
                            buried = false;
                            break;
                        }
                    }
                }

                if buried {
                    map.blocks[i] = None;
                    boundary_culled += 1;
                }
            }
        }
    }

    println!("Culled {} buried boundary blocks.", boundary_culled);

    println!("Culling lowest-floor full blocks...");

    let original3 = map.blocks.clone();
    let mut floor_culled = 0usize;

    let mut lowest_used_y: Option<i32> = None;

    for y in 0..h {
        let mut found = false;
        for z in 0..l {
            for x in 0..w {
                if original3[idx(x, y, z)].is_some() {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            lowest_used_y = Some(y);
            break;
        }
    }

    if let Some(lowest_y) = lowest_used_y {
        for z in 0..l {
            for x in 0..w {
                let i = idx(x, lowest_y, z);

                let center = match &original3[i] {
                    Some(b) => b,
                    None => continue,
                };

                if block_shapes::get_shape(center) == BlockShape::Full {
                    map.blocks[i] = None;
                    floor_culled += 1;
                }
            }
        }
    }

    println!("Culled {} lowest-floor blocks.", floor_culled);

        println!("Culling hidden dirt under grass blocks...");

    let mut grass_dirt_culled = 0usize;

    for y in 0..(h - 1) {
        for z in 0..l {
            for x in 0..w {
                let i = idx(x, y, z);

                let center = match &solid_snapshot[i] {
                    Some(b) => b,
                    None => continue,
                };

                if center.name != "minecraft:dirt" {
                    continue;
                }

                let above = match &solid_snapshot[idx(x, y + 1, z)] {
                    Some(b) => b,
                    None => continue,
                };

                if above.name != "minecraft:grass_block" {
                    continue;
                }

                let mut next_to_path = false;

                let path_neighbors = [
                    (x - 1, y, z, x > 0),
                    (x + 1, y, z, x < w - 1),
                    (x, y, z - 1, z > 0),
                    (x, y, z + 1, z < l - 1),
                    (x - 1, y + 1, z, x > 0),
                    (x + 1, y + 1, z, x < w - 1),
                    (x, y + 1, z - 1, z > 0),
                    (x, y + 1, z + 1, z < l - 1),
                ];

                for (nx, ny, nz, exists) in path_neighbors {
                    if !exists {
                        continue;
                    }

                    if let Some(nb) = &solid_snapshot[idx(nx, ny, nz)] {
                        if nb.name == "minecraft:dirt_path" {
                            next_to_path = true;
                            break;
                        }
                    }
                }

                if next_to_path {
                    continue;
                }

                let side_neighbors = [
                    (x - 1, y, z, x > 0),
                    (x + 1, y, z, x < w - 1),
                    (x, y, z - 1, z > 0),
                    (x, y, z + 1, z < l - 1),
                ];

                let mut hidden_from_side = true;

                for (nx, ny, nz, exists) in side_neighbors {
                    if !exists {
                        hidden_from_side = false;
                        break;
                    }

                    match &solid_snapshot[idx(nx, ny, nz)] {
                        Some(nb) if is_solid_for_boundary_cull(nb) => {}
                        _ => {
                            hidden_from_side = false;
                            break;
                        }
                    }
                }

                let has_support_below = if y > 0 {
    matches!(
        &solid_snapshot[idx(x, y - 1, z)],
        Some(nb) if is_solid_for_boundary_cull(nb)
    )
} else {
    false
};

if hidden_from_side && has_support_below && map.blocks[i].is_some() {
    map.blocks[i] = None;
    grass_dirt_culled += 1;
}
            }
        }
    }

    println!("Culled {} hidden dirt-under-grass blocks.", grass_dirt_culled);

    println!("Culling buried outer side-wall full blocks...");

    let mut side_culled = 0usize;

    let mut lowest_used_x: Option<i32> = None;
    let mut highest_used_x: Option<i32> = None;
    let mut lowest_used_z: Option<i32> = None;
    let mut highest_used_z: Option<i32> = None;

    for x in 0..w {
        let mut found = false;
        for y in 0..h {
            for z in 0..l {
                if solid_snapshot[idx(x, y, z)].is_some() {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            lowest_used_x = Some(x);
            break;
        }
    }

    for x in (0..w).rev() {
        let mut found = false;
        for y in 0..h {
            for z in 0..l {
                if solid_snapshot[idx(x, y, z)].is_some() {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            highest_used_x = Some(x);
            break;
        }
    }

    for z in 0..l {
        let mut found = false;
        for y in 0..h {
            for x in 0..w {
                if solid_snapshot[idx(x, y, z)].is_some() {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            lowest_used_z = Some(z);
            break;
        }
    }

    for z in (0..l).rev() {
        let mut found = false;
        for y in 0..h {
            for x in 0..w {
                if solid_snapshot[idx(x, y, z)].is_some() {
                    found = true;
                    break;
                }
            }
            if found {
                break;
            }
        }
        if found {
            highest_used_z = Some(z);
            break;
        }
    }

    for y in 0..h {
        for z in 0..l {
            for x in 0..w {
                let i = idx(x, y, z);

                let center = match &solid_snapshot[i] {
                    Some(b) => b,
                    None => continue,
                };

                if block_shapes::get_shape(center) != BlockShape::Full {
                    continue;
                }

                if !is_terrain_family(center) {
                    continue;
                }

                let on_outer_side = Some(x) == lowest_used_x
                    || Some(x) == highest_used_x
                    || Some(z) == lowest_used_z
                    || Some(z) == highest_used_z;

                if !on_outer_side {
                    continue;
                }

                let mut buried = true;

                let neighbor_coords = [
                    (x - 1, y, z, x > 0),
                    (x + 1, y, z, x < w - 1),
                    (x, y - 1, z, y > 0),
                    (x, y + 1, z, y < h - 1),
                    (x, y, z - 1, z > 0),
                    (x, y, z + 1, z < l - 1),
                ];

                for (nx, ny, nz, exists) in neighbor_coords {
                    if !exists {
                        continue;
                    }

                    match &solid_snapshot[idx(nx, ny, nz)] {
                        Some(nb) if is_solid_for_boundary_cull(nb) => {}
                        _ => {
                            buried = false;
                            break;
                        }
                    }
                }

                if buried && map.blocks[i].is_some() {
                    map.blocks[i] = None;
                    side_culled += 1;
                }
            }
        }
    }

    println!("Culled {} buried outer side-wall blocks.", side_culled);

    map
}