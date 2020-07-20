pub mod universe {
    use std::vec;
    use std::string;
    use crate::entity::entity::Entity;
    use serde::{Serialize, Deserialize};
    use rand::Rng;


    #[derive(Serialize, Deserialize, Debug)]
    pub struct Universe {
        id: string::String,
        n_beings: u32,
        state: vec::Vec<Entity>,
        next_state: vec::Vec<Entity>,
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
        pub fn new(id: string::String, n_beings: u32) -> Universe {
            let state = vec::Vec::<Entity>::new();
            let next_state = vec::Vec::<Entity>::new();

            Universe{id, n_beings, state, next_state}
        }

        pub fn spawn(&mut self, _config_file: &str) {
            /* - Initialize all entities according to config file.
             * - Populate and replicate until n entities randomly or in a specified proportion.
             * - nextState starts being a copy if current state.
             */
            let mut rng = rand::thread_rng();
            println!("Integer: {}", rng.gen_range(0, 10));
            println!("Float: {}", rng.gen_range(0.0, 10.0));
        }

        fn tick(&self) {
            let mut meets;
            for entity_idx in 0..self.state.len() {
                meets = self.get_who();
                for meet in meets {
                    self.evaluate_interaction(entity_idx, meet);
                }
            }
        }

        fn get_who(&self) -> Vec<usize> {
            /* Get who meets who, according to spacial collisions, randomly or whatever algorithm
             * is implemented.
             */
            vec![1, 2, 3]
        }

        fn evaluate_interaction(&self, _e1_idx: usize, _e2_idx: usize) {
            /* Basic interaction algorithm for numeric traits:
             *
             * if e1.a1 == e2.a1
             *   reduce plasticity, don't change values.
             * else if e1.a1 > e2.a1
             *   e1.a1 -= (e1.a1 - e2.a1) * e2.influence * e1.plasticity
             *   e2.a1 += (e1.a1 - e2.a1) * e1.influence * e2.plasticity
             * else
             *   e1.a1 += (e1.a1 - e2.a1) * e2.influence * e1.plasticity
             *   e2.a1 -= (e1.a1 - e2.a1) * e1.influence * e2.plasticity
             *
             * - Results are stored in the nextState copy of the universe.
             * - All interactions happen instantly.
             */
        }
    }
}

