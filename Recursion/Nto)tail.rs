fn SumTail(n: i32, acc: i32) -> i32 {
    if n == 0 {
        return acc;
    }

    SumTail(n - 1, acc + n)
}

fn main() {
    let n = 5;

    let result = SumTail(n, 0);

    println!(" Sum = {}", result);
}
