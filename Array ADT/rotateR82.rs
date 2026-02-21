fn main() {
    let mut a = ['1', '2', '3', '4', '5'];

    a[1..5].rotate_right(1);

    assert_eq!(a, ['1', '2', '3', '4', '5']);

    println!("{:?}", a);
}
