use std::{io, str::SplitWhitespace};

enum Direction {
    Up,
    Right,
    Down,
    Left,
}

impl Direction {
    fn turn_left(&self) -> Direction {
        match self {
            Direction::Up => Direction::Left,
            Direction::Right => Direction::Up,
            Direction::Down => Direction::Right,
            Direction::Left => Direction::Down,
        }
    }
}

struct Position {
    row: usize,
    col: usize,
    direction: Direction,
}

impl Position {
    fn turn_left(&mut self) {
        self.direction = self.direction.turn_left();
    }

    fn move_forward(&mut self) {
        match self.direction {
            Direction::Up => {
                self.row = self.row - 1;
            }
            Direction::Right => {
                self.col = self.col + 1;
            }
            Direction::Down => {
                self.row = self.row + 1;
            }
            Direction::Left => {
                self.col = self.col - 1;
            }
        }
    }

    fn move_backward(&mut self) {
        match self.direction {
            Direction::Up => {
                self.row = self.row + 1;
            }
            Direction::Right => {
                self.col = self.col - 1;
            }
            Direction::Down => {
                self.row = self.row - 1;
            }
            Direction::Left => {
                self.col = self.col + 1;
            }
        }
    }
}

#[derive(PartialEq)]
enum Status {
    Dirty,
    Clean,
    Wall,
}

fn str_to_status(strs: SplitWhitespace) -> Vec<Status> {
    let mut vector: Vec<Status> = Vec::new();

    for element in strs {
        let element: u8 = element.parse().unwrap();
        match element {
            0 => vector.push(Status::Dirty),
            1 => vector.push(Status::Wall),
            _ => vector.push(Status::Clean),
        }
    }
    vector
}

fn clear_room(room: &mut Vec<Vec<Status>>, pos: &mut Position, count: &mut u16) {
    let row = pos.row;
    let col = pos.col;

    if room[row][col] == Status::Dirty {
        room[row][col] = Status::Clean;
        *count = *count + 1;
    }

    for _i in 0..4 {
        let row_front: usize;
        let col_front: usize;

        pos.turn_left();
        match pos.direction {
            Direction::Up => {
                row_front = row - 1;
                col_front = col;
            }
            Direction::Right => {
                row_front = row;
                col_front = col + 1;
            }
            Direction::Down => {
                row_front = row + 1;
                col_front = col;
            }
            Direction::Left => {
                row_front = row;
                col_front = col - 1;
            }
        }
        if room[row_front][col_front] == Status::Dirty {
            pos.move_forward();
            return clear_room(room, pos, count);
        }
    }

    let row_back: usize;
    let col_back: usize;
    match pos.direction {
        Direction::Up => {
            row_back = row + 1;
            col_back = col;
        }
        Direction::Right => {
            row_back = row;
            col_back = col - 1;
        }
        Direction::Down => {
            row_back = row - 1;
            col_back = col;
        }
        Direction::Left => {
            row_back = row;
            col_back = col + 1;
        }
    }
    if room[row_back][col_back] == Status::Wall {
        return;
    } else {
        pos.move_backward();
        return clear_room(room, pos, count);
    }
}

fn main() {
    let mut inputs = String::new();

    io::stdin().read_line(&mut inputs).expect("");
    let size: Result<Vec<u8>, _> = inputs.split_whitespace().map(str::parse).collect();

    let size = size.unwrap();
    let row_cnt = size[0];

    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).expect("");
    let inputs: Result<Vec<usize>, _> = inputs.split_whitespace().map(str::parse).collect();
    let inputs = inputs.unwrap();

    let mut pos = Position {
        row: inputs[0],
        col: inputs[1],
        direction: match inputs[2] {
            0 => Direction::Up,
            1 => Direction::Right,
            2 => Direction::Down,
            _ => Direction::Left,
        },
    };
    let mut room: Vec<Vec<Status>> = Vec::new();
    let mut count: u16 = 0;

    for _i in 0..row_cnt {
        let mut line = String::new();

        io::stdin().read_line(&mut line).expect("");
        let line = str_to_status(line.split_whitespace());
        room.push(line);
    }
    clear_room(&mut room, &mut pos, &mut count);
    println!("{}", count);
}
