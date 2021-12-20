fn main() {
    println!("Hello, world!");

    let mylist = vec![1, 2, 8, 4, 5];

    let mean = mean(&mylist);

    println!("The mean of these {} elements is {}.", mylist.len(), mean);
}

fn mean(input: &Vec<i32>) -> i32 {
    let mut sum = 0;
    for i in input {
        sum += i
    }
    sum / (input.len() as i32)
}