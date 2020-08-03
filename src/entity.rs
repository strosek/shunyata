pub mod entity {
    use serde::{Deserialize, Serialize};
    use std::fmt;
    use std::string;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Entity {
        pub id: u32,
        pub name: string::String,
        pub plasticity: f64,
        pub influence: f64,
        pub value: f64,
    }

    impl Entity {
        pub fn new(
            id: u32,
            name: string::String,
            plasticity: f64,
            influence: f64,
            value: f64,
        ) -> Entity {
            Entity {
                id,
                name,
                plasticity,
                influence,
                value,
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
                "(ID: {}, Name: {}, Color: {})",
                self.id, self.name, self.value
            )
        }
    }
}
