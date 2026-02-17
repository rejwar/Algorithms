fn linear_search<T: PartialEq>(arr: &[T], target: T) -> Option<usize> {
    for (index, item) in arr.iter().enumerate() {
        if item == &target {
            return Some(index);
        }
    }

    None
}

fn main() {
    let names = ["Rahmin ", "Karim ", "SUMI"];
    let target = "SUMI";

    match linear_search(&names, target) {
        Some(index) => println!(" GOt this name {} in this index {}", target, index),
        None => println!("Sorry we didn't get this number "),
    }
}
