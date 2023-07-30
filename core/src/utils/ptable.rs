use rand::{rngs::StdRng, seq::SliceRandom, Rng, SeedableRng};

trait PermutationTable {
    fn p(&self, idx: usize) -> usize;
}

impl PermutationTable for Vec<usize> {
    fn p(&self, idx: usize) -> usize {
        self[idx % self.len()]
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
