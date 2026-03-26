fn main() {
    let number: i128 = 40;
    println!("Fibonacci sequence of: {number}");
    let result = fibonacci(number);
    println!("The {number}th Fibonacci number is: {result}");

}

fn fibonacci(n: i128) -> i128 {
    if n < 0 {
        panic!("Negative numbers are not allowed in Fibonacci sequence");
    } else if n <= 1 {
        n
    } else {
        fibonacci(n - 1) + fibonacci(n - 2)
    }
}