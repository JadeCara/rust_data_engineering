use std::collections::{LinkedList, VecDeque};
// Module: sequences

// Fruit salad sequence example
// 	A contiguous growable array type, written Vec<T> but pronounced 'vector'.
pub fn fruit_salad_sequence(fruit_number: usize) {
    let mut fruit_salad = Vec::new();
    // Add fruits to the fruit salad in random order
    for _ in 0..fruit_number {
        let fruit = crate::util::generate_fruit();
        println!("Fruit: {}", fruit);
        fruit_salad.push(fruit);
    }
    println!("Fruit Salad: {:?}", fruit_salad);
}

// Fruit salad deque example
// 	A double-ended queue implemented with a growable ring buffer.
pub fn fruit_salad_deque(fruit_number: usize) {
    let mut fruit_salad = VecDeque::new();
    // Add fruits to the fruit salad in random order
    for _ in 0..fruit_number {
        let fruit = crate::util::generate_fruit();
        if rand::random() {
            println!("Adding {} to front", fruit);
            fruit_salad.push_front(fruit);
        } else {
            println!("Adding {} to back", fruit);
            fruit_salad.push_back(fruit);
        }
    }
    println!("Fruit Salad: {:?}", fruit_salad);
}

// Fruit salad linked list example
// 	A doubly-linked list with owned nodes.
pub fn fruit_salad_linked_list(fruit_number: usize) {
    let mut fruit_salad = LinkedList::new();
    // Add fruits to the fruit salad in random order
    for _ in 0..fruit_number {
        let fruit = crate::util::generate_fruit();
        if rand::random() {
            println!("Adding {} to front", fruit);
            fruit_salad.push_front(fruit);
        } else {
            println!("Adding {} to back", fruit);
            fruit_salad.push_back(fruit);
        }
    }
    println!("Fruit Salad: {:?}", fruit_salad);
}
