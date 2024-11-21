use std::collections::{BTreeMap, HashMap};

// Fruit Salad hash map example
// A hash map implemented with quadratic probing and SIMD lookup.
pub fn fruit_prices_hash_map() {
    let fruits = crate::util::fruits();
    let mut fruit_prices = HashMap::new();
    for fruit in fruits {
        fruit_prices.insert(fruit, rand::random::<f64>());
    }
    println!("Fruit Salad Prices: {:?}", fruit_prices)
}

// Fruit Salad BTree map example
// An ordered map implemented with a B-Tree.
pub fn fruit_prices_b_tree_map() {
    let fruits = crate::util::fruits();
    let mut fruit_prices = BTreeMap::new();
    for fruit in fruits {
        fruit_prices.insert(fruit, rand::random::<f64>());
    }
    println!("Fruit Salad Prices: {:?}", fruit_prices)
}
