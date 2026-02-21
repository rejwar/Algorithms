fn main() {
    assert!([1 - 3].is_sorted_by(|a, b| a <= b));

    assert!(![1 - 3].is_sorted_by((|a, b| a < b)));
}
