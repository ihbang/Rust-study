use std::io;

fn main() {
    let mut n = String::new();

    io::stdin().read_line(&mut n).expect("");
    let n: usize = n.trim().parse().expect("");

    let mut count: usize = 0;
    let mut board = [[false; 15]; 15];

    backtrack(&mut board, 0, n, &mut count);
    println!("{}", count);
}

fn backtrack(board: &mut [[bool; 15]; 15], col: usize, size: usize, count: &mut usize) {
    for i in 0..size {
        let mut check = false;
        for j in 0..col {
            if j < col && board[i][j] {
                check = true;
                break;
            } else if i > j && board[i - j - 1][col - j - 1] {
                check = true;
                break;
            } else if i + j + 1 < size && board[i + j + 1][col - j - 1] {
                check = true;
                break;
            }
        }
        if !check {
            if col == size - 1 {
                *count += 1;
            } else {
                board[i][col] = true;
                backtrack(board, col + 1, size, count);
                board[i][col] = false;
            }
        }
    }
}
