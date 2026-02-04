fn printNumbers(n: i32) {
    if n == 0 {
        return;
    }

    printNumbers(n - 1);
    println!("{}", n);
}

fn main() {
    printNumbers(5);
}
