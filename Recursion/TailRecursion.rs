// // fn factorial_tail(n: u64, acc: u64) -> u64 {
// //     if n == 0 {
// //         acc
// //     } else {
// //         factorial_tail(n - 1, n * acc)
// //     }
// // }

// // fn main() {
// //     factorial_tail(3, 5);
// // }

// fn factorial_tail(n: u64 , acc: u64) -> u64 {
//     if n == 0 {
//         acc
//     } else {
//         factorial_tail(n -1 , n * acc)
//     }
// }

// fn main() {

// }

// fn sum_n_tail(n: u64, acc: u64) -> u64 {
//     if n == 0 {
//         return acc;
//     }

//     sum_n_tail(n - 1, acc + n)
// }

// fn main() {
//     let n = 10;
//     println!("Sum of 1 to {}:  {}", n, sum_n_tail(n, 0));
// }

// fn sum_n_tail(n: u64, acc: u64) -> u64 {
//     if n == 0 {
//         return acc;
//     }

//     sum_n_tail(n - 1, acc + n)
// }

// fn main() {
//     let n = 10;
//     println!(" SUm of 1 to {} : {} ", n, sum_n_tail(n, 0));
// }

fn sum_n_tail(n: u64, acc: u64) -> u64 {
    if n == 0 {
        return acc;
    }

    sum_n_tail(n - 1, acc + n)
}

fn main() {
    let n = 10;
    println!(" Sum of 1 to {} : {} ", n, sum_n_tail(n, 0));
}
