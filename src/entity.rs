pub mod entity {
    use std::string;
    use std::fmt;

    pub struct Entity {
        id: u32,
        name: string::String,
        color: u32,
    }

    impl Entity {
        pub fn create(id: u32, name: string::String, color: u32) -> Entity {
            Entity { id, name, color }
        }
    }

    impl fmt::Display for Entity {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "(ID: {}, Name: {}, Color: {:X})", self.id, self.name, self.color)
        }
    }
}
