mod universe {
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
     */
    pub struct Universe {
        id: u32,
        /* Contents of struct:
         *   - list of entities
         *   - support ds.
         * 
         * Methods:
         *   - Spawn()
         *   - Anihilate()
         *   - Tick()
         *   - Save()
         *   - Interact()
         *   - UpdateRelationships()
         */
    }
}

