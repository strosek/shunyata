pub mod universe {
    use crate::entity::entity::Entity;
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use std::string;
    use std::vec;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Universe {
        id: string::String,
        n_beings: usize,
        state: vec::Vec<Entity>,
    }

    /* Methods:
     *   - Spawn()
     *   - Anihilate() // Destructor
     *   - Tick()
     *   - Save()
     *   - Interact()
     *   - UpdateRelationships()
     */
    impl Universe {
        pub fn new(id: string::String, n_beings: usize) -> Universe {
            let state = vec::Vec::<Entity>::new();

            Universe {
                id,
                n_beings,
                state,
            }
        }

        pub fn spawn(&mut self, _config_file: &str) {
            /* - Initialize all entities according to config file.
             * - Populate and replicate until n entities randomly or in a specified proportion.
             * - nextState starts being a copy if current state.
             */
            let mut rng = rand::thread_rng();

            for i in 0..self.n_beings {
                let plasticity = rng.gen_range(0.1, 0.9);
                let influence = rng.gen_range(0.1, 0.9);
                let value = i as f64;
                //let.value = rng.gen_range(0x000000, 0xFF);

                self.state.push(Entity::new(
                    i as u32,
                    "entity".to_string(),
                    plasticity,
                    influence,
                    value as f64,
                ));
            }

            // TODO: change this for a loop statement.
            for _ in 0..100 {
                self.tick();
            }
        }

        fn tick(&mut self) {
            let interactions = &self.get_random_interactions();
            for i in 0..interactions.len() {
                self.evaluate_interaction(&interactions[i]);
            }
        }

        fn get_random_interactions(&self) -> Vec<Vec<usize>> {
            /// Get who interacts with who, randomly.

            let mut rng = rand::thread_rng();
            let mut interacted = Vec::<bool>::with_capacity(self.n_beings);
            for _ in 0..self.n_beings {
                interacted.push(false);
            }

            let n_groups = rng.gen_range(2, self.n_beings / 3);
            let mut groups = Vec::<Vec<usize>>::new();

            for _ in 0..n_groups {
                let n_encounters = rng.gen_range(2, self.n_beings / 5);

                let mut encounter = vec::Vec::<usize>::new();

                let mut i_encounters = 0usize;
                while i_encounters < n_encounters {
                    let entity_index = rng.gen_range(0, self.n_beings);
                    if ! interacted[entity_index] {
                        encounter.push(entity_index);
                        i_encounters += 1;
                        interacted[entity_index] = true;
                    }
                }
                groups.push(encounter);
            }

            groups
        }

        fn evaluate_interaction(&mut self, encounter: &vec::Vec<usize>) {
            // Calculate average of entities that met.
            let mut sum = 0.0f64;
            let mut i = 0usize;
            for meet in encounter {
                let entity = &self.state[*meet];
                sum += entity.value;
                print!(" Color: {:.2}", entity.value);
                i += 1usize;
            }
            let average = sum as f64 / i as f64;

            // Calculate target value according to each entity's influence factor.
            let mut target = average;
            for meet in encounter {
                let entity = &self.state[*meet];

                // Make target more similar to entities with their influence values.
                if entity.value < average {
                    target -= (average - entity.value) * entity.influence;
                }
                else if entity.value > average {
                    target += (entity.value - average) * entity.influence;
                }
            }

            // Make entities similar to target.
            for meet in encounter {
                let entity = &self.state[*meet];
                // Make target more similar to entities with their influence values.
                if entity.value < target {
                    self.state[*meet].value += (target - entity.value) / 2.0f64;
                }
                else if entity.value > target {
                    self.state[*meet].value -= (entity.value- target) / 2.0f64;
                }
            }
            print!("\n");
            println!("  Average: {:.2}", average);
            println!("  Target: {:.2}", target);
        }
    }
}
