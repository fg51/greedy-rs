use crate::direction::{Direction, NUM_OF_ACTION};

pub const NUM_OF_STATE: usize = 18; // 16 position + goal: 17 + start: 0 = total: 18

const MAZE: [[u32; NUM_OF_ACTION]; NUM_OF_STATE] = [
    [0, 0, 0, 0], // start
    [1, 2, 5, 1], // position[1]
    [2, 3, 2, 1],
    [3, 4, 3, 2],
    [4, 4, 8, 3],
    [1, 6, 9, 5], // position[5]
    [6, 7, 10, 5],
    [7, 7, 11, 6],
    [4, 8, 12, 8],
    [5, 9, 9, 9],
    [6, 10, 14, 10], // position[10]
    [7, 11, 15, 11],
    [8, 12, 12, 12],
    [13, 14, 13, 13],
    [10, 14, 14, 13],
    [11, 16, 15, 15], // position[15]
    [16, 17, 16, 15], // position[16]
    [0, 0, 0, 0],     // goal, position[17]
];

pub struct Maze([[u32; NUM_OF_ACTION]; NUM_OF_STATE]);

impl Maze {
    pub fn new() -> Self {
        Self(MAZE)
    }

    pub fn next(&self, position: u32, action: &Direction) -> u32 {
        return MAZE[position as usize][action.clone() as usize];
    }
}
