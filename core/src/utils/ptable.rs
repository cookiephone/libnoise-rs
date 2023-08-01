use rand::{rngs::StdRng, seq::SliceRandom, SeedableRng};

pub(crate) struct PermutationTable {
    pub(crate) table: Vec<usize>,
}

impl PermutationTable {
    pub(crate) fn new(seed: u64, w: usize, doubleup: bool) -> Self {
        let mut table = Vec::from_iter(0..w);
        let mut rng = StdRng::seed_from_u64(seed);
        table.shuffle(&mut rng);
        if doubleup {
            table.extend_from_within(..);
        }
        table.shrink_to_fit();
        Self { table }
    }

    unsafe fn get(&self, i: usize) -> usize {
        *self.table.get_unchecked(i)
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
