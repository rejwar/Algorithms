fn factorials(n: u64) -> u64 {
    if n == 0 {
        1
    } else {
        n * factorials(n - 1)
    }
}

fn main() {
    println!("Factorials of 5 is {}", factorials(6));
}
