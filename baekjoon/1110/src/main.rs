use std::io;

fn main() {
    let mut orig_number = String::new();
    io::stdin()
        .read_line(&mut orig_number)
        .expect("cannot reach here");

    let orig_number: u32 = orig_number.trim().parse().expect("cannot reach here");
    let mut number = orig_number;
    let mut cycle: u32 = 1;

    loop {
        let n1 = number % 10;
        let n2 = (number / 10 + n1) % 10;
        number = n1 * 10 + n2;

        if number == orig_number {
            break;
        } else {
            cycle = cycle + 1;
        }
    }
    println!("{}", cycle);
}
