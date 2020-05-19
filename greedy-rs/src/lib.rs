use rand::{rngs::ThreadRng, thread_rng, Rng};

pub fn mean(xs: &[f64]) -> f64 {
    let mut num: usize = 0;
    let mut sum = 0.;
    for i in xs {
        sum += i;
        num += 1;
    }
    sum / num as f64
}

pub struct Agent {
    epsilon: f64,
    values: Vec<f64>,
    rng: ThreadRng,
}

impl<'a> Agent {
    pub fn new(epsilon: f64) -> Self {
        Self {
            epsilon,
            values: vec![],
            rng: thread_rng(),
        }
    }

    fn policy(&mut self) -> usize {
        if self.rng.gen::<f64>() < self.epsilon {
            let coins = 0..self.values.len();
            let y: f64 = (coins.len() as f64) * self.rng.gen::<f64>();
            let index = y.round() as usize;
            for i in 0..self.values.len() {
                if i == index {
                    return i;
                }
            }
            return self.values.len() - 1;
        } else {
            return Self::argmax(&self.values);
        }
    }

    fn argmax(xs: &[f64]) -> usize {
        let mut index = 0;
        let mut max = 0.;
        for (i, &val) in xs.iter().enumerate() {
            if val >= max {
                index = i;
                max = val;
            }
        }
        return index;
    }

    pub fn play(&mut self, env: &mut CoinToss) -> Vec<f64> {
        let mut nums = vec![0; env.length()];
        self.values = vec![0.; env.length()];

        env.reset();
        let mut done = false;
        let mut rewards = vec![];
        loop {
            if done == true {
                break;
            }
            let selected_coin = self.policy();
            let (reward, done1) = env.step(selected_coin).unwrap();
            done = done1;
            rewards.push(reward);
            self.update_average(reward, selected_coin, &mut nums);
        }
        return rewards;
    }

    fn update_average(&mut self, reward: f64, selected_coin: usize, nums: &mut [usize]) {
        let n = nums[selected_coin];
        let coin_average = self.values[selected_coin];
        let new_average = (coin_average * n as f64 + reward) / (n + 1) as f64;
        nums[selected_coin] += 1;
        self.values[selected_coin] = new_average;
    }
}

pub struct CoinToss {
    head_probs: Vec<f64>,
    pub max_episode_steps: usize,
    toss_count: usize,
    rng: ThreadRng,
}

impl CoinToss {
    pub fn new(head_probs: Vec<f64>) -> Self {
        Self {
            head_probs: head_probs,
            max_episode_steps: 30,
            toss_count: 0,
            rng: thread_rng(),
        }
    }

    pub fn length(&self) -> usize {
        self.head_probs.len()
    }

    pub fn reset(&mut self) {
        self.toss_count = 0;
    }

    pub fn step(&mut self, action: usize) -> Result<(f64, bool), String> {
        let final_count = self.max_episode_steps - 1;
        if self.toss_count > final_count {
            return Err("The step count exceeded maximum. Please reset env.".to_string());
        }
        let done = if self.toss_count == final_count {
            true
        } else {
            false
        };

        if action >= self.head_probs.len() {
            return Err(format!("The No.{} coin does'n exist.", action));
        }
        let head_prob = self.head_probs[action];
        let reward = if self.rng.gen::<f64>() < head_prob {
            1.0
        } else {
            0.
        };
        self.toss_count += 1;
        return Ok((reward, done));
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
