fn fibonacchi(n: u32) -> u32 {
    if n <= 1 {
        return n;
    }

    let a = fibonacchi(n - 1);
    let b = fibonacchi(n - 2);
    a + b
}

fn main() {
    println!("{}", fibonacchi(9));
}
