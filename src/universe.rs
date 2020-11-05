pub mod universe {
    use crate::entity::entity::Entity;
    use rand::Rng;
    use serde::{Deserialize, Serialize};

    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::path::Path;
    use std::string;

    const DOMAIN_ATTRIBUTES_LOW: f64 = 0.0;
    const DOMAIN_ATTRIBUTES_HIGH: f64 = 10.0;


    #[derive(Serialize, Deserialize, Debug)]
    pub struct Universe {
        id: string::String,
        cycles: usize,
        n_beings: usize,
        n_being_attributes: usize,
        state: Vec<Entity>,
        next_state: Vec<Entity>,
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

    pub fn fitness(entity: &Entity) -> f64 {
        let mut fitness = solution_difference(120.0f64, &entity.attributes) * -1.0f64;

        // Make solution close to pure integers.
        let attributes_sum = sum_vector(&entity.attributes);
        fitness -= f64::abs(attributes_sum) - attributes_sum.floor();

        // Consider fitness very close to 0, as solution. Stop learning.
        if fitness > -0.01f64 {
            fitness = 1.0f64;
        }

        fitness
    }

    pub fn solution_difference(success_value: f64, attributes: &Vec<f64>) -> f64 {
        let mut difference = 0.0f64;
        let attributes_product = multiply_vector(attributes);
        if success_value > attributes_product {
            difference = success_value - attributes_product;
        }
        else if success_value < attributes_product {
            difference = attributes_product - success_value;
        }
        difference.abs()
    }

    impl Universe {
        pub fn new(id: string::String,
                   cycles: usize,
                   n_beings: usize,
                   n_being_attributes: usize) -> Universe {
            let state = Vec::<Entity>::new();
            let next_state = Vec::<Entity>::new();

            Universe {
                id,
                cycles,
                n_beings,
                n_being_attributes,
                state,
                next_state,
            }
        }

        pub fn spawn(&mut self, _config_file: &str) {
            /* - Initialize all entities according to config file.
             * - Populate and replicate until n entities randomly or in a specified proportion.
             * - nextState starts being a copy if current state.
             */
            let mut rng = rand::thread_rng();

            for i in 0..self.n_beings {
                let plasticity = rng.gen_range(0.0, 0.8);
                let influence = rng.gen_range(1.0, 3.0);
                let mut attributes = Vec::<f64>::with_capacity(self.n_being_attributes);
                for _ in 0..self.n_being_attributes {
                    attributes.push(rng.gen_range(
                            DOMAIN_ATTRIBUTES_LOW, DOMAIN_ATTRIBUTES_HIGH).round());
                }

                self.state.push(Entity::new(
                    i as u32,
                    "entity".to_string(),
                    plasticity,
                    influence,
                    attributes,
                ));

                self.next_state = self.state.to_vec();
            }

            for i in 0..self.cycles {
                self.tick();
                if i % 100_000_usize == 0 {
                    self.write_csv_line();
                    self.print_state();
                }
            }
        }

        fn print_state(&self) {
            for entity in &self.state {
                println!("{}", entity);
            }
            println!();
        }

        fn write_csv_line(&self) {
            let path = Path::new("shunyata.csv");

            let mut file;
            if path.exists() {
                file = OpenOptions::new()
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();
            }
            else {
                file = OpenOptions::new()
                    .create(true)
                    .write(true)
                    .append(true)
                    .open(path)
                    .unwrap();
            }


            let mut line = String::new();
            for entity in &self.state {
                line.push_str(&format!("{:.5},", multiply_vector(&entity.attributes)));
            }
            line.push_str("\n");

            file.write_all(line.as_bytes());
        }

        fn tick(&mut self) {
            let interactions = &self.get_random_interactions();
            for i in 0..interactions.len() {
                self.evaluate_interaction(&interactions[i]);
            }

            self.mutate_entities();
        }

        fn mutate_entities(&mut self) {
            let mut rng = rand::thread_rng();
            let n_mutations = rng.gen_range(0, self.n_beings / 10usize);
            for _ in 0..n_mutations {
                let entity_idx = rng.gen_range(0, self.n_beings);

                // Stop mutations if solution is reached.
                if fitness(&self.state[entity_idx]) < 0.0f64 {
                    let attribute_idx = rng.gen_range(0usize, self.n_being_attributes);
                    self.next_state[entity_idx].attributes[attribute_idx] +=
                        rng.gen_range(-0.001f64, 0.001f64);
                }
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
                let n_encounters = rng.gen_range(2, 5);

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



        fn evaluate_interaction(&mut self, encounter: &Vec<usize>) {
            // Calculate average of entities that met.
            let mut average = Vec::<f64>::with_capacity(self.n_being_attributes);
            let mut target = Vec::<f64>::with_capacity(self.n_being_attributes);

            // Build vector for averages.
            for _ in 0..self.n_being_attributes {
                average.push(0.0);
                target.push(0.0);
            }

            for i in 0..self.n_being_attributes {
                for j in encounter {
                    average[i] += self.state[*j].attributes[i];
                    if j == encounter.last().unwrap() {
                        average[i] /= encounter.len() as f64;
                    }
                }
            }

            // Calculate target value according to each entity's influence factor.
            for i in 0..self.n_being_attributes {
                for j in encounter {
                    if average[i] < self.state[*j].attributes[i] {
                        // TODO: consider using fitness to evaluate influence over encounter.
                        target[i] += (self.state[*j].attributes[i] - average[i]) * self.state[*j].influence;
                    }
                    else if average[i] > self.state[*j].attributes[i] {
                        target[i] -= (average[i] - self.state[*j].attributes[i]) * self.state[*j].influence;
                    }
                    // TODO: consider what to do if the value is the same.
                }
            }

            // Make entities similar to target.
            for i in 0..self.n_being_attributes {
                for j in encounter {
                    let target_entity = Entity::new(
                        u32::max_value(),
                        "target_entity".to_string(),
                        0.0,
                        0.0,
                        target.to_vec()
                    );

                    // Make entity more similar. Only learn if target is more successful.
                    let plasticity = self.state[*j].plasticity;
                    if fitness(&self.state[*j]) < fitness(&target_entity) {
                        if self.state[*j].attributes[i] < target[i] {
                            self.next_state[*j].attributes[i] += (target[i] - self.state[*j].attributes[i]) * plasticity;
                        } else if self.state[*j].attributes[i] > target[i] {
                            self.next_state[*j].attributes[i] -= (self.state[*j].attributes[i] - target[i]) * plasticity;
                        }
                    }
                }
            }

            self.state = self.next_state.to_vec();
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
