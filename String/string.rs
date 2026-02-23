fn main() {
    let string_slice: &str = "Hello , Rust !";

    let s1 = String::from(string_slice);

    let s2 = string_slice.to_string();

    let s3 = string_slice.to_owned();

    println!(" {} | {} | {}", s1, s2, s3);
}
