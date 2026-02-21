fn main() {
    let numbers = [-2i32, -1, 0, 1];

    let is_sorted = numbers.is_sorted_by(|n| n.abs());

    assert!(!is_sorted);

    println!("{} sorted by absolute value ", is_sorted);
}
