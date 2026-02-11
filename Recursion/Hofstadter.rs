fn female(n: i32) -> i32 {
    if n == 0 {
        1
    } else {
        n - male(female(n - 1))
    }
}

fn male(n: i32) -> i32 {
    if n == 0 {
        0
    } else {
        n - female(male(n - 1))
    }
}

fn main() {
    println!(" Hofstader Female Sequence ");
    for i in 0..10 {
        println!("{}", female(i));
    }

    println!(" \n Hofstader male Sequence ");
    for i in 0..10 {
        println!(" {}", male(i));
    }
}
