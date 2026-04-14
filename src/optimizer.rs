use clap::ValueEnum;

use crate::config::BLOCK_UNIT;
use crate::block_shapes::{self, BlockShape};
use crate::parser::{Block, VoxelMap};

const HALF_BLOCK: i32 = BLOCK_UNIT / 2;

const PATH_DROP: i32 = (BLOCK_UNIT * 3) / 32;

const THIN_MIN: i32 = (BLOCK_UNIT * 14) / 32;
const THIN_MAX: i32 = (BLOCK_UNIT * 18) / 32;

const THIN_PANEL_HALF_WIDTH: i32 = 4;
const THIN_PANEL_MIN: i32 = (BLOCK_UNIT / 2) - THIN_PANEL_HALF_WIDTH;
const THIN_PANEL_MAX: i32 = (BLOCK_UNIT / 2) + THIN_PANEL_HALF_WIDTH;

const TORCH_TOP: i32 = (BLOCK_UNIT * 24) / 32;

const CHEST_INSET: i32 = (BLOCK_UNIT * 2) / 32;
const CHEST_DROP: i32 = (BLOCK_UNIT * 4) / 32;

#[derive(ValueEnum, Copy, Clone, Debug, PartialEq, Default)]
pub enum GreedyLevel {
    /// No merging — one brush per block
    None,
    /// Merge full blocks only
    #[value(name = "full-only")]
    FullOnly,
    /// Merge all compatible shapes (default)
    #[default]
    All,
}

#[derive(Copy, Clone, Debug, PartialEq)]
pub enum BrushKind {
    Box,
    Plant45,
}

pub struct Brush {
    pub min: (i32, i32, i32),
    pub max: (i32, i32, i32),
    pub texture: String,
    pub kind: BrushKind,
}

#[derive(Copy, Clone)]
enum Axis {
    X,
    Y,
    Z,
}

const AXES_XZ: &[Axis] = &[Axis::X, Axis::Z];
const AXES_ZY: &[Axis] = &[Axis::Z, Axis::Y];
const AXES_XY: &[Axis] = &[Axis::X, Axis::Y];

fn to_quake(bx: i32, bz: i32, by: i32, map_width: i32, map_length: i32) -> (i32, i32, i32) {
    let center_x = map_width / 2;
    let center_z = map_length / 2;

    (
        (bx - center_x) * BLOCK_UNIT,
        (bz - center_z) * BLOCK_UNIT,
        by * BLOCK_UNIT,
    )
}

fn is_shape_match(
    bx: i32,
    by: i32,
    bz: i32,
    texture: &str,
    shape: BlockShape,
    visited: &[bool],
    blocks: &[Option<Block>],
    w: i32,
    l: i32,
) -> bool {
    let i = (by * (w * l) + bz * w + bx) as usize;
    if visited[i] {
        return false;
    }
    if let Some(b) = &blocks[i] {
        b.texture == texture && block_shapes::get_shape(b) == shape
    } else {
        false
    }
}

fn greedy_expand(
    x: i32,
    y: i32,
    z: i32,
    texture: &str,
    shape: BlockShape,
    axes: &[Axis],
    map: &VoxelMap,
    visited: &[bool],
) -> (i32, i32, i32) {
    let w = map.width;
    let h = map.height;
    let l = map.length;

    let mut ex = x + 1;
    let mut ey = y + 1;
    let mut ez = z + 1;

    for &axis in axes {
        match axis {
            Axis::X => {
                'x: loop {
                    if ex >= w {
                        break;
                    }
                    for cy in y..ey {
                        for cz in z..ez {
                            if !is_shape_match(ex, cy, cz, texture, shape, visited, &map.blocks, w, l)
                            {
                                break 'x;
                            }
                        }
                    }
                    ex += 1;
                }
            }
            Axis::Y => {
                'y: loop {
                    if ey >= h {
                        break;
                    }
                    for cx in x..ex {
                        for cz in z..ez {
                            if !is_shape_match(cx, ey, cz, texture, shape, visited, &map.blocks, w, l)
                            {
                                break 'y;
                            }
                        }
                    }
                    ey += 1;
                }
            }
            Axis::Z => {
                'z: loop {
                    if ez >= l {
                        break;
                    }
                    for cx in x..ex {
                        for cy in y..ey {
                            if !is_shape_match(cx, cy, ez, texture, shape, visited, &map.blocks, w, l)
                            {
                                break 'z;
                            }
                        }
                    }
                    ez += 1;
                }
            }
        }
    }

    (ex, ey, ez)
}

