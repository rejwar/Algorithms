//takeuchi function

fn tak(x: i32, y: i32, z: i32) -> i32 {
    if x <= y {
        y
    } else {
        tak(tak(x - 1, y, z), tak(y - 1, z, x), tak(z - 1, x, y))
    }
}

fn main() {
    let result = tak(10, 5, 0);
    println!("tak(10 , 5 ,0) of result {}", result);
}
