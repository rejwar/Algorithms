fn SumOfDigits(n: u32) -> u32 {
    if n == 0 {
        0
    } else {
        (n % 10) + SumOfDigits(n / 10)
    }
}

fn main() {
    let n = 1234;
    println!("Sum Of digits {} is {}", n, SumOfDigits(n));
}
