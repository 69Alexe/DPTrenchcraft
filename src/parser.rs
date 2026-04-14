use fastnbt::Value;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Block {
    pub name: String,
    pub texture: String,
    pub properties: HashMap<String, String>,
}

pub struct VoxelMap {
    pub width: i32,
    pub height: i32,
    pub length: i32,
    pub blocks: Vec<Option<Block>>,
}

pub fn parse_nbt(data: &Value) -> Result<VoxelMap, String> {
    let root = match data {
        Value::Compound(map) => map,
        other => {
            return Err(format!("Top-level NBT is not a Compound: {:?}", other));
        }
    };

    println!("Top-level keys: {:?}", root.keys().collect::<Vec<_>>());

    let schematic = if let Some(Value::Compound(inner)) = root.get("Schematic") {
        println!("Schematic keys: {:?}", inner.keys().collect::<Vec<_>>());
        inner
    } else {
        root
    };

    println!(
        "Active schematic keys: {:?}",
        schematic.keys().collect::<Vec<_>>()
    );

    let get_int = |key: &str| -> Result<i32, String> {
        match schematic.get(key) {
            Some(Value::Byte(b)) => Ok(*b as i32),
            Some(Value::Short(s)) => Ok(*s as i32),
            Some(Value::Int(i)) => Ok(*i),
            Some(other) => Err(format!("Field '{}' has unexpected type: {:?}", key, other)),
            None => Err(format!("Missing required field '{}'", key)),
        }
    };

    let width = get_int("Width")?;
    let height = get_int("Height")?;
    let length = get_int("Length")?;

    if width <= 0 || height <= 0 || length <= 0 {
        return Err(format!(
            "Invalid dimensions: Width={}, Height={}, Length={}",
            width, height, length
        ));
    }

    let total_blocks_i64 = width as i64 * height as i64 * length as i64;
    if total_blocks_i64 <= 0 {
        return Err(format!(
            "Block volume is invalid: {} x {} x {} = {}",
            width, height, length, total_blocks_i64
        ));
    }

    let total_blocks = usize::try_from(total_blocks_i64).map_err(|_| {
        format!(
            "Block volume is too large for this platform: {} x {} x {} = {}",
            width, height, length, total_blocks_i64
        )
    })?;

    let (palette_value, block_data_value) =
        if let Some(Value::Compound(blocks)) = schematic.get("Blocks") {
            println!("Blocks keys: {:?}", blocks.keys().collect::<Vec<_>>());

            let palette = blocks.get("Palette");
            let data = blocks.get("Data").or_else(|| blocks.get("BlockData"));

            (palette, data)
        } else {
            (
                schematic.get("Palette"),
                schematic.get("BlockData").or_else(|| schematic.get("Data")),
            )
        };

    let mut palette_map: HashMap<i32, Block> = HashMap::new();

    match palette_value {
        Some(Value::Compound(palette)) => {
            println!("Palette found with {} entries.", palette.len());

            for (name, val) in palette {
                let id = match val {
                    Value::Int(i) => *i,
                    Value::Short(s) => *s as i32,
                    Value::Byte(b) => *b as i32,
                    other => {
                        return Err(format!(
                            "Palette entry '{}' has unexpected type: {:?}",
                            name, other
                        ));
                    }
                };

                let block_name = name.split('[').next().unwrap_or(name).to_string();
                let props_str = name
                    .split('[')
                    .nth(1)
                    .unwrap_or("")
                    .trim_end_matches(']');

                let mut properties = HashMap::new();
                if !props_str.is_empty() {
                    for pair in props_str.split(',') {
                        let mut kv = pair.splitn(2, '=');
                        if let (Some(k), Some(v)) = (kv.next(), kv.next()) {
                            if !k.is_empty() {
                                properties.insert(k.to_string(), v.to_string());
                            }
                        }
                    }
                }

                palette_map.insert(
                    id,
                    Block {
                        texture: block_name.clone(),
                        name: block_name,
                        properties,
                    },
                );
            }
        }
        Some(other) => {
            return Err(format!("Palette has unexpected type: {:?}", other));
        }
        None => {
            return Err("Missing Palette compound".to_string());
        }
    }

    if palette_map.is_empty() {
        return Err("Palette was present but empty".to_string());
    }

    let block_data = match block_data_value {
        Some(Value::ByteArray(data)) => data,
        Some(other) => {
            return Err(format!("Block data has unexpected type: {:?}", other));
        }
        None => {
            return Err("Missing block data (Data or BlockData)".to_string());
        }
    };

    let mut blocks = vec![None; total_blocks];
    let mut cursor = 0usize;

    for ptr in 0..total_blocks {
        if cursor >= block_data.len() {
            return Err(format!(
                "Block data ended early at block {} of {}",
                ptr, total_blocks
            ));
        }

        let mut val = 0i32;
        let mut shift = 0u32;

        loop {
            if cursor >= block_data.len() {
                return Err(format!(
                    "Unexpected end of block data while decoding VarInt at block {}",
                    ptr
                ));
            }

            let b = block_data[cursor] as u8;
            cursor += 1;

            val |= ((b & 0x7F) as i32) << shift;

            if (b & 0x80) == 0 {
                break;
            }

            shift += 7;
            if shift > 28 {
                return Err(format!("VarInt too large at block {}", ptr));
            }
        }

        match palette_map.get(&val) {
            Some(block) => {
                blocks[ptr] = Some(block.clone());
            }
            None => {
                return Err(format!(
                    "Palette ID {} at block {} not found in Palette",
                    val, ptr
                ));
            }
        }
    }

    println!("Successfully parsed {} blocks.", blocks.len());

    Ok(VoxelMap {
        width,
        height,
        length,
        blocks,
    })
}

