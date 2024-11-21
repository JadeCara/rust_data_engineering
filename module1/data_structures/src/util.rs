use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn fruits() -> Vec<&'static str> {
    vec![
        "apple",
        "banana",
        "cherry",
        "date",
        "elderberry",
        "fig",
        "grape",
    ]
}

pub fn generate_fruit() -> &'static str {
    let fruits = fruits();
    let mut rng = thread_rng();
    fruits.choose(&mut rng).unwrap()
}
