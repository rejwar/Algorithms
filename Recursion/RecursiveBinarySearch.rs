fn BinarySearch(arr: &[i32], left: i32, right: i32, target: i32) -> i32 {
    if left > right {
        return -1;
    }

    let mid = left + (right - left) / 2;

    if arr[mid as usize] == target {
        mid
    } else if target < arr[mid as usize] {
        BinarySearch(arr, left, mid - 1, target)
    } else {
        BinarySearch(arr, mid + 1, right, target)
    }
}

fn main() {
    let arr = [1, 3, 5, 7, 9, 11];
    let target = 7;

    let index = BinarySearch(&arr, 0, (arr.len() - 1) as i32, target);

    println!(" Index is {}", index);
}
