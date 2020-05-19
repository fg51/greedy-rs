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
    let mut agent = Agent::new(LEARN_RATIO, DISCOUNT_RATIO); //initialize(rl);

    for i in 0..EPISODE_MAX {
        learn_one_episode(&maze, &mut agent, i);

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

fn learn_one_episode(maze: &Maze, agent: &mut Agent, episode: usize) {
    let mut position = agent.start();
    let mut cnt = 0;
    loop {
        // move through the maze until reaching the goal.
        if position == agent.goal() {
            break;
        }
        let action = agent.next_action(position, episode);
        let next_position = maze.next(position, &action);
        agent.update_q(position, &action, next_position);

        position = next_position;
        cnt += 1;
    }
    println!("#episode, {}, count_until_goal, {}", episode, cnt);
}

fn output_result(agent: &Agent) {
    for s in 1..NUM_OF_STATE - 1 {
        let max_a = agent.get_max_action(s as u32);
        print!("{}", Direction::new(max_a));
    }
    print!("\n");

    for state in 1..NUM_OF_STATE - 1 {
        let state = state as u32;
        let max_a = agent.get_max_action(state);

        print!(
            "state:, {}, max action:, {}, ",
            state,
            Direction::new(max_a)
        );

        for action in 0..NUM_OF_ACTION {
            print!(
                "{}: ,{:.4},",
                Direction::new(action as u32),
                agent.get_q_value(state, action as u32)
            );
        }
        print!("\n");
    }
}
