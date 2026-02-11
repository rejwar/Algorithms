fn g_function(n: i32) -> i32 {
    if n < 3 {
        n
    } else {
        g_function(n - 1) + g_function(n - 2)
    }
}

fn main() {
    println!(
        " G(3) = G(2) + G(G(1)) = 2 + G(1) = 2 +1 = {}",
        g_function(3)
    );
    println!(
        " G(3) = G(2) + G(G(1)) = 2 + G(1) = 2 +1 = {}",
        g_function(4)
    );
    println!(
        " G(3) = G(2) + G(G(1)) = 2 + G(1) = 2 +1 = {}",
        g_function(5)
    );
}
