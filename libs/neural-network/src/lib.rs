use rand::Rng;


pub struct Network {
    layers: Vec<Layer>,
}

pub struct LayerTopology {
    pub neurons: usize,
}

struct Layer {
    neurons: Vec<Neuron>,
}

struct Neuron {
    bias: f32,
    weights: Vec<f32>,
}

impl Network {
    pub fn random(layers: &[LayerTopology]) -> Self {
        assert!(layers.len() > 1);
        
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(layers[0].neurons, layers[1].neurons)
            })
            .collect();

        Self { layers }
    }

    pub fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.layers
            .iter()
            .fold(inputs, |inputs, layer| layer.propagate(inputs))
    }
}

impl Layer {
    fn random(input_neurons: usize, output_neurons: usize) -> Self {
        let mut rng = rand::thread_rng();
        let neurons = (0..output_neurons)
            .map(|_| Neuron::random(&mut rng, input_neurons))
            .collect();

        Self { neurons }
    }

    fn propagate(&self, inputs: Vec<f32>) -> Vec<f32> {
        self.neurons
            .iter()
            .map(|neuron| neuron.propagate(&inputs))
            .collect()
    }
}

impl Neuron {
    fn random(
        rng: &mut dyn rand::RngCore,
        output_size: usize
    ) -> Self {
        let bias = rng.gen_range(-1.0..=1.0);

        let weights = (0..output_size)
            .map(|_| rng.gen_range(-1.0..=1.0) )
            .collect();

        Self { bias, weights }
    }

    fn propagate(&self, inputs: &[f32]) -> f32 {
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

    use rand_chacha::ChaCha8Rng;
    use rand::SeedableRng;
    use approx::assert_relative_eq;

    mod random {
        use super::*;

        #[test]
        fn test_neuron() {
            let mut rng = ChaCha8Rng::from_seed(Default::default());
            let neuron = Neuron::random(&mut rng, 4);

            assert_relative_eq!(neuron.bias, -0.6255188);
            assert_relative_eq!(neuron.weights.as_slice(), [
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
        fn test_neuron() {
            let neuron = Neuron {
                bias: 0.5,
                weights: vec![-0.3, 0.8],
            };

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
