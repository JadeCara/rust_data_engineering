// Use sequences module
mod maps;
mod misc;
mod sequences;
mod sets;

fn _fruits() -> Vec<&'static str> {
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

fn main() {
    println!("Common Rust Collections:");
    let fruits = _fruits();
    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    sequences::fruit_salad_sequence(5, fruits.clone());
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    sequences::fruit_salad_deque(5, fruits.clone());
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");
    sequences::fruit_salad_linked_list(5, fruits.clone());

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    maps::fruit_prices_hash_map(fruits.clone());
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");
    maps::fruit_prices_b_tree_map(fruits.clone());

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    sets::fruit_salad_hash_set(5, fruits.clone());
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");
    sets::fruit_salad_b_tree_set(5, fruits.clone());

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
    misc::fruit_salad_binary_heap(5, fruits.clone());
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruit_salad_sequence() {
        sequences::fruit_salad_sequence(5, _fruits());
    }

    #[test]
    fn test_fruit_salad_deque() {
        sequences::fruit_salad_deque(5, _fruits());
    }

    #[test]
    fn test_fruit_salad_linked_list() {
        sequences::fruit_salad_linked_list(5, _fruits());
    }

    #[test]
    fn test_fruit_prices_hash_map() {
        maps::fruit_prices_hash_map(_fruits());
    }

    #[test]
    fn test_fruit_prices_b_tree_map() {
        maps::fruit_prices_b_tree_map(_fruits());
    }

    #[test]
    fn test_fruit_salad_hash_set() {
        sets::fruit_salad_hash_set(5, _fruits());
    }

    #[test]
    fn test_fruit_salad_b_tree_set() {
        sets::fruit_salad_b_tree_set(5, _fruits());
    }

    #[test]
    fn test_fruit_salad_binary_heap() {
        misc::fruit_salad_binary_heap(5, _fruits());
    }
}
