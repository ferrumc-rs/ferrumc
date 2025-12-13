use std::range::{Range, RangeInclusive};

use bevy_math::{IVec2, IVec3, Vec2Swizzles, Vec3Swizzles};
use ferrumc_world::vanilla_chunk_format::BlockData;
use itertools::{Itertools, repeat_n};

use crate::{
    ChunkAccess, HeightmapType,
    biome::Biome,
    direction::Direction,
    perlin_noise::BIOME_INFO_NOISE,
    pos::{BlockPos, ChunkHeight},
    random::Rng,
};

//filters
struct BlockPredicateFilter {
    block: Vec<BlockData>,
}
impl BlockPredicateFilter {
    fn filter(&self, block: BlockData) -> bool {
        self.block.contains(&block)
    }
    fn apply(&self, pos: BlockPos, block: BlockData) -> impl Iterator<Item = BlockPos> {
        Some(pos).filter(|_| self.filter(block)).into_iter()
    }
}

struct RarityFilter {
    chance: f32,
}
impl RarityFilter {
    fn on_average_once_every(chance: u32) -> Self {
        Self {
            chance: 1.0 / chance as f32,
        }
    }

    fn filter(&self, random: &mut impl Rng) -> bool {
        random.next_f32() < self.chance
    }
    fn apply(&self, pos: BlockPos, random: &mut impl Rng) -> impl Iterator<Item = BlockPos> {
        Some(pos).filter(|_| self.filter(random)).into_iter()
    }
}
struct SurfaceRelativeThresholdFilter {
    range: RangeInclusive<i32>,
    surface_type: HeightmapType,
}
impl SurfaceRelativeThresholdFilter {
    fn filter(&self, pos: BlockPos, height: i32) -> bool {
        (height + self.range.start..=height + self.range.end).contains(&pos.y)
    }
    fn apply(&self, pos: BlockPos, height: i32) -> impl Iterator<Item = BlockPos> {
        Some(pos)
            .filter(|pos| self.filter(*pos, height))
            .into_iter()
    }
}
struct SurfaceWaterDepthFilter {
    max_water_depth: u32,
}

impl SurfaceWaterDepthFilter {
    fn filter(&self, ocean_floor: i32, height: i32) -> bool {
        ocean_floor - height <= self.max_water_depth as i32
    }
    fn apply(
        &self,
        pos: BlockPos,
        ocean_floor: i32,
        height: i32,
    ) -> impl Iterator<Item = BlockPos> {
        Some(pos)
            .filter(|_| self.filter(ocean_floor, height))
            .into_iter()
    }
}
struct BiomeFilter {
    biomes: Vec<Biome>, //TODO: const
}

impl BiomeFilter {
    fn filter(&self, biome: Biome) -> bool {
        self.biomes.contains(&biome)
    }
    fn apply(&self, pos: BlockPos, biome: Biome) -> impl Iterator<Item = BlockPos> {
        Some(pos).filter(|_| self.filter(biome)).into_iter()
    }
}

struct HeightmapPlacement {
    height_type: HeightmapType,
}

impl HeightmapPlacement {
    fn filter(&self, pos: BlockPos, chunk_height: ChunkHeight) -> bool {
        pos.y > chunk_height.min_y //TODO should never fail
    }
    fn apply(
        &self,
        pos: BlockPos,
        chunk_access: &ChunkAccess,
        chunk_height: ChunkHeight,
    ) -> impl Iterator<Item = BlockPos> {
        Some(pos.with_y(chunk_access.get_height(self.height_type, pos.x, pos.z)))
            .filter(|_| self.filter(pos, chunk_height))
            .into_iter()
    }
}
//count placements

struct CountPlacement {
    count: u32,
}

impl CountPlacement {
    fn count(&self, random: &mut impl Rng) -> usize {
        random.next_bounded(self.count) as usize //TODO
    }
    fn apply(&self, pos: BlockPos, random: &mut impl Rng) -> impl Iterator<Item = BlockPos> {
        repeat_n(pos, self.count(random))
    }
}

struct NoiseBasedCountPlacement {
    noise_to_count_ratio: i32,
    factor: f64,
    offset: f64,
}
impl NoiseBasedCountPlacement {
    fn count(&self, pos: BlockPos) -> usize {
        (BIOME_INFO_NOISE.legacy_simplex_at(pos.xz().as_dvec2() / self.factor) + self.offset)
            .ceil()
            .max(0.0) as usize
    }
    fn apply(&self, pos: BlockPos) -> impl Iterator<Item = BlockPos> {
        repeat_n(pos, self.count(pos))
    }
}

