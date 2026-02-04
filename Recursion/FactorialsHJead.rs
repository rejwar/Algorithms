fn Factorials(n: i32) -> i32 {
    if n == 1 {
        return 1;
    }

    let Result = Factorials(n - 1);
    n * Result
}

fn main() {
    Factorials(9);
}
