fn main() {
    let matrix = [[1, 2, 3], [4, 5, 6]];

    for row in matrix.iter() {
        for item in row.iter() {
            print!("{}", item);
        }

        println!();
    }
}
