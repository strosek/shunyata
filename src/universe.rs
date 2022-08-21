extern crate piston_window;

use piston_window::*;
use rand::Rng;
use rayon::prelude::*;

use crate::entity::{Entity, N_VALUES};
use crate::math::distance;

const N_ENTITIES: usize = 5000;
const SUCCESS_MARGIN: f64 = 0.00001;
const WINDOW_WIDTH: u32 = 3840;
const WINDOW_HEIGHT: u32 = 2160;

pub const UNIVERSE_WIDTH: f64 = (WINDOW_WIDTH * 2) as f64;
pub const UNIVERSE_HEIGHT: f64 = (WINDOW_HEIGHT * 2) as f64;

fn make_similar(current_entity: &mut Entity, other_entity: &Entity) {
    for i in 0..N_VALUES {
        if fitness(current_entity) < fitness(other_entity) {
            if current_entity.values[i] < other_entity.values[i] {
                current_entity.values[i] = current_entity.values[i]
                    + ((other_entity.values[i] - current_entity.values[i]) / 2.0)
                        / distance(&current_entity.position, &other_entity.position);
            } else if current_entity.values[i] > other_entity.values[i] {
                current_entity.values[i] = current_entity.values[i]
                    - ((current_entity.values[i] - other_entity.values[i]) / 2.0)
                        / distance(&current_entity.position, &other_entity.position);
            }
        }
    }
}

fn move_entity(current_entity: &mut Entity, _state: &[Entity]) {
    let mut rng = rand::thread_rng();
    current_entity.position.x += rng.gen_range(-5.0..5.0);
    current_entity.position.y += rng.gen_range(-5.0..5.0);
}

fn interact(current_entity: &mut Entity, state: &[Entity]) {
    for other_entity in state {
        if fitness(other_entity) < -SUCCESS_MARGIN
            && state[current_entity.id].energy > 0
            && other_entity.energy > 0
            && state[current_entity.id].id != other_entity.id
        {
            make_similar(current_entity, other_entity);
            move_entity(current_entity, state);
        }
    }

    if current_entity.energy > 0 {
        current_entity.energy -= 1;
    }
}

pub fn fitness(entity: &Entity) -> f64 {
    // Finding values that satisfy the equation:  2*sin(x)^2 + y + 3*z^(1/2) = 666
    -(((2.0 * entity.values[3].sin()).powi(2)
        + entity.values[4]
        + 3.0 * entity.values[5].sqrt())
        - 666.0)
        .abs()
}

fn _print_state_random(limit: usize, state: &[Entity]) {
    println!("State sample ({}):", limit);

    let mut rng = rand::thread_rng();
    for _ in 0..limit {
        let index = rng.gen_range(0..state.len());
        println!("{:?}, ", state[index]);
    }
}

fn print_state_first(limit: usize, state: &[Entity]) {
    println!("State sample ({}):", limit);

    for i in 0..limit {
        println!("{:?}, ", state[i]);
    }
}

fn count_successful(state: &[Entity]) -> usize {
    state
        .par_iter()
        .filter(|e| fitness(*e) > -SUCCESS_MARGIN)
        .count()
}

fn print_state_successes(limit: usize, state: &[Entity]) {
    println!("Succeeded entities: {}", count_successful(state));
    for entity in state
        .iter()
        .filter(|e| fitness(*e) > -SUCCESS_MARGIN)
        .take(limit)
    {
        println!("{:?}, ", entity);
    }
}

fn generate_initial_state(n_entities: usize) -> Vec<Entity> {
    let mut state = Vec::with_capacity(n_entities);
    for i_entity in 0..n_entities {
        state.push(Entity::with_id(i_entity));
    }
    state
}

pub fn run_simulation() {
    let mut state = generate_initial_state(N_ENTITIES);
    let mut next_state = state.to_vec();

    println!("Processing {} entities", N_ENTITIES);
    print_state_first(20, &state);

    let mut window: PistonWindow = WindowSettings::new(
        "Shunyata: influence simulator",
        [WINDOW_WIDTH, WINDOW_HEIGHT],
    )
    .exit_on_esc(true)
    .fullscreen(true)
    .graphics_api(OpenGL::V3_2)
    .build()
    .unwrap();

    let mut i = 0usize;
    while let Some(event) = window.next() {
        if i % 10usize == 0 {
            window.draw_2d(&event, |context, graphics, _device| {
                clear([0.0, 0.0, 0.0, 1.0], graphics);

                for entity in &state {
                    if entity.position.x > 0.0
                        && entity.position.x < WINDOW_WIDTH as f64
                        && entity.position.y > 0.0
                        && entity.position.y < WINDOW_HEIGHT as f64
                    {
                        let alpha = 1.0 / entity.initial_energy as f32 * entity.energy as f32;
                        circle_arc(
                            [
                                entity.values[0] as f32,
                                entity.values[1] as f32,
                                entity.values[1] as f32,
                                alpha,
                            ],
                            entity.radius,
                            0.0,
                            f64::_360(),
                            [
                                entity.position.x.round(),
                                entity.position.y.round(),
                                entity.radius * 2.0,
                                entity.radius * 2.0,
                            ],
                            context.transform,
                            graphics,
                        );

                        let remaining_energy =
                            1.0 / entity.initial_energy as f64 * entity.energy as f64;
                        circle_arc(
                            [1.0, 1.0, 1.0, 1.0],
                            0.5,
                            0.0,
                            f64::_360() * remaining_energy,
                            //f64::_360(),
                            [
                                entity.position.x.round(),
                                entity.position.y.round(),
                                entity.radius * 2.0,
                                entity.radius * 2.0,
                            ],
                            context.transform,
                            graphics,
                        );
                    }
                }
            });
        }

        next_state.par_iter_mut().for_each(|entity| {
            interact(entity, &state);
        });
        state = next_state.to_vec();

        i += 1;

        if state.par_iter().all(|e| e.energy <= 0) {
            println!("All entities died.\n");
            break;
        }
    }

    println!("Total cycles: {}", i);

    print_state_first(20, &state);
    print_state_successes(5, &state);
}
