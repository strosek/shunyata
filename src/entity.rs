pub mod entity {
    pub use crate::math::math::{multiply_vector, sum_vector};
    pub use crate::universe::universe::fitness;
    use std::fmt;
    use std::string;

    pub struct Entity {
        pub id: u32,
        pub name: string::String,
        pub plasticity: f64,
        pub influence: f64,
        pub attributes: Vec<f64>,
        pub n_interactions: u64,
    }

    impl Entity {
        pub fn new(
            id: u32,
            name: string::String,
            plasticity: f64,
            influence: f64,
            attributes: Vec<f64>,
            n_interactions: u64,
        ) -> Entity {
            Entity {
                id,
                name,
                plasticity,
                influence,
                attributes,
                n_interactions,
            }
        }

        #[allow(dead_code)]
        pub fn similarity(e1: &Entity, e2: &Entity) -> f64 {
            let similarity = sum_vector(&e1.attributes) - sum_vector(&e2.attributes);
            similarity.abs()
        }
    }

    impl fmt::Display for Entity {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:3} - Sum: {:6.2}: Product: {:10.4} Fitness: {:10.4} Values: {:10.6?}    Interactions: {}",
                self.id,
                sum_vector(&self.attributes),
                multiply_vector(&self.attributes),
                fitness(&self, 666.0f64),
                self.attributes,
                self.n_interactions
            )
        }
    }

    impl std::clone::Clone for Entity {
        fn clone(&self) -> Self {
            Entity::new(
                self.id,
                self.name.to_string(),
                self.plasticity,
                self.influence,
                self.attributes.to_vec(),
                self.n_interactions,
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::entity::entity::Entity;

    #[test]
    fn test_new() {
        let entity = Entity::new(12u32, "Hello".to_string(), 0.5, 2.3, Vec::<f64>::new(), 0);

        assert_ne!(entity.to_string(), "".to_string());
    }
}
