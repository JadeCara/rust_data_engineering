/*
This code defines a function called create_fruit_salad
that takes a mutable vector of strings as input and returns
a new vector of strings that contains the same elements as the input vector,
but in a random order.
*/

use rand::seq::SliceRandom;
use rand::thread_rng;

pub fn create_fruit_salad(mut fruits: Vec<String>) -> Vec<String> {
    let mut rng = thread_rng();
    fruits.shuffle(&mut rng);

    fruits
}

pub fn display_fruit_salad(fruits: Vec<String>) {
    println!("Your fruit salad contains:");
    for fruit in fruits {
        println!("{}", fruit);
    }
}

// Examples of mutability/immutability in Rust

pub fn fruit_salad_vec() -> Vec<&'static str> {
    vec!["apple", "banana", "cherry", "dates", "elderberries"]
}

pub fn check_and_add_fruit(mut fruit_salad: Vec<&str>) -> Result<Vec<&str>, &'static str> {
    fruit_salad.push("figs");
    Ok(fruit_salad)
}

pub fn mutability_examples() {
    // immutable vector
    let fruit_salad_immutable = fruit_salad_vec();
    println!(
        "Original immutable fruit salad: {:?}",
        fruit_salad_immutable
    );

    // To mutate the vector, we need to declare it as mutable:
    let mut fruit_salad_mutable = fruit_salad_vec();
    fruit_salad_mutable.push("figs");
    println!("Modified mutable fruit salad: {:?}", fruit_salad_mutable);

    // Clone immutable to mutable and add figs by changing ownership.
    let result = check_and_add_fruit(fruit_salad_immutable.clone());
    match result {
        Ok(mutated_fruit_salad) => println!(
            "Added figs to the cloned fruit salad: {:?}",
            mutated_fruit_salad
        ),
        Err(err) => println!("Error: {}", err),
    }
    println!(
        "Original immutable fruit salad: {:?}",
        fruit_salad_immutable
    );
}
