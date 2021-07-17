pub mod math {
    pub fn equation_result(vector: &Vec<f64>) -> f64 {
        // 4x^3 + 2y + z (z + 3) = success_value
        4.0f64 * vector[0].powf(3.0f64) + 2.0f64 * vector[1] + vector[2] * (vector[2] + 3.0f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::math::*;
}
