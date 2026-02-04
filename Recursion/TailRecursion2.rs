fn gcd(a: u64, b: u64) -> u64 {
    if b == 0 {
        return a;
    }
    gcd(b, a % b)
}
fn main() {
    println!(" GCD of 48 and 18 {}", gcd(48, 18));
}
