use std::io;

fn main() {
    let names = ["rahim", "kareem", "sumi", "Sex"];

    println!("Type the name that u want to find out : ");

    let mut input = String::new();

    io::stdin()
        .read_line(&mut input)
        .expect("Getting issue to take input");

    let target = input.trim().to_lowercase();

    let mut found = false;
    for (index, &name) in names.iter().enumerate() {
        if name == target {
            println!("{} is got in index no {}", target, index);
            found = true;
            break;
        }
    }

    if !found {
        println!("Sorry not in this list {}", target);
    }
}
