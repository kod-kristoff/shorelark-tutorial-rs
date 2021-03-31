mod uniform;

pub use self::uniform::*;

use crate::{
    chromosome::Chromosome,
};

pub trait CrossoverMethod {
    fn crossover(
        &self,
        rng: &mut dyn rand::RngCore,
        parent_a: &Chromosome,
        parent_b: &Chromosome,
    ) -> Chromosome;
}
