use crate::parser::Block;

#[derive(Copy, Clone, PartialEq)]
pub enum BlockShape {
    Full,
    SlabBottom,
    SlabTop,
    ThinPanelNS,
    ThinPanelEW,
    ThinCross,
    Path,

    PlantCross,
    Torch,
    Chest,
    Bed,

    StairNorthBottom,
    StairSouthBottom,
    StairEastBottom,
    StairWestBottom,
    StairNorthTop,
    StairSouthTop,
    StairEastTop,
    StairWestTop,
}

pub fn get_shape(block: &Block) -> BlockShape {
    let name = &block.name;

    // Dirt path
    if name == "minecraft:dirt_path" {
        return BlockShape::Path;
    }

    // Chests
    if matches!(
        name.as_str(),
        "minecraft:chest"
            | "minecraft:trapped_chest"
            | "minecraft:ender_chest"
            | "minecraft:copper_chest"
            | "minecraft:exposed_copper_chest"
            | "minecraft:weathered_copper_chest"
            | "minecraft:oxidized_copper_chest"
            | "minecraft:waxed_copper_chest"
            | "minecraft:waxed_exposed_copper_chest"
            | "minecraft:waxed_weathered_copper_chest"
            | "minecraft:waxed_oxidized_copper_chest"
    ) {
        return BlockShape::Chest;
    }

    // Plants / flowers
    let is_plant = matches!(
        name.as_str(),
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
    ) || name.contains("flower") || name.contains("sapling") || name.contains("mushroom");

    if is_plant {
        return BlockShape::PlantCross;
    }

    // Torches
    if name.contains("torch") {
        return BlockShape::Torch;
    }
    // bed
    if name.ends_with("_bed") {
    return BlockShape::Bed;
    }
    // Slabs
    if name.ends_with("_slab") {
    	let slab_type = block
        	.properties
        	.get("type")
        	.map(|s| s.as_str())
        	.unwrap_or("bottom");

    	return match slab_type {
        	"top" => BlockShape::SlabTop,
        	"bottom" => BlockShape::SlabBottom,
        	"double" => BlockShape::Full, // treat as full block
        	_ => BlockShape::SlabBottom,
    	};
     }

    // Stairs
    if name.ends_with("_stairs") {
        let half = block.properties.get("half").map(|s| s.as_str()).unwrap_or("bottom");
        let facing = block.properties.get("facing").map(|s| s.as_str()).unwrap_or("north");

        return match (half, facing) {
            ("top", "north") => BlockShape::StairNorthTop,
            ("top", "south") => BlockShape::StairSouthTop,
            ("top", "east") => BlockShape::StairEastTop,
            ("top", "west") => BlockShape::StairWestTop,
            ("bottom", "north") => BlockShape::StairNorthBottom,
            ("bottom", "south") => BlockShape::StairSouthBottom,
            ("bottom", "east") => BlockShape::StairEastBottom,
            ("bottom", "west") => BlockShape::StairWestBottom,
            _ => BlockShape::StairNorthBottom,
        };
    }

    // Thin blocks
    let is_thin = name.ends_with("_fence")
        || name.contains("glass_pane")
        || name.contains("iron_bars")
	|| name.contains("nether_portal");
	

    if is_thin {
        let prop_true = |key: &str| -> bool {
            block.properties
                .get(key)
                .map(|v| v != "false" && v != "none")
                .unwrap_or(false)
        };
        let ns = prop_true("north") || prop_true("south");
        let ew = prop_true("east") || prop_true("west");
        return match (ns, ew) {
            (true, false) => BlockShape::ThinPanelNS,
            (false, true) => BlockShape::ThinPanelEW,
            _ => BlockShape::ThinCross,
        };
    }

    BlockShape::Full
}