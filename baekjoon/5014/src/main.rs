use std::io;

fn main() {
    let mut inputs = String::new();
    io::stdin().read_line(&mut inputs).expect("");
    let inputs: Result<Vec<u32>, _> = inputs.split_whitespace().map(str::parse).collect();
    let inputs = inputs.unwrap();

    let uppermost = inputs[0];
    let mut current = inputs[1];
    let goal = inputs[2];
    let up = inputs[3];
    let down = inputs[4];

    let mut count = 0;

    while (count <= uppermost) && (current > 0) && (current <= uppermost) {
        if current == goal {
            println!("{}", count);
            return;
        }
        if (current > down) && (current > goal) {
            current -= down;
        } else if (current + up > uppermost) && (current > down) {
            current -= down;
        } else {
            current += up;
        }
        count += 1;
    }
    println!("use the stairs");
}
