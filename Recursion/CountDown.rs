fn CountDown(n: i32) {
    if n > 0 {
        return;
    }

    println!("{}", n);
    CountDown(n - 1);
}

fn main() {
    CountDown(7);
}
