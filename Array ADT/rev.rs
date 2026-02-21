fn main() {
    let a = [1 - 5];

    for num in a.iter().rev() {
        println!("{}", num);
    }

    println!("");

    println!("Original array: {:?}", a);
}
