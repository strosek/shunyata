pub mod entity {
    use std::fmt;
    use std::string;
    use serde::{Serialize, Deserialize};

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Entity {
        id: u32,
        name: string::String,
        plasticity: f64,
        influence: f64,
        color: u32,
    }

    impl Entity {
        pub fn new(id: u32,
                   name: string::String,
                   plasticity: f64,
                   influence: f64,
                   color: u32,
                   ) -> Entity {
            Entity { id, name, plasticity, influence, color}
        }

        pub fn similarity(_e1: &Entity, _e2: &Entity) -> f64 {
            0.0
        }
    }

    impl fmt::Display for Entity {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(ID: {}, Name: {}, Color: {:X})", self.id, self.name, self.color)
        }
    }
}
