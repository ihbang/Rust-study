use std::collections::HashMap;

fn median(l: &[i32]) -> i32 {
    let mut vec: Vec<i32> = l.into();

    vec.sort();
    vec[vec.len() / 2]
}

fn mode(l: &[i32]) -> i32 {
    let mut dic = HashMap::new();
    let mut most_freq = i32::MAX;
    let mut init = false;

    for i in l.iter() {
        let count = dic.entry(i).or_insert(0);
        *count += 1;
        if !init || *count > dic[&most_freq] {
            most_freq = *i;
            init = true;
        }
    }
    most_freq
}

fn pig_latin(s: String) -> String {
    let mut pig = String::new();
    let mut char = s.chars();
    let vowels = ['a', 'e', 'i', 'o', 'u'];

    match char.next() {
        None => (),
        Some(c) if vowels.contains(&c) => {
            pig = format!("{}-hay", s);
        }
        Some(first_char) => {
            while let Some(c) = char.next() {
                pig.push(c);
            }
            pig = format!("{}-{}ay", pig, first_char);
        }
    }
    pig
}

fn main() {
    let integers = [1, 3, 5, 7, 9, 11, 3, 8, 32, 7, 5, 12, 7];
    let apple = String::from("apple");
    let first = String::from("first");

    let median = median(&integers);
    let mode = mode(&integers);
    let piggy_apple = pig_latin(apple);
    let piggy_first = pig_latin(first);
    println!("median: {}", median);
    println!("mode:   {}", mode);
    println!("apple:  {}", piggy_apple);
    println!("fisrt:  {}", piggy_first);
}
