pub mod individual;
pub mod selection;
pub mod chromosome;
pub mod crossover;
pub mod mutation;

use crate::{
    crossover::CrossoverMethod,
    individual::Individual,
    mutation::MutationMethod,
    selection::SelectionMethod,
};

pub struct GeneticAlgorithm<S> {
    selection_method: S,
    crossover_method: Box<dyn CrossoverMethod>,
    mutation_method: Box<dyn MutationMethod>,
}

impl<S> GeneticAlgorithm<S>
where
    S: SelectionMethod
{
    pub fn new(
        selection_method: S,
        crossover_method: impl CrossoverMethod + 'static,
        mutation_method: impl MutationMethod + 'static,
    ) -> Self {
        Self {
            selection_method,
            crossover_method: Box::new(crossover_method),
            mutation_method: Box::new(mutation_method),
        }
    }

    pub fn evolve<I>(
        &self,
        rng: &mut dyn rand::RngCore,
        population: &[I]
    ) -> Vec<I>
    where
        I: Individual,
    {
        assert!(!population.is_empty());

        (0..population.len())
            .map(|_| {
                let parent_a = self
                    .selection_method
                    .select(rng, population)
                    .chromosome();

                let parent_b = self
                    .selection_method
                    .select(rng, population)
                    .chromosome();

                let mut child = self
                    .crossover_method
                    .crossover(rng, parent_a, parent_b);

                self.mutation_method.mutate(rng, &mut child);
                I::create(child)
            })
            .collect()
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use rand::SeedableRng;
    use rand_chacha::ChaCha8Rng;
    use crate::{
        individual::TestIndividual,
        selection::RouletteWheelSelection,
        mutation::GaussianMutation,
        crossover::UniformCrossover,
    };

    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }

    #[test]
    #[should_panic]
    fn evolve_with_empty_population_panics() {
        let mut rng = ChaCha8Rng::from_seed(Default::default());
        let population = Vec::<TestIndividual>::default();
        let algo = GeneticAlgorithm::new(
            RouletteWheelSelection,
            UniformCrossover,
            GaussianMutation::new(0.5, 0.5),
        );

        algo.evolve(&mut rng, &population);
    }


}
