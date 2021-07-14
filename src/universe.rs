pub mod universe {
    use crate::entity::entity::Entity;
    use crate::math::math::{equation_result, multiply_vector, sum_vector};

    use rand::Rng;
    use std::fs;
    use std::fs::OpenOptions;
    use std::io::prelude::*;
    use std::ops::Index;
    use std::path::Path;

    pub fn solution_difference(success_value: f64, attributes: &Vec<f64>) -> f64 {
        (success_value - equation_result(attributes)).abs()
    }

    pub fn fitness(entity: &Entity, success_value: f64, success_margin: f64) -> f64 {
        let mut fitness = solution_difference(success_value, &entity.attributes) * -1.0f64;

        // Consider a difference very close to 0 as a sufficiently good solution.
        if fitness >= 0.0f64 - success_margin {
            fitness += sum_vector(&entity.attributes);
        }

        fitness
    }

    pub struct Universe {
        // Core data
        id: String,
        state: Vec<Entity>,
        next_state: Vec<Entity>,

        // Program parameters
        csv_out: String,

        // Simulation parameters
        attribute_domain_high: f64,
        attribute_domain_low: f64,
        encounters_per_tick_max: usize,
        encounters_per_tick_min: usize,
        entities_per_encounter_max: usize,
        entities_per_encounter_min: usize,
        favor_integers: bool,
        n_being_attributes: usize,
        n_beings: usize,
        state_dump_frequency: usize,
        success_margin: f64,
        success_value: f64,
        total_cycles: usize,
    }

    impl Universe {
        pub fn new() -> Universe {
            let id = "somewhere".to_string();
            let state = Vec::<Entity>::new();
            let next_state = Vec::<Entity>::new();
            let csv_out = "shunyata_output.csv".to_string();
            let attribute_domain_high = 0.0f64;
            let attribute_domain_low = 100.0f64;
            let encounters_per_tick_max = 3usize;
            let encounters_per_tick_min = 1usize;
            let entities_per_encounter_max = 5usize;
            let entities_per_encounter_min = 2usize;
            let favor_integers = false;
            let n_being_attributes = 3usize;
            let n_beings = 50usize;
            let state_dump_frequency = 10_000usize;
            let success_margin = 0.001f64;
            let success_value = 666.0f64;
            let total_cycles = 1_000_000usize;

            Universe {
                id,
                state,
                next_state,
                csv_out,
                attribute_domain_high,
                attribute_domain_low,
                encounters_per_tick_max,
                encounters_per_tick_min,
                entities_per_encounter_max,
                entities_per_encounter_min,
                favor_integers,
                n_being_attributes,
                n_beings,
                state_dump_frequency,
                success_margin,
                success_value,
                total_cycles,
            }
        }

        pub fn from_config(config_path: &str) -> Universe {
            let json_string = fs::read_to_string(config_path).unwrap();
            let parsed_json = json::parse(json_string.as_str()).unwrap();
            let universe_values = parsed_json.index("universe");

            let id = universe_values.index("id").to_string();
            let state = Vec::<Entity>::new();
            let next_state = Vec::<Entity>::new();
            let csv_out = universe_values.index("csv_output_filename").to_string();
            let attribute_domain_high = universe_values["attribute_domain_high"].as_f64().unwrap();
            let attribute_domain_low = universe_values["attribute_domain_low"].as_f64().unwrap();
            let encounters_per_tick_max = universe_values["encounters_per_tick_max"]
                .as_usize()
                .unwrap();
            let encounters_per_tick_min = universe_values["encounters_per_tick_min"]
                .as_usize()
                .unwrap();
            let entities_per_encounter_max = universe_values["entities_per_encounter_max"]
                .as_usize()
                .unwrap();
            let entities_per_encounter_min = universe_values["entities_per_encounter_min"]
                .as_usize()
                .unwrap();
            let favor_integers = universe_values["favor_integers"].as_bool().unwrap();
            let n_being_attributes = universe_values["n_being_attributes"].as_usize().unwrap();
            let n_beings = universe_values["n_beings"].as_usize().unwrap();
            let state_dump_frequency = universe_values["state_dump_frequency"].as_usize().unwrap();
            let success_margin = universe_values["success_margin"].as_f64().unwrap();
            let success_value = universe_values["success_value"].as_f64().unwrap();
            let total_cycles = universe_values["total_cycles"].as_usize().unwrap();

            Universe {
                id,
                state,
                next_state,
                csv_out,
                attribute_domain_high,
                attribute_domain_low,
                encounters_per_tick_max,
                encounters_per_tick_min,
                entities_per_encounter_max,
                entities_per_encounter_min,
                favor_integers,
                n_being_attributes,
                n_beings,
                state_dump_frequency,
                success_margin,
                success_value,
                total_cycles,
            }
        }

        pub fn spawn(&mut self) {
            /* - Initialize all entities according to config file.
             * - Populate and replicate until n entities randomly or in a specified proportion.
             * - nextState starts being a copy if current state.
             */
            let mut rng = rand::thread_rng();

            for i in 0..self.n_beings {
                // TODO: add max and min plasticity and influence as Universe parameters.
                let plasticity = rng.gen_range(0.0, 0.8);
                let influence = rng.gen_range(1.0, 3.0);

                let mut attributes = Vec::<f64>::with_capacity(self.n_being_attributes);
                for _ in 0..self.n_being_attributes {
                    attributes.push(
                        rng.gen_range(self.attribute_domain_low, self.attribute_domain_high)
                            .round(),
                    );
                }

                self.state.push(Entity::new(
                    i as u32,
                    "entity".to_string(),
                    plasticity,
                    influence,
                    attributes,
                    0u64,
                ));

                // Set state and next_state with the same values.
                self.next_state = self.state.to_vec();
            }

            self.create_csv();

            println!("Universe: {}", self.id);
            for i in 0..self.total_cycles {
                self.tick();

                if i % self.state_dump_frequency == 0 {
                    println!("Tick no: {}", i);
                    self.write_csv_line();
                    self.print_state();
                }
            }
            println!("Tick no: {}", self.total_cycles);
            self.write_csv_line();
            self.print_state();
        }

        fn print_state(&self) {
            let mut successes = 0usize;
            for entity in &self.state {
                print!("{}", entity);
                if fitness(&entity, self.success_value, self.success_margin) >= 0.0f64 {
                    println!(" - Succeeded!");
                    successes += 1usize;
                } else {
                    println!();
                }
            }
            println!("Successes: {}\n", successes);
        }

        fn create_csv(&self) {
            let path = Path::new(self.csv_out.as_str());

            OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(true)
                .open(path)
                .unwrap();
        }

        fn write_csv_line(&self) {
            let path = Path::new(self.csv_out.as_str());

            let mut file;
            file = OpenOptions::new()
                .write(true)
                .append(true)
                .open(path)
                .unwrap();

            let mut line = String::new();
            for entity in &self.state {
                line.push_str(&format!("{:.5},", multiply_vector(&entity.attributes)));
            }
            line.push_str("\n");

            file.write_all(line.as_bytes()).unwrap();
        }

        fn tick(&mut self) {
            let interactions = &self.gen_random_interactions();
            for i in 0..interactions.len() {
                self.evaluate_interaction(&interactions[i]);
            }

            self.mutate_entities();

            // TODO: assign the next state with Rayon.
            self.state = self.next_state.to_vec();
        }

        fn mutate_entities(&mut self) {
            let mut rng = rand::thread_rng();
            let n_mutations = rng.gen_range(0, self.n_beings / 10usize);
            for _ in 0..n_mutations {
                let entity_idx = rng.gen_range(0, self.n_beings);

                // Stop mutations if solution is reached.
                if fitness(
                    &self.state[entity_idx],
                    self.success_value,
                    self.success_margin,
                ) < 0.0f64
                {
                    let attribute_idx = rng.gen_range(0usize, self.n_being_attributes);
                    self.next_state[entity_idx].attributes[attribute_idx] +=
                        rng.gen_range(-0.001f64, 0.001f64);
                }
            }
        }

        /// Get who interacts with who, randomly.
        fn gen_random_interactions(&mut self) -> Vec<Vec<usize>> {
            let mut rng = rand::thread_rng();
            let mut has_interacted = vec![false; self.n_beings];

            let n_groups =
                rng.gen_range(self.encounters_per_tick_min, self.encounters_per_tick_max);
            let mut groups = Vec::<Vec<usize>>::new();

            for _ in 0..n_groups {
                let n_entities_group = rng.gen_range(
                    self.entities_per_encounter_min,
                    self.entities_per_encounter_max,
                );

                let mut encounter = Vec::<usize>::new();

                let mut i_entity = 0usize;
                while i_entity < n_entities_group {
                    let entity_index = rng.gen_range(0, self.n_beings);
                    if has_interacted[entity_index] == false {
                        encounter.push(entity_index);

                        has_interacted[entity_index] = true;
                        self.next_state[entity_index].n_interactions += 1u64;

                        i_entity += 1;
                    }
                }
                groups.push(encounter);
            }

            groups
        }

        fn evaluate_interaction(&mut self, encounter: &Vec<usize>) {
            let mut average = Vec::<f64>::with_capacity(self.n_being_attributes);
            let mut target = Vec::<f64>::with_capacity(self.n_being_attributes);

            // Build vector for averages.
            for _ in 0..self.n_being_attributes {
                average.push(0.0);
                target.push(0.0);
            }

            // Calculate average of entities that met.
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
                        target[i] +=
                            (self.state[*j].attributes[i] - average[i]) * self.state[*j].influence;
                    } else if average[i] > self.state[*j].attributes[i] {
                        target[i] -=
                            (average[i] - self.state[*j].attributes[i]) * self.state[*j].influence;
                    }
                }
            }

            // Make entities similar to target.
            for i in 0..self.n_being_attributes {
                for j in encounter {
                    let target_entity = Entity::new(
                        u32::MAX,
                        "target_entity".to_string(),
                        0.0,
                        0.0,
                        target.to_vec(),
                        0u64,
                    );

                    // Make entity more similar. Only learn if target is more successful.
                    let plasticity = self.state[*j].plasticity;
                    if fitness(&self.state[*j], self.success_value, self.success_margin)
                        < fitness(&target_entity, self.success_value, self.success_margin)
                    {
                        if self.state[*j].attributes[i] < target[i] {
                            self.next_state[*j].attributes[i] +=
                                (target[i] - self.state[*j].attributes[i]) * plasticity;
                        } else if self.state[*j].attributes[i] > target[i] {
                            self.next_state[*j].attributes[i] -=
                                (self.state[*j].attributes[i] - target[i]) * plasticity;
                        }
                    }

                    // Round if favoring integers.
                    let favor_integers = self.favor_integers;
                    if favor_integers {
                        self.next_state[*j].attributes[i] =
                            self.next_state[*j].attributes[i].round();
                    }
                }
            }
        }
    }
}
