// Problem 1: Will the following Rust program compile? If not, how can you correct
// it?
// fn main() {
//     let x = 10 as f64;
//     let y = 5.0;
//     println!("x + y = {}", x+y);
// }

// Answer: It won't due to x and y have different types. i32 and f64. Convert x to type f64 will solve the problem.

// Problem 2: Let us say you are given an array of four floating-point values. You
// have to implement the function find_mean() used in the following
// code snippet:

fn main() {
    let arr = [1.0, 2.0, 3.0, 4.0];
    println!("mean of elements of arr = {}", find_mean(&arr));
}

fn find_mean(arr: &[f64]) -> f64 {
    let len = arr.iter().len() as i32;
    if len % 2 == 0 {
        return (arr[(len/2) as usize] + arr[(len/2 - 1) as usize]) / 2.0;
    }

    arr[(len / 2) as usize]
}
