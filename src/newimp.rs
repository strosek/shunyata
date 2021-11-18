use core::fmt;
use rand::Rng;
use rayon::prelude::*;
use std::collections::HashMap;

const N_ENTITIES: usize = 100000;
const N_CYCLES: usize = 100;

pub struct Position {
    x: f64,
    y: f64,
}

impl Clone for Position {
    fn clone(&self) -> Self {
        Position {
            x: self.x,
            y: self.y,
        }
    }
}

pub struct Entity {
    id: usize,
    value: u64,
    energy: u64,
    position: Position,
}

impl Entity {
    fn with_id(id: usize) -> Entity {
        let mut rng = rand::thread_rng();
        Entity {
            id,
            value: rng.gen_range(0..100000),
            energy: rng.gen_range(0..N_CYCLES as u64),
            position: Position {
                x: rng.gen_range(-100.0..100.0),
                y: rng.gen_range(-100.0..100.0),
            },
        }
    }
}

impl Clone for Entity {
    fn clone(&self) -> Entity {
        Entity {
            id: self.id,
            value: self.value,
            energy: self.energy,
            position: self.position.clone(),
        }
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            //"v:{} e:{}",
            //self.value, self.energy,
            "v:{}",
            self.value,
        )
    }
}

fn distance(a: &Position, b: &Position) -> f64 {
    ((a.x - b.x).powi(2) + (a.y - b.y).powi(2)).sqrt()
}

fn interact(current_entity: &mut Entity, state: &[Entity]) {
    let mut sum = 0u64;
    let mut alive = 0usize;

    for entity in state {
        let distance = distance(&current_entity.position, &entity.position);

        if current_entity.id != entity.id && current_entity.energy > 0 {
            if distance > 1.0 {
                sum += entity.value / distance.powi(2) as u64;
            }
            sum += entity.value;
            alive += 1;
        }
    }

    if current_entity.energy > 0 {
        current_entity.energy -= 1;
    }

    if alive > 0 {
        current_entity.value = sum / alive as u64;
    }
}

fn main() {
    let mut state = Vec::with_capacity(N_ENTITIES);

    for i_entity in 0..N_ENTITIES {
        state.push(Entity::with_id(i_entity));
    }
    let mut next_state = state.to_vec();

    let mut initial_buckets = HashMap::<u64, usize>::with_capacity(N_ENTITIES);
    for entity in &state {
        if initial_buckets.contains_key(&entity.value) {
            initial_buckets.insert(entity.value, initial_buckets[&entity.value] + 1);
        } else {
            initial_buckets.insert(entity.value, 1);
        }
    }
    println!("Initial diversity: {}", initial_buckets.len());

    println!(
        "Avg: {}",
        state.par_iter().map(|entity| entity.value).sum::<u64>() / N_ENTITIES as u64
    );

    //println!("{:?}", state);
    for i in 0..N_CYCLES {
        next_state.par_iter_mut().for_each(|entity| {
            interact(entity, &state);
        });

        state = next_state.to_vec();
        //println!("i:{} - {:?}", i, state);
        println!("i:{}", i);
    }

    let mut buckets = HashMap::<u64, usize>::with_capacity(N_ENTITIES);
    for entity in state {
        if buckets.contains_key(&entity.value) {
            *buckets.get_mut(&entity.value).unwrap() += 1;
        } else {
            buckets.insert(entity.value, 1);
        }
    }

    println!("Final diversity: {}", buckets.len());
    println!("Done.")
}
