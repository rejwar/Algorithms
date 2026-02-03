static mut SUM: i32 = 0;

unsafe fn reverse(n: i32) {
    if n == 0 {
        return;
    }

    let rem = n % 10;

    unsafe {
        SUM = SUM * 10 + rem;
    }

    reverse(n / 10);
}

fn main() {
    unsafe {
        reverse(1234);
        println!("{}", SUM);
    }
}
