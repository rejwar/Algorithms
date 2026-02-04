fn BinaryToDecimal(n: i32) -> i32 {
    if n == 0 {
        return 0;
    }

    let last = n % 10;
    let rest = BinaryToDecimal(n / 10);
    rest * 2 + last
}

fn main() {
    println!("{}", BinaryToDecimal(1011));
}
