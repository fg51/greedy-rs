mod agent;
use agent::Agent;

mod maze_data;
use maze_data::{Maze, NUM_OF_STATE};

mod direction;
use direction::{Direction, NUM_OF_ACTION};

const EPISODE_MAX: usize = 100;
const DISCOUNT_RATIO: f64 = 0.95;
const LEARN_RATIO: f64 = 0.5; //0.10;

pub fn run() {
    let maze = Maze::new();
    let goal = 17;
    let environment = Environment::new(goal);
    let mut agent = Agent::new(LEARN_RATIO, DISCOUNT_RATIO, environment.max_reward()); //initialize(rl);

    for i in 0..EPISODE_MAX {
        learn_one_episode(&maze, &environment, &mut agent, i);

        // episode = 0, 半分経過、最後のタイミングだけ出力する
        if i == 0 {
            output_result(&agent);
        }
        if i + 1 == EPISODE_MAX / 2 {
            output_result(&agent);
        }
        if i + 1 == EPISODE_MAX {
            output_result(&agent);
        }
    }
}

fn learn_one_episode(
    maze: &Maze,
    environment: &Environment,
    agent: &mut Agent,
    episode: usize,
) {
    agent.reset();

    let mut position = agent.start();
    let mut cnt = 0;
    loop {
        // move through the maze until reaching the goal.
        if agent.is_finished() {
            break;
        }
        let action = agent.next_action(position, episode);
        let next_position = maze.next(position.into(), &action);
        agent.update_q(
            position,
            action,
            next_position.into(),
            environment.reward(position.into(), next_position.into()),
        );

        position = next_position;
        cnt += 1;
    }
    println!("#episode, {}, count_until_goal, {}", episode, cnt);
}

#[derive(Clone, Copy)]
pub struct State(u32);

impl std::convert::From<State> for u32 {
    fn from(from: State) -> Self {
        from.0
    }
}

impl std::convert::From<State> for usize {
    fn from(from: State) -> Self {
        let from = from.0;
        return from as usize;
    }
}

impl std::convert::From<u32> for State {
    fn from(from: u32) -> Self {
        Self(from)
    }
}

impl std::convert::From<usize> for State {
    fn from(from: usize) -> Self {
        Self(from as u32)
    }
}

pub struct Reward(f64);

impl std::convert::From<Reward> for f64 {
    fn from(from: Reward) -> Self {
        from.0
    }
}

impl std::convert::From<f64> for Reward {
    fn from(from: f64) -> Self {
        Self(from)
    }
}

struct Environment {
    goal: u32,
    success: f64,
    penalty: f64,
}

impl Environment {
    pub fn new(goal: u32) -> Self {
        Self {
            goal,
            success: 1.,
            penalty: -1.,
        }
    }

    pub fn max_reward(&self) -> Reward {
        self.success.into()
    }

    pub fn reward(&self, state: u32, next_state: u32) -> Reward {
        if next_state == self.goal {
            return self.success.into();
        }
        if next_state == state {
            return self.penalty.into();
        }
        return 0f64.into();
    }
}

fn output_result(agent: &Agent) {
    for s in 1..NUM_OF_STATE - 1 {
        let max_a = agent.get_max_action((s as u32).into());
        print!("{}", max_a);
    }
    print!("\n");

    for state in 1..NUM_OF_STATE - 1 {
        let max_a = agent.get_max_action((state as u32).into());

        print!("state:, {}, max action:, {}, ", state, max_a);

        for action in 0..NUM_OF_ACTION {
            print!(
                "{}: ,{:.4},",
                Direction::new(action as u32),
                agent.get_q_value(state.into(), action.into())
            );
        }
        print!("\n");
    }
}
