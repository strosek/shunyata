pub mod universe {
    use crate::entity::entity::Entity;
    use bitvec::prelude::*;
    use rand::Rng;
    use serde::{Deserialize, Serialize};
    use std::string;
    use std::vec;

    #[derive(Serialize, Deserialize, Debug)]
    pub struct Universe {
        id: string::String,
        n_beings: usize,
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
        pub fn new(id: string::String, n_beings: usize) -> Universe {
            let state = vec::Vec::<Entity>::new();
            let next_state = vec::Vec::<Entity>::new();

            Universe {
                id,
                n_beings,
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
                let plasticity = rng.gen_range(0.1, 0.9);
                let influence = rng.gen_range(0.1, 0.9);
                let color = rng.gen_range(0x000000, 0xFFFFFF);

                self.state.push(Entity::new(
                    i as u32,
                    "entity".to_string(),
                    plasticity,
                    influence,
                    color,
                ));

                self.next_state.push(Entity::new(
                    i as u32,
                    "entity".to_string(),
                    plasticity,
                    influence,
                    color,
                ));
            }
        }

        fn tick(&self) {
            let mut meets: vec::Vec<usize>;
            for i in 0..self.state.len() {
                self.evaluate_interaction(&self.get_who());
            }
        }

        fn get_who(&self) -> &Vec<usize> {
            /* Get who meets who, according to spacial collisions, randomly or whatever algorithm
             * is implemented.
             */
            let mut rng = rand::thread_rng();
            let mut interacted = bitvec![0; self.n_beings];
            interacted.set_all(false);

            let n_meets = rng.gen_range(0, self.n_beings / 10);

            let mut meets = &vec::Vec::<usize>::new();
            for hello in meets {
                meets.push(rng.gen_range(0, self.n_beings));
            }

            meets
        }

        fn evaluate_interaction(&self, meets: &vec::Vec<usize>) {
            let mut sum = 0;
            for meet in meets {
                sum += self.state[meet].color;
            }
        }
    }
}
