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

pub fn fruit_salad_vec() -> Vec<&'static str> {
    vec!["apple", "banana", "cherry", "dates", "elderberries"]
}

pub fn check_and_add_fruit(mut fruit_salad: Vec<&str>) -> Result<Vec<&str>, &'static str> {
    fruit_salad.push("figs");
    Ok(fruit_salad)
}