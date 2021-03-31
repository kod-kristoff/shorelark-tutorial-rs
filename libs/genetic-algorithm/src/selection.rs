mod roulette_wheel;

pub use self::roulette_wheel::*;

use crate::{
    individual::Individual,
};

pub trait SelectionMethod {
    fn select<'a, I>(
        &self,
        rng: &mut dyn rand::RngCore,
        population: &'a [I],
    ) -> &'a I
    where
        I: Individual;
}
