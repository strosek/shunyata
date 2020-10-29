pub mod entity {
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::string;
    pub use crate::universe::universe::{sum_vector, multiply_vector, fitness};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Entity {
        pub id: u32,
        pub name: string::String,
        pub plasticity: f64,
        pub influence: f64,
        pub attributes: Vec::<f64>,
    }

    impl Entity {
        pub fn new(
            id: u32,
            name: string::String,
            plasticity: f64,
            influence: f64,
            attributes: Vec::<f64>,
        ) -> Entity {
            Entity {
                id,
                name,
                plasticity,
                influence,
                attributes,
            }
        }

        pub fn similarity(_e1: &Entity, _e2: &Entity) -> f64 {
            0.0
        }
    }

    impl fmt::Display for Entity {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(
                f,
                "{:3} - Sum: {:10.2}: Product: {:10.2} Fitness: {:10.4} Values: {:10.4?}",
                self.id,
                sum_vector(&self.attributes),
                multiply_vector(&self.attributes),
                fitness(&self),
                self.attributes
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
                self.attributes.to_vec()
            )
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::Entity;

    #[test]
    fn test_new() {
        let entity = Entity::new(
            12u32,
            "Hello".to_string(),
            1.2,
            2.3,
            Vec::<f64>::new()
        );

        assert_ne!(entity.to_string(), "".to_string());
    }
}