# Shunyata

Shunyata is an influence simulator.

***Śūnyatā*** is a Buddhist concept related to the *interdependence* of all things in the universe, which this simulator tries to be a playground of.

The simulation represents entities that interact with others and become more
similar to the others. The effects of interaction should be easy to change and, as they become more complex, they should produce more interesting phenomena within this little universe.

### Current simulation characteristics

- Interactions between Entities are random.
- Interaction between more than two Entities is possible.
- Interaction between similar Entities is stronger.
- An entity may have plasticity, which determines the probability of being
  influenced on an interaction.
- Interact function is independent of when an interaction happens and between who, so multiple interaction algorithms can be implemented.
- Entities have a coefficient of plasticity and influence, that is how easily they learn from others, and how easily they influence others.
- Implemented in Rust to make it fast and well done!

### Possible uses of this software

- Optimizing traits of anything, given that you have a fitness function.
- Pleasing the curious mind.



### Future ideas

- Entities can have different plasticity, that is, how much it can be influenced by others.
- Factors that could change interactions:
  - Value diversity of entities.
  - A fitness function that gives purpose to the "evolution", or learning.
  - An entity could teach useful traits to others, based on its success with the fitness functions.
  - Group effects like gravity.
  - Radius of influence.
  - Movement, energy, impetus.
  - "Eating" other entities.
  - Death/Birth of entities.
  - Keeping ties with previous entities with which an entity interacted. Chaining interactions with known entities.
  - Influence errors, mimic replication errors of evolution.
  - Evolve based on fitness function when a "universe" becomes stable, that is no entities change much.
  - Introduce the concept of compassion vs selfishness.
  - An entity may have an influencer coefficient.
  - Entities can influence others in a truthful or lier way.
  - Implement geographic interactions, based on vectors of 2, 3 or 4 dimensions.
- Read Json or similar config file with characteristics of the universe and the entities in it.