fn main() {
    let arr = [1, 2, 3, 4, 5];
    let mut m = arr.to_vec();

    m.push(10);
    m.push(30);

    println!(" {:?}", m);
}
