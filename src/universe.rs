use Entity;

mod universe {
    pub struct Universe {
        id: u32,
        state: vec<Entity>,
        nextState: vec<Entity>,
    }

    /* Methods:
     *   - Spawn()
     *   - Anihilate() // Destructor
     *   - Tick()
     *   - Save()
     *   - Interact()
     *   - UpdateRelationships()
     */

    pub fn spawn(self, &configFile: str) {
        /* - Initialize all entities according to config file.
         * - Populate and replicate until n entities randomly or in a specified proportion.
         * - nextState starts being a copy if current state.
         */
    }

    fn tick() {
        let mut meets;
        for entity in self.state {
            meets = self.getWho();
            for meet in meets {
                evaluateInteraction(entity, meets);
            }
        }
    }

    fn getWho() {
        /* Get who meets who, according to spacial collisions, randomly or whatever algorithm
         * is implemented.
         */
    }

    pub fn evaluateInteraction(&entity: Entity, indexes: vec<u64>) {
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

