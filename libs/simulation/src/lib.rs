use nalgebra;

pub struct Simulation {
    world: World,
}

impl Simulation {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        Self {
            world: World::random(rng),
        }
    }

    pub fn world(&self) -> &World {
        &self.world
    }
}

#[derive(Debug)]
pub struct World {
    animals: Vec<Animal>,
    food: Vec<Food>,
}

impl World {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        let animals = (0..40)
            .map(|_| Animal::random(rng))
            .collect();
        let food = (0..60)
            .map(|_| Food::random(rng))
            .collect();

        Self { animals, food }
    }

    pub fn animals(&self) -> &[Animal] {
        &self.animals
    }

    pub fn food(&self) -> &[Food] {
        &self.food
    }
}

#[derive(Debug)]
pub struct Animal {
    position: nalgebra::Point2<f32>,
    rotation: nalgebra::Rotation2<f32>,
    speed: f32,
}

impl Animal {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        use rand::Rng;
        Self {
            position: rng.gen(),
            rotation: rng.gen(),
            speed: 0.02,
        }
    }

    pub fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }

    pub fn rotation(&self) -> nalgebra::Rotation2<f32> {
        self.rotation
    }
}

#[derive(Debug)]
pub struct Food {
    position: nalgebra::Point2<f32>,
}

impl Food {
    pub fn random(rng: &mut dyn rand::RngCore) -> Self {
        use rand::Rng;
        Self {
            position: rng.gen(),
        }
    }

    pub fn position(&self) -> nalgebra::Point2<f32> {
        self.position
    }
}


#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
