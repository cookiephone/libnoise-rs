use super::math::{Vec2, Vec3, Vec4};
use rand::seq::SliceRandom;
use rand_chacha::{rand_core::SeedableRng, ChaCha12Rng};

#[derive(Clone, Debug)]
pub(crate) struct PermutationTable {
    pub(crate) table: Vec<usize>,
}

impl PermutationTable {
    pub(crate) fn new(seed: u64, w: usize, doubleup: bool) -> Self {
        let mut table = Vec::from_iter(0..w);
        let mut rng = ChaCha12Rng::seed_from_u64(seed);
        table.shuffle(&mut rng);
        if doubleup {
            table.extend_from_within(..);
        }
        table.shrink_to_fit();
        Self { table }
    }

    #[inline]
    pub(crate) unsafe fn get(&self, i: usize) -> usize {
        *self.table.get_unchecked(i)
    }

    #[inline]
    pub(crate) unsafe fn hash1d(&self, i: usize) -> usize {
        self.get(i)
    }

    #[inline]
    pub(crate) unsafe fn hash2d(&self, i: usize, j: usize) -> usize {
        self.get(j + self.get(i))
    }

    #[inline]
    pub(crate) unsafe fn hash3d(&self, i: usize, j: usize, k: usize) -> usize {
        self.get(k + self.get(j + self.get(i)))
    }

    #[inline]
    pub(crate) unsafe fn hash4d(&self, i: usize, j: usize, k: usize, l: usize) -> usize {
        self.get(l + self.get(k + self.get(j + self.get(i))))
    }

    #[inline]
    pub(crate) unsafe fn hash2d_vec(&self, value: Vec2<usize>) -> usize {
        self.get(value.y + self.get(value.x))
    }

    #[inline]
    pub(crate) unsafe fn hash3d_vec(&self, value: Vec3<usize>) -> usize {
        self.get(value.z + self.get(value.y + self.get(value.x)))
    }

    #[inline]
    pub(crate) unsafe fn hash4d_vec(&self, value: Vec4<usize>) -> usize {
        self.get(value.w + self.get(value.z + self.get(value.y + self.get(value.x))))
    }
}
