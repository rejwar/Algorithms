fn main() {
    let s = " Hi world";

    for c in s.chars() {
        println!(" the words {} ", c);
    }

    for b in s.bytes() {
        println!(" Byte {}", b);
    }

    for (index, c) in s.char_indices() {
        println!(" Index {} {}", index, c);
    }
}
