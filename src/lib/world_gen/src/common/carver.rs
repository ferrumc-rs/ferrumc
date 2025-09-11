use std::iter::empty;

use bevy_math::{DVec2, DVec3, IVec3, Vec2Swizzles, Vec3Swizzles};
use itertools::{Either, Itertools};

use crate::pos::{ChunkHeight, ChunkPos};

pub(crate) struct CarvingMask {
    carved: Vec<bool>,
    min_y: i32,
}
impl CarvingMask {
    pub(crate) fn new(chunk_height: ChunkHeight) -> Self {
        Self {
            min_y: chunk_height.min_y,
            carved: vec![false; (chunk_height.height * 16 * 16) as usize],
        }
    }
    pub(crate) fn carve(&mut self, pos: IVec3) -> bool {
        let i = pos.x & 15 | (pos.z & 15) << 4 | (pos.y - self.min_y) << 8;
        let res = self.carved[i as usize];
        self.carved[i as usize] = true;
        res
    }
}

pub(crate) fn carve_ellipsoid(
    chunk_pos: ChunkPos,
    pos: DVec3,
    radii: DVec2,
    chunk_height: ChunkHeight,
) -> impl Iterator<Item = (DVec3, IVec3)> {
    if (chunk_pos.column_pos(8, 8).pos.as_dvec2() - pos.xz())
        .abs()
        .max_element()
        > 16.0 + radii.x * 2.0
    {
        return Either::Left(empty());
    }

    let radii = radii.xyx();
    let min = ((pos - radii).floor().as_ivec3() - IVec3::splat(1))
        .max((0, chunk_height.min_y + 1, 0).into());
    let max = (pos + radii)
        .floor()
        .as_ivec3()
        .min((15, chunk_height.max_y() - 1 - 7, 15).into());
    let x = move |x| ((f64::from(x) + 0.5 - pos.x) / radii.x, x);
    let z = (min.z..=max.z).map(move |z| ((f64::from(z) + 0.5 - pos.z) / radii.z, z));
    let y = (min.y..=max.y)
        .rev()
        .map(move |y| ((f64::from(y) - 0.5 - pos.y) / radii.y, y));
    Either::Right(
        (min.x..=max.x)
            .map(x)
            .cartesian_product(z)
            .filter(|((dx, _), (dz, _))| dx * dx + dz * dz < 1.0)
            .cartesian_product(y)
            .map(|(((dx, x), (dz, z)), (dy, y))| (DVec3::new(dx, dy, dz), IVec3::new(x, y, z))),
    )
}
