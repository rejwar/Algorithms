fn PrintReverse(s: &str, i: usize) {
    if i == s.len() {
        return;
    }

    PrintReverse(s, i + 1);
    println!("{}", s.as_bytes()[i] as char);
}

fn main() {
    PrintReverse("Rifat", 0);
}
