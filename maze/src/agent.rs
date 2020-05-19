use rand::{rngs::ThreadRng, thread_rng, Rng};

use crate::direction::Direction;
use crate::{Reward, State};

use super::{EPISODE_MAX, NUM_OF_ACTION, NUM_OF_STATE};

pub struct Agent {
    start: State,
    is_finished: bool,
    learn_ratio: f64,
    discount_ratio: f64,
    q_values: [[f64; NUM_OF_ACTION]; NUM_OF_STATE],
    rand: ThreadRng,
    target: f64,
}

impl Agent {
    pub fn new(learn_ratio: f64, discount_ratio: f64, target: Reward) -> Self {
        Self {
            learn_ratio,
            discount_ratio,
            start: 1u32.into(),
            is_finished: false,
            q_values: std::default::Default::default(),
            rand: thread_rng(),
            target: target.0,
        }
    }

    pub fn reset(&mut self) {
        self.is_finished = false;
    }

    pub fn start(&self) -> State {
        self.start.into()
    }

    pub fn is_finished(&self) -> bool {
        self.is_finished
    }

    pub fn next_action(&mut self, state: State, episode: usize) -> Direction {
        let epsilon = (EPISODE_MAX - episode) as f64 / EPISODE_MAX as f64;
        if self.rand.gen::<f64>() <= epsilon {
            self.rand.gen::<u32>().into()
        } else {
            self.get_max_action(state).into()
        }
    }

    pub fn update_q(
        &mut self,
        state: State,
        action: Direction,
        next_state: State,
        reward: Reward,
    ) {
        let reward: f64 = reward.into();
        if reward == self.target {
            self.is_finished = true;
        }

        let max_q =
            self.get_q_value(next_state.into(), self.get_max_action(next_state.into()));
        let q = self.get_q_value(state, action);

        let delta_q: f64 = reward + self.discount_ratio * max_q - q;
        let state: usize = state.into();
        let action = action as usize;
        self.q_values[state][action] += self.learn_ratio * delta_q;
    }

    // NOTE: Return the action that maximizes the Q-value at the current position.
    pub fn get_max_action(&self, state: State) -> Direction {
        let state: usize = state.into();
        let mut max_action: u32 = 0;
        let mut max = -std::f64::INFINITY;

        let actions = self.q_values[state];
        for i in 0..NUM_OF_ACTION {
            if actions[i] >= max {
                max = actions[i];
                max_action = i as u32;
            }
        }
        return max_action.into();
    }

    pub fn get_q_value(&self, state: State, action: Direction) -> f64 {
        let (state, action): (usize, u32) = (state.into(), action.into());
        self.q_values[state][action as usize]
    }
}
