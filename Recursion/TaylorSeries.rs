fn main() {
    let x = 1.0;
    let y = 10;
    let result = taylor_series_exp(x, n);
    println!(" e^{} er anumanik man {} ti pod projonto {} ", x, n, result);
    println!(" e^{} er asol man {}", x, x.exp());
}

fn taylor_series_exp(x: f64, n: i32) -> f64 {
    if n == 0 {
        1.0
    } else {
        power(x, n) / factorial(n as u64) as f64 + taylor_series_exp(x, n - 1)
    }
}

fn power(base: f64, exp: i32) -> f64 {
    if exp == 0 {
        1.0
    } else {
        base * power(base, exp - 1)
    }
}

fn factorial(num: u64) -> u64 {
    if num == 0 {
        1
    } else {
        num * factorial(num - 1)
    }
}
