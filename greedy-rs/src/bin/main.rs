extern crate greedy_rs as lib;

use lib::{mean, Agent, CoinToss};

pub fn main() {
    let mut environment = CoinToss::new(vec![0.1, 0.5, 0.1, 0.9, 0.1]);
    let epsilons = [0., 0.1, 0.2, 0.5, 0.8];
    let game_steps: Vec<usize> = (1..32).map(|x| x * 10).collect();
    let mut result = ResultData::new();

    for &e in &epsilons {
        let mut agent = Agent::new(e);
        let mut means = vec![];
        for &i in &game_steps {
            environment.max_episode_steps = i;
            let rewards = agent.play(&mut environment);
            means.push(mean(&rewards))
        }
        result.epsilon.push((e, means));
    }
    result.coin_toss_count.extend(game_steps);
    println!("{:?}", result.epsilon);
    println!("{:?}", result.coin_toss_count);
}

struct ResultData {
    epsilon: Vec<(f64, Vec<f64>)>,
    coin_toss_count: Vec<usize>,
}

impl ResultData {
    pub fn new() -> Self {
        Self {
            epsilon: vec![],
            coin_toss_count: vec![],
        }
    }
}
