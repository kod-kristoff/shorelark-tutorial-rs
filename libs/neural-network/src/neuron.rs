use rand::{Rng, RngCore};

#[derive(Clone, Debug)]
pub struct Neuron {
    pub bias: f32,
    pub weights: Vec<f32>,
}

impl Neuron {
    pub fn new(bias: f32, weights: Vec<f32>) -> Self {
        assert!(!weights.is_empty());

        Self { bias, weights }
    }

    pub fn random(
        output_size: usize,
        rng: &mut dyn RngCore,
    ) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0) )
            .collect();

        Self { bias, weights }
    }

    pub fn propagate(&self, inputs: &[f32]) -> f32 {
        let output = inputs
            .iter()
            .zip(&self.weights)
            .map(|(input, weight)| input * weight)
            .sum::<f32>();

        (output + self.bias).max(0.0)
    }
}

#[cfg(test)]
mod tests {
    use super::*;


    mod random {
        use super::*;
        use rand_chacha::ChaCha8Rng;
        use rand::SeedableRng;

        #[test]
        fn test() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(4, &mut rng);

            approx::assert_relative_eq!(neuron.bias, -0.6255188);
            approx::assert_relative_eq!(neuron.weights.as_slice(), [
                0.67383957,
                0.8181262,
                0.26284897,
                0.5238807,
                ].as_ref()
            );
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let neuron = Neuron::new(0.5, vec![-0.3, 0.8]);

            // Ensures `.max()` (our ReLU) works:
            approx::assert_relative_eq!(
                neuron.propagate(&[-10.0, -10.0]),
                0.0,
            );

            // `0.5` and `1.0` chosen by a fair dice roll:
            approx::assert_relative_eq!(
                neuron.propagate(&[0.5, 1.0]),
                (-0.3 * 0.5) + (0.8 * 1.0) + 0.5,
            );
        }
    }
}
