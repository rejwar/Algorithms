fn main() {
    let arr = [1, 2, 4, 5];
    let n = 5;

    let expected_sum = n * (n + 1) / 2;

    let actual_sum: i32 = arr.iter().sum();

    let missing_element = expected_sum - actual_sum;

    println!(" the missing number is {}", missing_element);
}
