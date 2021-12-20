use std::collections::HashMap{self, Entry};

fn main() {
    println!("Hello, world!");

    let mylist = vec![1, 2, 8, 4, 6, 9];

    println!("The mean is {}.", mean(&mylist));
    println!("The median is {}.", median(&mylist));
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