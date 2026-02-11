fn maccarthy91(n: i32) -> i32 {
    println!("Calling M({}) ", n);

    if n > 100 {
        println!(" ->n >100 , returning {}", n - 10);
        n - 10
    } else {
        println!(" -> n < 100 , calling M(M ({}  + 11)}}", n);
        maccarthy91(maccarthy91(n + 11))
    }
}

fn main() {
    let n = 99;
    let result = maccarthy91(n);
    println!("\n Final result for M ({}) is {}", n, result);
}