fn stair_brushes(
    qx1: i32,
    qy1: i32,
    qz1: i32,
    qx2: i32,
    qy2: i32,
    qz2: i32,
    shape: BlockShape,
    texture: String,
) -> Vec<Brush> {
    match shape {
        BlockShape::StairNorthBottom => vec![
            Brush {
                min: (qx1, qy1, qz1),
                max: (qx2, qy2, qz1 + HALF_BLOCK),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1, qy1, qz1 + HALF_BLOCK),
                max: (qx2, qy1 + HALF_BLOCK, qz2),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::StairSouthBottom => vec![
            Brush {
                min: (qx1, qy1, qz1),
                max: (qx2, qy2, qz1 + HALF_BLOCK),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1, qy1 + HALF_BLOCK, qz1 + HALF_BLOCK),
                max: (qx2, qy2, qz2),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::StairEastBottom => vec![
            Brush {
                min: (qx1, qy1, qz1),
                max: (qx2, qy2, qz1 + HALF_BLOCK),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1 + HALF_BLOCK, qy1, qz1 + HALF_BLOCK),
                max: (qx2, qy2, qz2),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::StairWestBottom => vec![
            Brush {
                min: (qx1, qy1, qz1),
                max: (qx2, qy2, qz1 + HALF_BLOCK),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1, qy1, qz1 + HALF_BLOCK),
                max: (qx1 + HALF_BLOCK, qy2, qz2),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::StairNorthTop => vec![
            Brush {
                min: (qx1, qy1, qz1 + HALF_BLOCK),
                max: (qx2, qy2, qz2),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1, qy1, qz1),
                max: (qx2, qy1 + HALF_BLOCK, qz1 + HALF_BLOCK),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::StairSouthTop => vec![
            Brush {
                min: (qx1, qy1, qz1 + HALF_BLOCK),
                max: (qx2, qy2, qz2),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1, qy1 + HALF_BLOCK, qz1),
                max: (qx2, qy2, qz1 + HALF_BLOCK),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::StairEastTop => vec![
            Brush {
                min: (qx1, qy1, qz1 + HALF_BLOCK),
                max: (qx2, qy2, qz2),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1 + HALF_BLOCK, qy1, qz1),
                max: (qx2, qy2, qz1 + HALF_BLOCK),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::StairWestTop => vec![
            Brush {
                min: (qx1, qy1, qz1 + HALF_BLOCK),
                max: (qx2, qy2, qz2),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1, qy1, qz1),
                max: (qx1 + HALF_BLOCK, qy2, qz1 + HALF_BLOCK),
                texture,
                kind: BrushKind::Box,
            },
        ],
        _ => vec![Brush {
            min: (qx1, qy1, qz1),
            max: (qx2, qy2, qz2),
            texture,
            kind: BrushKind::Box,
        }],
    }
}

fn single_block_brush(
    x: i32,
    y: i32,
    z: i32,
    shape: BlockShape,
    texture: String,
    map_width: i32,
    map_length: i32,
) -> Vec<Brush> {
    let (qx1, qy1, qz1) = to_quake(x, z, y, map_width, map_length);
    let (qx2, qy2, qz2) = to_quake(x + 1, z + 1, y + 1, map_width, map_length);

    match shape {
        BlockShape::SlabBottom => vec![Brush {
            min: (qx1, qy1, qz1),
            max: (qx2, qy2, qz1 + HALF_BLOCK),
            texture,
            kind: BrushKind::Box,
        }],
        BlockShape::Bed => vec![Brush {
            min: (qx1, qy1, qz1),
            max: (qx2, qy2, qz1 + HALF_BLOCK),
            texture,
            kind: BrushKind::Box,
        }],
        BlockShape::SlabTop => vec![Brush {
            min: (qx1, qy1, qz1 + HALF_BLOCK),
            max: (qx2, qy2, qz2),
            texture,
            kind: BrushKind::Box,
        }],
        BlockShape::ThinPanelNS => vec![Brush {
    	    min: (qx1 + THIN_PANEL_MIN, qy1, qz1),
    	    max: (qx1 + THIN_PANEL_MAX, qy2, qz2),
    	    texture,
    	    kind: BrushKind::Box,
	}],
	BlockShape::ThinPanelEW => vec![Brush {
    	    min: (qx1, qy1 + THIN_PANEL_MIN, qz1),
    	    max: (qx2, qy1 + THIN_PANEL_MAX, qz2),
    	    texture,
    	    kind: BrushKind::Box,
	}],
        BlockShape::ThinCross => vec![
            Brush {
                min: (qx1 + THIN_PANEL_MIN, qy1, qz1),
                max: (qx1 + THIN_PANEL_MAX, qy2, qz2),
                texture: texture.clone(),
                kind: BrushKind::Box,
            },
            Brush {
                min: (qx1, qy1 + THIN_PANEL_MIN, qz1),
                max: (qx2, qy1 + THIN_PANEL_MAX, qz2),
                texture,
                kind: BrushKind::Box,
            },
        ],
        BlockShape::Path => vec![Brush {
            min: (qx1, qy1, qz1),
            max: (qx2, qy2, qz2 - PATH_DROP),
            texture,
            kind: BrushKind::Box,
        }],
        BlockShape::PlantCross => vec![Brush {
            min: (qx1, qy1, qz1),
            max: (qx2, qy2, qz1 + HALF_BLOCK),
            texture,
            kind: BrushKind::Plant45,
        }],
        BlockShape::Torch => vec![Brush {
            min: (qx1 + THIN_MIN, qy1 + THIN_MIN, qz1),
            max: (qx1 + THIN_MAX, qy1 + THIN_MAX, qz1 + TORCH_TOP),
            texture,
            kind: BrushKind::Box,
        }],
        BlockShape::Chest => vec![Brush {
            min: (qx1 + CHEST_INSET, qy1 + CHEST_INSET, qz1 + CHEST_INSET),
            max: (qx2 - CHEST_INSET, qy2 - CHEST_INSET, qz2 - CHEST_INSET),
            texture,
            kind: BrushKind::Box,
        }],
        BlockShape::StairNorthBottom
        | BlockShape::StairSouthBottom
        | BlockShape::StairEastBottom
        | BlockShape::StairWestBottom
        | BlockShape::StairNorthTop
        | BlockShape::StairSouthTop
        | BlockShape::StairEastTop
        | BlockShape::StairWestTop => stair_brushes(qx1, qy1, qz1, qx2, qy2, qz2, shape, texture),
        BlockShape::Full => vec![Brush {
            min: (qx1, qy1, qz1),
            max: (qx2, qy2, qz2),
            texture,
            kind: BrushKind::Box,
        }],
    }
}

fn full_block_texture_at<'a>(
    map: &'a VoxelMap,
    visited: &[bool],
    x: i32,
    y: i32,
    z: i32,
) -> Option<&'a str> {
    if x < 0 || x >= map.width || y < 0 || y >= map.height || z < 0 || z >= map.length {
        return None;
    }

    let idx = (y * (map.width * map.length) + z * map.width + x) as usize;
    if visited[idx] {
        return None;
    }

    match &map.blocks[idx] {
        Some(block) if block_shapes::get_shape(block) == BlockShape::Full => Some(block.texture.as_str()),
        _ => None,
    }
}

fn find_best_full_rect_on_layer(
    map: &VoxelMap,
    visited: &[bool],
    y: i32,
) -> Option<(i32, i32, i32, i32, String)> {
    let mut best: Option<(i32, i32, i32, i32, String)> = None;
    let mut best_area = 0i32;

    for z in 0..map.length {
        for x in 0..map.width {
            let Some(texture) = full_block_texture_at(map, visited, x, y, z) else {
                continue;
            };

            let mut max_width = 0;
            while x + max_width < map.width {
                match full_block_texture_at(map, visited, x + max_width, y, z) {
                    Some(t) if t == texture => max_width += 1,
                    _ => break,
                }
            }

            let mut min_width = max_width;
            let mut depth = 0;

            while z + depth < map.length {
                let mut row_width = 0;
                while row_width < min_width {
                    match full_block_texture_at(map, visited, x + row_width, y, z + depth) {
                        Some(t) if t == texture => row_width += 1,
                        _ => break,
                    }
                }

                if row_width == 0 {
                    break;
                }

                min_width = min_width.min(row_width);
                let area = min_width * (depth + 1);

                if area > best_area {
                    best_area = area;
                    best = Some((x, z, min_width, depth + 1, texture.to_string()));
                }

                depth += 1;
            }
        }
    }

    best
}

fn can_extend_full_rect_up(
    map: &VoxelMap,
    visited: &[bool],
    x: i32,
    y: i32,
    z: i32,
    width: i32,
    depth: i32,
    texture: &str,
) -> bool {
    if y >= map.height {
        return false;
    }

    for dz in 0..depth {
        for dx in 0..width {
            match full_block_texture_at(map, visited, x + dx, y, z + dz) {
                Some(t) if t == texture => {}
                _ => return false,
            }
        }
    }

    true
}

fn mark_full_rect_visited(
    map: &VoxelMap,
    visited: &mut [bool],
    x: i32,
    y: i32,
    z: i32,
    width: i32,
    height: i32,
    depth: i32,
) {
    for dy in 0..height {
        for dz in 0..depth {
            for dx in 0..width {
                let idx = ((y + dy) * (map.width * map.length) + (z + dz) * map.width + (x + dx)) as usize;
                visited[idx] = true;
            }
        }
    }
}

fn optimize_full_blocks_best_rectangles(map: &VoxelMap, visited: &mut [bool], brushes: &mut Vec<Brush>) {
    for y in 0..map.height {
        loop {
            let Some((x, z, width, depth, texture)) = find_best_full_rect_on_layer(map, visited, y) else {
                break;
            };

            let mut height = 1;
            while can_extend_full_rect_up(map, visited, x, y + height, z, width, depth, &texture) {
                height += 1;
            }

            mark_full_rect_visited(map, visited, x, y, z, width, height, depth);

            let (qx1, qy1, qz1) = to_quake(x, z, y, map.width, map.length);
            let (qx2, qy2, qz2) = to_quake(x + width, z + depth, y + height, map.width, map.length);

            brushes.push(Brush {
                min: (qx1, qy1, qz1),
                max: (qx2, qy2, qz2),
                texture,
                kind: BrushKind::Box,
            });
        }
    }
}

pub fn optimize_mesh(map: &VoxelMap, level: GreedyLevel) -> Vec<Brush> {
    println!("Optimizing geometry... (greedy: {:?})", level);
    let mut brushes = Vec::new();
    let mut visited = vec![false; map.blocks.len()];

    if level != GreedyLevel::None {
        optimize_full_blocks_best_rectangles(map, &mut visited, &mut brushes);
    }

    let w = map.width;
    let h = map.height;
    let l = map.length;

    let get_idx = |x, y, z| (y * (w * l) + z * w + x) as usize;

    for y in 0..h {
        for z in 0..l {
            for x in 0..w {
                let idx = get_idx(x, y, z);
                if visited[idx] {
                    continue;
                }

                let block = match &map.blocks[idx] {
                    Some(b) => b.clone(),
                    None => continue,
                };

                let texture = block.texture.clone();
                let shape = block_shapes::get_shape(&block);

                let axes: Option<&[Axis]> = match (level, shape) {
                    (GreedyLevel::None, _) => None,
                    (GreedyLevel::FullOnly, BlockShape::Full) => None,
                    (GreedyLevel::FullOnly, _) => None,
                    (GreedyLevel::All, BlockShape::Full) => None,
                    (GreedyLevel::All, BlockShape::SlabBottom | BlockShape::SlabTop) => Some(AXES_XZ),
                    (GreedyLevel::All, BlockShape::ThinPanelNS) => Some(AXES_ZY),
                    (GreedyLevel::All, BlockShape::ThinPanelEW) => Some(AXES_XY),
                    (GreedyLevel::All, BlockShape::ThinCross) => None,
                    (GreedyLevel::All, BlockShape::Path) => None,
                    (GreedyLevel::All, BlockShape::PlantCross) => None,
                    (GreedyLevel::All, BlockShape::Torch) => None,
                    (GreedyLevel::All, BlockShape::Chest) => None,
                    (GreedyLevel::All, BlockShape::Bed) => None,
                    (
                        GreedyLevel::All,
                        BlockShape::StairNorthBottom
                            | BlockShape::StairSouthBottom
                            | BlockShape::StairEastBottom
                            | BlockShape::StairWestBottom
                            | BlockShape::StairNorthTop
                            | BlockShape::StairSouthTop
                            | BlockShape::StairEastTop
                            | BlockShape::StairWestTop,
                    ) => None,
                };

                if let Some(axes) = axes {
                    let (ex, ey, ez) =
                        greedy_expand(x, y, z, &texture, shape, axes, map, &visited);

                    for vy in y..ey {
                        for vz in z..ez {
                            for vx in x..ex {
                                visited[get_idx(vx, vy, vz)] = true;
                            }
                        }
                    }

                    let (qx1, qy1, qz1) = to_quake(x, z, y, map.width, map.length);
                    let (qx2, qy2, qz2) = to_quake(ex, ez, ey, map.width, map.length);

                    let brush = match shape {
                        BlockShape::SlabBottom => Brush {
                            min: (qx1, qy1, qz1),
                            max: (qx2, qy2, qz1 + HALF_BLOCK),
                            texture,
                            kind: BrushKind::Box,
                        },
                        BlockShape::SlabTop => Brush {
                            min: (qx1, qy1, qz1 + HALF_BLOCK),
                            max: (qx2, qy2, qz2),
                            texture,
                            kind: BrushKind::Box,
                        },
                        BlockShape::ThinPanelNS => Brush {
    			    min: (qx1 + THIN_PANEL_MIN, qy1, qz1),
    			    max: (qx1 + THIN_PANEL_MAX, qy2, qz2),
    			    texture,
    			    kind: BrushKind::Box,
			},
			BlockShape::ThinPanelEW => Brush {
    			    min: (qx1, qy1 + THIN_PANEL_MIN, qz1),
    			    max: (qx2, qy1 + THIN_PANEL_MAX, qz2),
    			    texture,
    			    kind: BrushKind::Box,
			},
                        BlockShape::Full => unreachable!(),
                        BlockShape::Path
                        | BlockShape::ThinCross
                        | BlockShape::PlantCross
                        | BlockShape::Torch
                        | BlockShape::Chest
                        | BlockShape::Bed
                        | BlockShape::StairNorthBottom
                        | BlockShape::StairSouthBottom
                        | BlockShape::StairEastBottom
                        | BlockShape::StairWestBottom
                        | BlockShape::StairNorthTop
                        | BlockShape::StairSouthTop
                        | BlockShape::StairEastTop
                        | BlockShape::StairWestTop => unreachable!(),
                    };
                    brushes.push(brush);
                } else {
                    visited[idx] = true;
                    brushes.extend(single_block_brush(
                        x,
                        y,
                        z,
                        shape,
                        texture,
                        map.width,
                        map.length,
                    ));
                }
            }
        }
    }

    println!("Optimized into {} brushes.", brushes.len());
    brushes
}