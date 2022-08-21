use crate::math::Position;
use crate::universe::{fitness, UNIVERSE_HEIGHT, UNIVERSE_WIDTH};
use rand::Rng;
use std::fmt;

pub const N_VALUES: usize = 6;
pub const MAX_ENERGY: u64 = 300;
pub const MAX_RADIUS: f64 = 8.0;

pub struct Entity {
    pub id: usize,
    pub values: Vec<f64>,
    pub initial_energy: u64,
    pub energy: u64,
    pub position: Position,
    pub radius: f64,
}

impl Entity {
    pub fn with_id(id: usize) -> Entity {
        let mut rng = rand::thread_rng();
        let mut initial_values = Vec::with_capacity(N_VALUES);
        let colors = vec![
            [1.0, 0.0, 0.0], // red
            [0.0, 1.0, 0.0], // green
            [0.0, 0.0, 1.0], // blue
            [1.0, 1.0, 0.0], // yellow
            [1.0, 0.0, 1.0], // magenta
            [0.0, 1.0, 1.0], // cyan
        ];
        let color = colors[rng.gen_range(0..colors.len())];
        initial_values.push(color[0]);
        initial_values.push(color[1]);
        initial_values.push(color[2]);

        // General values.
        for _ in 3..N_VALUES {
            initial_values.push(rng.gen_range(-1000.0..1000.0));
        }

        let initial_energy = rng.gen_range(0..MAX_ENERGY);

        Entity {
            id,
            values: initial_values,
            initial_energy,
            energy: initial_energy,
            position: Position {
                x: rng.gen_range(0.0..UNIVERSE_WIDTH),
                y: rng.gen_range(0.0..UNIVERSE_HEIGHT),
            },
            radius: rng.gen_range(1.0..MAX_RADIUS),
        }
    }
}

impl Clone for Entity {
    fn clone(&self) -> Entity {
        Entity {
            id: self.id,
            values: self.values.to_vec(),
            initial_energy: self.initial_energy,
            energy: self.energy,
            position: self.position.clone(),
            radius: self.radius,
        }
    }
}

impl fmt::Debug for Entity {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            //"v:{} e:{}",
            //self.value, self.energy,
            "i:{} s:{:.2} p:({:.2},{:.2}) f:{:.5} v:{:?}: e: {}",
            self.id,
            self.values.iter().sum::<f64>(),
            self.position.x,
            self.position.y,
            fitness(self),
            self.values,
            self.energy,
        )
    }
}

#[cfg(test)]
mod tests {}
