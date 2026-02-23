fn main() {
    let s = "Hello world";

    for b in s.chars() {
        println!(" {}", b);
    }

    println!("");

    for (index, b) in s.char_indices() {
        println!(" {} {} ", b, index);
    }
}
