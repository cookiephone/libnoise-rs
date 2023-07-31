use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};
use std::sync::Once;

use crate::generators::constants::PERMUTATION_TABLE_SIZE;

pub(crate) struct StaticPermutationTable {
    pub(crate) table: Option<Vec<usize>>,
    pub(crate) seed: Option<u64>,
    pub(crate) sync: Once,
}

impl StaticPermutationTable {
    pub(crate) const fn const_default() -> Self {
        Self {
            table: None,
            seed: None,
            sync: Once::new(),
        }
    }

    unsafe fn get(&self, i: usize) -> usize {
        *self.table.as_ref().unwrap_unchecked().get_unchecked(i)
    }

    pub(crate) unsafe fn hash1d(&self, i: usize) -> usize {
        self.get(i)
    }

    pub(crate) unsafe fn hash2d(&self, i: usize, j: usize) -> usize {
        self.get(i + self.get(j))
    }

    pub(crate) unsafe fn hash3d(&self, i: usize, j: usize, k: usize) -> usize {
        self.get(i + self.get(j + self.get(k)))
    }

    pub(crate) unsafe fn hash4d(&self, i: usize, j: usize, k: usize, l: usize) -> usize {
        self.get(i + self.get(j + self.get(k + self.get(l))))
    }
}

pub(crate) fn build_permutation_table(seed: u64, w: usize, doubleup: bool) -> Vec<usize> {
    let mut permutation_table = Vec::from_iter(0..w);
    let mut rng = make_seeded_rng(seed);
    permutation_table.shuffle(&mut rng);
    if doubleup {
        permutation_table.extend_from_within(..);
    }
    permutation_table.shrink_to_fit();
    permutation_table
}

fn make_seeded_rng(seed: u64) -> impl Rng {
    StdRng::seed_from_u64(seed)
}

pub(crate) fn get_static_permutation_table(
    static_table: &'static mut StaticPermutationTable,
    seed: u64,
) -> &'static StaticPermutationTable {
    if static_table.seed.is_some_and(|old_seed| old_seed != seed) {
        static_table.sync = Once::new();
    }
    static_table.sync.call_once(|| {
        static_table.seed = Some(seed);
        static_table.table = Some(build_permutation_table(seed, PERMUTATION_TABLE_SIZE, true));
    });
    static_table
}
