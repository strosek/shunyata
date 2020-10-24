pub mod universe {
    use crate::entity::entity::Entity;
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use std::string;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Universe {
        id: string::String,
        cycles: usize,
        n_beings: usize,
        n_being_attributes: usize,
        state: Vec<Entity>,
        success_value: f64,
    }

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

    impl Universe {
        pub fn new(id: string::String,
                   cycles: usize,
                   n_beings: usize,
                   n_being_attributes: usize,
                   success_value: f64) -> Universe {
            let state = Vec::<Entity>::new();

            Universe {
                id,
                cycles,
                n_beings,
                n_being_attributes,
                state,
                success_value,
            }
        }

        pub fn spawn(&mut self, _config_file: &str) {
            /* - Initialize all entities according to config file.
             * - Populate and replicate until n entities randomly or in a specified proportion.
             * - nextState starts being a copy if current state.
             */
            let mut rng = rand::thread_rng();

            for i in 0..self.n_beings {
                let plasticity = rng.gen_range(0.0, 0.2);
                let influence = rng.gen_range(0.1, 0.5);
                let mut attributes = Vec::<f64>::with_capacity(self.n_being_attributes);
                for _ in 0..self.n_being_attributes {
                    attributes.push(rng.gen_range(-100.0, 100.0));
                }

                self.state.push(Entity::new(
                    i as u32,
                    "entity".to_string(),
                    plasticity,
                    influence,
                    attributes,
                ));
            }

            self.write_csv_line();
            for i in 0..self.cycles {
                self.tick();
                if i % 2 == 0 {
                    self.write_csv_line();
                }
            }
        }

        fn write_csv_line(&self) {
            for entity in &self.state {
                print!("{} - {:?},", sum_vector(&entity.attributes), entity.attributes);
            }
            println!();
        }

        fn tick(&mut self) {
            let interactions = &self.get_random_interactions();
            for i in 0..interactions.len() {
                self.evaluate_interaction(&interactions[i]);
            }
        }

        /// Get who interacts with who, randomly.
        fn get_random_interactions(&self) -> Vec<Vec<usize>> {

            let mut rng = rand::thread_rng();
            let mut interacted = Vec::<bool>::with_capacity(self.n_beings);
            for _ in 0..self.n_beings {
                interacted.push(false);
            }

            let n_groups = rng.gen_range(2, 3);
            let mut groups = Vec::<Vec<usize>>::new();

            for _ in 0..n_groups {
                let n_encounters = rng.gen_range(2, 3);

                let mut encounter = Vec::<usize>::new();

                let mut i_encounters = 0usize;
                while i_encounters < n_encounters {
                    let entity_index = rng.gen_range(0, self.n_beings);
                    if interacted[entity_index] == false {
                        encounter.push(entity_index);

                        interacted[entity_index] = true;
                        i_encounters += 1;
                    }
                }
                groups.push(encounter);
            }
            interacted.clear();

            groups
        }


        fn solution_difference(&self, attributes: &Vec<f64>) -> f64 {
            let mut difference = 0.0f64;
            let attributes_product = multiply_vector(attributes);
            if self.success_value > attributes_product {
                difference = self.success_value - attributes_product;
            }
            else if self.success_value < attributes_product {
                difference = attributes_product - self.success_value;
            }
            difference.abs()
        }

        fn evaluate_interaction(&mut self, encounter: &Vec<usize>) {
            // Calculate average of entities that met.
            let mut averages = Vec::<f64>::with_capacity(self.n_being_attributes);

            // Build vector for averages.
            for _ in 0..self.n_being_attributes {
                averages.push(0.0);
            }

            for i in 0..self.n_being_attributes {
                for j in encounter {
                    averages[i] += self.state[*j].attributes[i];
                    if j == encounter.last().unwrap() {
                        averages[i] /= encounter.len() as f64;
                    }
                }
            }

            // Calculate target value according to each entity's influence factor.
            let mut targets = Vec::<f64>::with_capacity(self.n_being_attributes);
            for i in encounter {
                for j in 0..self.n_being_attributes {
                    let with_influence = 0.0;
                    if self.solution_difference(&self.state[*i].attributes) < self.solution_difference(&averages) {

                    }
                    if self.state[*i].attributes[j] > targets[j] {

                    }
                    else if self.state[*i].attributes[j] < targets[j] {

                    }
                    targets.push(with_influence);
                }
            }

            // Make target more similar to entities with their influence values and fitness.

            // Make entities similar to target.
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::universe::universe::*;

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
