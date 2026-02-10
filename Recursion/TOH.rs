fn tower_of_hanoi(n: i32, from: char, to: char, aux: char) {
    if n == 1 {
        println!(" Move disk 1 from {} to {}", from, to);
        return;
    }

    tower_of_hanoi(n - 1, from, aux, to);

    println!(" Move disk {} from  {} to {}", n, from, to);

    tower_of_hanoi(n - 1, aux, to, from);
}

fn main() {
    let n = 3;
    tower_of_hanoi(n, 'A', 'C', 'B');
}
