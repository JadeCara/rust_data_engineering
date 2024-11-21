// Fruit Salad hash set example
// A hash set implemented as a HashMap where the value is ().

pub fn fruit_salad_hash_set(n: usize) {
    println!("Generating {} random fruits...", n);
    let mut fruit_salad = std::collections::HashSet::new();
    for _ in 0..n {
        let fruit = crate::util::generate_fruit();
        fruit_salad.insert(fruit);
    }
    println!("Fruit Salad: {:?}", fruit_salad)
}

// Fruit Salad BTree set example
//An ordered set based on a B-Tree.
pub fn fruit_salad_b_tree_set(n: usize) {
    let mut fruit_salad = std::collections::BTreeSet::new();
    for _ in 0..n {
        let fruit = crate::util::generate_fruit();
        fruit_salad.insert(fruit);
    }
    println!("Fruit Salad: {:?}", fruit_salad)
}
