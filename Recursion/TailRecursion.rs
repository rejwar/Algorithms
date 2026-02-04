fn factorial_tail(n: u64, acc: u64) -> u64 {
    if n == 0 {
        acc
    } else {
        factorial_tail(n - 1, n * acc)
    }
}

fn main() {
    factorial_tail(3, 5);
}