impl VoxelMap {
    pub fn from_litematic(schem: &rustmatica::Litematic) -> Result<VoxelMap, String> {
        if schem.regions.is_empty() {
            return Err("Litematic contains no regions".to_string());
        }

        let region = &schem.regions[0];
        let size = &region.size;

        let width = size.x.abs();
        let height = size.y.abs();
        let length = size.z.abs();

        if width <= 0 || height <= 0 || length <= 0 {
            return Err(format!(
                "Invalid litematic region size: x={}, y={}, z={}",
                size.x, size.y, size.z
            ));
        }

        let total_blocks_i64 = width as i64 * height as i64 * length as i64;
        if total_blocks_i64 <= 0 {
            return Err(format!(
                "Litematic region volume is invalid: {} x {} x {} = {}",
                width, height, length, total_blocks_i64
            ));
        }

        let total_blocks = usize::try_from(total_blocks_i64).map_err(|_| {
            format!(
                "Litematic region volume is too large for this platform: {} x {} x {} = {}",
                width, height, length, total_blocks_i64
            )
        })?;

        let mut blocks = vec![None; total_blocks];

        let start_x = if size.x < 0 { size.x + 1 } else { 0 };
        let start_y = if size.y < 0 { size.y + 1 } else { 0 };
        let start_z = if size.z < 0 { size.z + 1 } else { 0 };

        for y in 0..height {
            for z in 0..length {
                for x in 0..width {
                    let rx = start_x + x;
                    let ry = start_y + y;
                    let rz = start_z + z;

                    let mc_block = region.get_block(mcdata::util::BlockPos::new(rx, ry, rz));
                    let block_name = mc_block.name.to_string();

                    let idx = (y * (width * length) + z * width + x) as usize;
                    blocks[idx] = Some(Block {
                        texture: block_name.clone(),
                        name: block_name,
                        properties: HashMap::new(),
                    });
                }
            }
        }

        println!("Successfully extracted {} blocks from Litematic.", blocks.len());

        Ok(VoxelMap {
            width,
            height,
            length,
            blocks,
        })
    }
}