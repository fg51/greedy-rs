use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::direction::Direction;

use super::{EPISODE_MAX, NUM_OF_ACTION, NUM_OF_STATE};

pub struct Agent {
    start: u32,
    goal: u32,
    learn_ratio: f64,
    discount_ratio: f64,
    q_values: [[f64; NUM_OF_ACTION]; NUM_OF_STATE],
    rand: ThreadRng,
}

impl Agent {
    pub fn new(learn_ratio: f64, discount_ratio: f64) -> Self {
        Self {
            learn_ratio,
            discount_ratio,
            start: 1,
            goal: 17,
            q_values: std::default::Default::default(),
            rand: thread_rng(),
        }
    }

    pub fn start(&self) -> u32 {
        self.start
    }

    pub fn goal(&self) -> u32 {
        self.goal
    }

    pub fn next_action(&mut self, s: u32, episode: usize) -> Direction {
        let p: f64 = self.rand.gen();
        let epsilon = (EPISODE_MAX - episode) as f64 / EPISODE_MAX as f64;
        Direction::new(if p <= epsilon {
            self.rand.gen::<u32>()
        } else {
            self.get_max_action(s)
        })
    }

    pub fn update_q(&mut self, state: u32, action: &Direction, next_state: u32) {
        let (state, action) = (state as usize, action.clone() as usize);
        let reward = if next_state == self.goal {
            1.0
        } else if next_state == state as u32 {
            -1.0
        } else {
            let max_action = self.get_max_action(next_state) as usize;
            self.learn_ratio
                * (self.discount_ratio * self.q_values[next_state as usize][max_action]
                    - self.q_values[state][action])
        };
        self.q_values[state][action] += reward;
    }

    // NOTE: Return the action that maximizes the Q-value at the current position.
    pub fn get_max_action(&self, state: u32) -> u32 {
        let state = state as usize;
        let mut max_action: u32 = 0;
        let mut max = -100000.0;

        let actions = self.q_values[state];
        for i in 0..NUM_OF_ACTION {
            if actions[i] >= max {
                max = actions[i];
                max_action = i as u32;
            }
        }
        return max_action;
    }

    pub fn get_q_value(&self, state: u32, action: u32) -> f64 {
        self.q_values[state as usize][action as usize]
    }
}
