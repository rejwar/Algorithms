fn main() {
    let s1 = String::from("Hello ,");
    let s2 = String::from("World");

    let result = s1 + &s2;

    let s3 = String::from("Rust");

    let s4 = String::from("Programming");

    let resutl2 = format!("{} {} ", s3, s4);

    let mut s5 = String::from("Learning");
    s5.push_str("Rust");
}
