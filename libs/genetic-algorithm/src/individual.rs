use crate::{chromosome::Chromosome};

pub trait Individual {
    fn chromosome(&self) -> &Chromosome;
    fn fitness(&self) -> f32;
}

#[cfg(test)]
#[derive(Clone, Debug)]
pub struct TestIndividual {
    fitness: f32,
}

#[cfg(test)]
impl TestIndividual {
    pub fn new(fitness: f32) -> Self {
        Self { fitness }
    }
}

#[cfg(test)]
impl Individual for TestIndividual {
    fn fitness(&self) -> f32 {
        self.fitness
    }
    fn chromosome(&self) -> &Chromosome {
        panic!("not supported for TestIndividual")
    }
}
