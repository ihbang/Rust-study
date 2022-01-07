use std::io;

static mut ROW_CNT: usize = 0;
static mut COL_CNT: usize = 0;
static mut BOARD: Vec<Vec<char>> = Vec::new();

unsafe fn travel(row: usize, col: usize, len: u8, max: &mut u8, visited: &mut [bool; 26]) {
    let chr = BOARD[row][col];
    let idx = chr as usize - 'A' as usize;

    if !visited[idx] {
        visited[idx] = true;

        let len = len + 1;
        if len > *max {
            *max = len;
        }
        if *max >= 26 {
            return;
        }
        if row < ROW_CNT - 1 {
            travel(row + 1, col, len, max, visited);
        }
        if col < COL_CNT - 1 {
            travel(row, col + 1, len, max, visited);
        }
        if row > 0 {
            travel(row - 1, col, len, max, visited);
        }
        if col > 0 {
            travel(row, col - 1, len, max, visited);
        }
        visited[idx] = false;
    }
}

fn main() {
    let mut inputs = String::new();
    let mut visited = [false; 26];

    io::stdin().read_line(&mut inputs).expect("");

    let inputs: Vec<&str> = inputs.trim().split_whitespace().collect();
    unsafe {
        let mut max = 0;
        ROW_CNT = inputs[0].parse().expect("");
        COL_CNT = inputs[1].parse().expect("");

        for _row in 0..ROW_CNT {
            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("");
            BOARD.push(input.chars().collect());
        }
        travel(0, 0, 0, &mut max, &mut visited);
        println!("{}", max);
    }
}
