fn main() {
    let number: i32 = 45;
    let time = std::time::Instant::now();
    println!("Fibonacci sequence of: {number}");
    let result = fibonacci(number);
    let elapsed = time.elapsed();
    println!("The {number}th Fibonacci number is: {result}");
    println!("Time taken: {:.5?}", elapsed);
}

fn fibonacci(n: i32) -> i32 {
    if n < 0 {
        panic!("Negative numbers are not allowed in Fibonacci sequence");
    } else if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}