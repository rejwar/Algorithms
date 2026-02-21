fn main() {
    let words = vec!["Hello ! world".to_string()];

    for word in words.into_iter() {
        println!(" {}", word);
    }

    // println!("{:?}", words);
}
