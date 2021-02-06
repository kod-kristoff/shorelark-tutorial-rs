
pub use self::layer_topology::*;

use self::{layer::*, neuron::*};
use rand::{Rng, RngCore};

mod neuron;
mod layer;
mod layer_topology;

#[derive(Clone, Debug)]
pub struct Network {
    layers: Vec<Layer>,
}

impl Network {
    pub fn new(layers: Vec<Layer>) -> Self {
        assert!(!layers.is_empty());
        Self { layers }
    } 
    pub fn random(
        layers: &[LayerTopology],
        mut rng: &mut dyn RngCore,
    ) -> Self {
        assert!(layers.len() > 1);
        
        let layers = layers
            .windows(2)
            .map(|layers| {
                Layer::random(
                    layers[0].neurons,
                    layers[1].neurons,
                    &mut rng
                )
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

            let network = Network::random(
                &[
                    LayerTopology { neurons: 3 },
                    LayerTopology { neurons: 2 },
                    LayerTopology { neurons: 1 },
                ], 
                &mut rng
            );

            assert_eq!(network.layers.len(), 2);
            assert_eq!(network.layers[0].neurons.len(), 2);
            assert_eq!(network.layers[1].neurons.len(), 1);
        }
    }

    mod propagate {
        use super::*;

        #[test]
        fn test() {
            let layers = (
                Layer::new(vec![
                    Neuron::new(0.0, vec![-0.5, -0.4, -0.3]),
                    Neuron::new(0.0, vec![-0.2, -0.1, 0.0]),
                ]),
                Layer::new(vec![Neuron::new(0.0, vec![-0.5, 0.5])]),
            );
            let network = Network::new(vec![layers.0.clone(), layers.1.clone()]);

            let actual = network.propagate(vec![0.5, 0.6, 0.7]);
            let expected = layers.1.propagate(layers.0.propagate(vec![0.5, 0.6, 0.7]));

            approx::assert_relative_eq!(actual.as_slice(), expected.as_slice());
        }
    }
}
