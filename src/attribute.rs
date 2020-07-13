pub mod attribute {
    // An idea on how to implement behavior for different data types, which are mapped within an
    // Entity struct.
    trait Influence {
        fn influence();
    }

    trait Comparable {
        fn compare();
    }

    pub struct Attribute<T> {
        value: T,
    }

    impl Attribute<T> {
        pub fn new(&self, value: T) -> Attribute<T> {
            self.value = value;
        }
    }
}