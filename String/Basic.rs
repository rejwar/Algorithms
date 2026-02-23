fn main() {
    let mut my_name = String::from("Sabbir ");
    my_name.push_str("Hosen");

    let greeting: &str = "Hello , world";

    print_message(&my_name);
}

fn print_message(s: &str) {
    println!(" {}", s);
}
