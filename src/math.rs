pub mod math {
    pub fn multiply_vector(vector: &Vec<f64>) -> f64 {
        let mut product = 1.0;
        for value in vector {
            product *= *value;
        }

        product
    }

    pub fn sum_vector(vector: &Vec<f64>) -> f64 {
        let mut sum = 0.0;
        for value in vector {
            sum += *value;
        }

        sum
    }

    pub fn equation_result(vector: &Vec<f64>) -> f64 {
        // 4x^3 + 2y + z (z + 3) = success_value
        4.0f64 * vector[0].powf(3.0f64) + 2.0f64 * vector[1] + vector[2] * (vector[2] + 3.0f64)
    }
}

#[cfg(test)]
mod tests {
    use crate::math::math::*;

    #[test]
    fn test_sum_vector() {
        let v1 = vec![1.0, 2.0, 3.0, 4.0];

        assert_eq!(10.0, sum_vector(&v1));
    }

    #[test]
    fn test_multiply_vector() {
        let v1 = vec![1.0, 2.0, 3.0, 4.0];

        assert_eq!(24.0, multiply_vector(&v1));
    }
}
