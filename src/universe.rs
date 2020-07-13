mod universe {
    use std::vec;
    use crate::entity::entity::Entity;

    pub struct Universe {
        id: u32,
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
        pub fn spawn(&mut self, config_file: &str) {
            /* - Initialize all entities according to config file.
             * - Populate and replicate until n entities randomly or in a specified proportion.
             * - nextState starts being a copy if current state.
             */
            self.state = vec::Vec::<Entity>::new();
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

        fn evaluate_interaction(&self, e1_idx: usize, e2_idx: usize) {
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

