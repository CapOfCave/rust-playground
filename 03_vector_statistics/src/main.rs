use std::collections::HashMap;

fn main() {
    println!("Hello, world!");

    let mylist = vec![1, 2, 6, 4, 6, 11];

    println!("The mean is {}.", mean(&mylist));
    println!("The median is {}.", median(&mylist));
    println!("The mode is {}.", mode(&mylist));
}

fn mean(input: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in input {
        sum += i
    }
    sum / (input.len() as i32)
}

fn median(input: &Vec<i32>) -> i32 {
    let mut sorted = input.clone();
    sorted.sort();

    let lower = sorted[(sorted.len() - 1) / 2];
    let upper = sorted[sorted.len() / 2];
    (lower + upper) / 2
}

fn mode(input: &Vec<i32>) -> i32 {
    let mut occurences: HashMap<i32, i32> = HashMap::with_capacity(input.len());
    for i in input {
        let count = occurences.entry(*i).or_insert(0);
        *count += 1;
    }
    let mut max_key = -1;
    let mut max_value = i32::MIN;
    for (key, value) in occurences {
        if value > max_value {
            max_key = key;
            max_value = value;
        }
    }
    max_key
}