struct NoiseThresholdCountPlacement {
    noise_level: f64,
    below: usize,
    above: usize,
}
impl NoiseThresholdCountPlacement {
    fn count(&self, pos: BlockPos) -> usize {
        if BIOME_INFO_NOISE.legacy_simplex_at(pos.xz().as_dvec2() / 200.0) < self.noise_level {
            self.below
        } else {
            self.above
        }
    }
    fn apply(&self, pos: BlockPos) -> impl Iterator<Item = BlockPos> {
        repeat_n(pos, self.count(pos))
    }
}

struct CountOnEveryLayerPlacement {
    count: u32, //TODO
}

impl CountOnEveryLayerPlacement {
    fn apply(
        &self,
        pos: BlockPos,
        random: &mut impl Rng,
        height: &ChunkAccess,
        chunk_height: ChunkHeight,
    ) -> impl Iterator<Item = BlockPos> {
        let round = 0;
        let mut res = Vec::new();
        while res.is_empty() {
            for _ in 0..random.next_bounded(self.count) {
                let pos = pos.xz()
                    + IVec2::new(
                        random.next_bounded(16) as i32,
                        random.next_bounded(16) as i32,
                    );
                let y = height.get_height(HeightmapType::MotionBlocking, pos.x, pos.y);
                let pos = pos.xxy().with_y(y);
                if let Some((_, (i, _))) = (chunk_height.min_y..pos.y)
                    .rev()
                    .map(|y| height.get_block_state(pos.with_y(y)))
                    .tuple_windows()
                    .enumerate()
                    .filter(|(_, (a, b))| {
                        matches!(
                            a.name.as_str(),
                            "minecraft:water" | "minecraft:lava" | "minecraft:air"
                        ) && !matches!(
                            b.name.as_str(),
                            "minecraft:water"
                                | "minecraft:lava"
                                | "minecraft:air"
                                | "minecraft:bedrock"
                        )
                    })
                    .enumerate()
                    .find(|(i, _)| *i == round)
                {
                    res.push(pos.with_y(pos.y - i as i32))
                }
            }
        }
        res.into_iter()
    }
}

struct EnvironmentScanPlacement {
    direction: Direction,
    target_condition: Vec<BlockData>,
    allowed_search_condition: Vec<BlockData>,
    max_steps: usize,
}

impl EnvironmentScanPlacement {
    fn apply(
        &self,
        pos: BlockPos,
        chunk_access: &ChunkAccess,
        chunk_height: ChunkHeight,
    ) -> impl Iterator<Item = BlockPos> {
        if !self
            .allowed_search_condition
            .contains(&chunk_access.get_block_state(pos))
        {
            return None.into_iter();
        }
        (0..self.max_steps as i32)
            .map(|i| pos + self.direction.as_unit() * i)
            .take_while(|pos| chunk_height.iter().contains(&pos.y))
            .map(|pos| (pos, chunk_access.get_block_state(pos)))
            .take_while_inclusive(|(_, block)| self.allowed_search_condition.contains(block))
            .find(|(_, block)| self.target_condition.contains(block))
            .map(|(pos, _)| pos)
            .into_iter()
    }
}

struct HeightRangeModifier {
    range: Range<i32>,
}

impl HeightRangeModifier {
    fn apply(&self, pos: BlockPos, random: &mut impl Rng) -> impl Iterator<Item = BlockPos> {
        Some(pos.with_y(random.next_i32_range(self.range))).into_iter()
    }
}

struct InSquarePlacement;
impl InSquarePlacement {
    fn apply(&self, pos: BlockPos, random: &mut impl Rng) -> impl Iterator<Item = BlockPos> {
        Some(pos)
            .map(|pos| {
                pos + IVec3::new(
                    random.next_bounded(16) as i32,
                    0,
                    random.next_bounded(16) as i32,
                )
            })
            .into_iter()
    }
}
struct RandomOffsetPlacement {
    xz_offset: Range<i32>, //TODO
    y_offset: Range<i32>,
}
impl RandomOffsetPlacement {
    fn apply(&self, pos: BlockPos, random: &mut impl Rng) -> impl Iterator<Item = BlockPos> {
        Some(pos)
            .map(|pos| {
                pos + IVec3::new(
                    random.next_i32_range(self.xz_offset),
                    random.next_i32_range(self.y_offset),
                    random.next_i32_range(self.xz_offset),
                )
            })
            .into_iter()
    }
}
