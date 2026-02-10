fn ReverseNUmber(n: u32, rev: u32) -> u32 {
    if n == 0 {
        rev
    } else {
        ReverseNUmber(n / 10, rev * 1 + n % 10)
    }
}

fn main() {
    let n = 1234;
    let reversed = ReverseNUmber(n, 0);

    println!(" Reversed number is {}", reversed);
}
