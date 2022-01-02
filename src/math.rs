pub mod math {
    pub struct Position {
        pub x: f64,
        pub y: f64,
    }

    impl Clone for Position {
        fn clone(&self) -> Self {
            Position {
                x: self.x,
                y: self.y,
            }
        }
    }

    pub fn distance(a: &Position, b: &Position) -> f64 {
        ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
    }
}

#[cfg(test)]
mod tests {
    use crate::math::math::*;
}
