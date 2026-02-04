use serde_json::value::Index;

fn BinarySearch(arr: &[i32], target: i32, left: isize, right: isize) -> Option<usize> {
    if left > right {
        return None;
    }
}

let mid = left + (right - left) / 2;

let mid_index = mid as usize;

if arr[mid_index] == target {
    Some(mid_index)
} else_if arr[mid_index] > target {
    BinarySearch(arr, target, left, mid-1)
} else {
    BinarySearch(arr, target, mid + 1, right)
}

fn main() {
    let arr = [ 1 ,3, 5, 7, 9 , 11];
    let target = 7;

    let result = BinarySearch(&arr, target, 0, (arr.len()-1) , as isize,);
    match result {
        Some(index) => println!("Found at index {}" , index),
        None => println!("Not found "),
    }
}