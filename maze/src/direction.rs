pub const NUM_OF_ACTION: usize = 4; // 行動の種類(上・右・下・左の4種類)

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Direction {
    Up = 0,
    Right = 1,
    Down = 2,
    Left = 3,
}

impl Direction {
    pub fn new(item: u32) -> Self {
        match item % NUM_OF_ACTION as u32 {
            0 => Self::Up,
            1 => Self::Right,
            2 => Self::Down,
            3 => Self::Left,
            _ => unreachable!(),
        }
    }
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Up => "up",
                Self::Right => "->",
                Self::Down => "dn",
                Self::Left => "<-",
            }
        )
    }
}

impl std::convert::From<u32> for Direction {
    fn from(from: u32) -> Self {
        Direction::new(from)
    }
}

impl std::convert::From<usize> for Direction {
    fn from(from: usize) -> Self {
        Direction::new(from as u32)
    }
}

impl std::convert::From<Direction> for u32 {
    fn from(from: Direction) -> Self {
        from as u32
    }
}

impl std::convert::From<Direction> for usize {
    fn from(from: Direction) -> Self {
        from as usize
    }
}

//impl std::convert::From<Direction> for u32{
//    fn from(from: Direction) -> Self {
//        from
//    }
//}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn direction_display_test() {
        for (v, expect) in [
            (Direction::Up, "up"),
            (Direction::Right, "->"),
            (Direction::Down, "dn"),
            (Direction::Left, "<-"),
        ]
        .iter()
        {
            assert_eq!(format!("{}", v), expect.to_string());
        }
    }

    #[test]
    fn direction_from_u32_test() {
        for (v, expect) in [
            (0, Direction::Up),
            (1, Direction::Right),
            (2, Direction::Down),
            (3, Direction::Left),
        ]
        .iter()
        {
            assert_eq!(Direction::new(*v), *expect);
        }
    }

    #[test]
    fn direction_into_u32_test() {
        for (v, expect) in [
            (Direction::Up, 0u32),
            (Direction::Right, 1u32),
            (Direction::Down, 2u32),
            (Direction::Left, 3u32),
        ]
        .iter()
        {
            assert_eq!(v.clone() as u32, *expect);
        }
    }
}
