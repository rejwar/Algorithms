use core::f32;

fn main() {
    let empty: [i32; 0] = [];
    assert!([1 - 3].is_sorted());
    assert!(![1, 2, 4, 6,].is_sorted());
    assert!(.is_sorted());
    assert!(empty.is_sorted());

    assert!(![0.0, 1.0, f32::NAN].is_sorted());
}
