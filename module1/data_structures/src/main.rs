// Run with `cargo run -- --number 5`

// Use sequences module
mod cli_salad;
mod maps;
mod misc;
mod sequences;
mod sets;
mod util;

use clap::Parser;

#[derive(Parser)]
#[clap(
    version = "1.0",
    author = "Jade <your.email@example.com>",
    about = "Number of fruits to include in the salad"
)]
struct Opts {
    #[clap(short, long)]
    number: usize,
}

fn main() {
    let opts: Opts = Opts::parse();

    // Get the number of fruits the user requested
    let num_fruits = opts.number;

    // Print the fruit salad in human readable format with a count of fruits used
    println!(
        "Created Fruit salad with {} fruits: {:?}",
        num_fruits,
        cli_salad::create_fruit_salad(num_fruits)
    );

    // Generate a fruit salad with two servings of figs (binary heap)
    misc::binary_heap_fig_salad();
    info(num_fruits);
}

fn info(num_fruits: usize) {
    println!("Common Rust Collections:");

    // Sequences
    println!("\n\tSequences:");
    println!("\t\tVec: https://doc.rust-lang.org/std/vec/struct.Vec.html");
    sequences::fruit_salad_sequence(num_fruits);
    println!("\t\tVecDeque: https://doc.rust-lang.org/std/collections/struct.VecDeque.html");
    sequences::fruit_salad_deque(num_fruits);
    println!("\t\tLinkedList: https://doc.rust-lang.org/std/collections/struct.LinkedList.html");
    sequences::fruit_salad_linked_list(num_fruits);

    // Maps
    println!("\n\tMaps:");
    println!("\t\tHashMap: https://doc.rust-lang.org/std/collections/struct.HashMap.html");
    maps::fruit_prices_hash_map();
    println!("\t\tBTreeMap: https://doc.rust-lang.org/std/collections/struct.BTreeMap.html");
    maps::fruit_prices_b_tree_map();

    // Sets
    println!("\n\tSets:");
    println!("\t\tHashSet: https://doc.rust-lang.org/std/collections/struct.HashSet.html");
    sets::fruit_salad_hash_set(num_fruits);
    println!("\t\tBTreeSet: https://doc.rust-lang.org/std/collections/struct.BTreeSet.html");
    sets::fruit_salad_b_tree_set(num_fruits);

    // Misc
    println!("\n\tMisc:");
    println!("\t\tBinaryHeap: https://doc.rust-lang.org/std/collections/struct.BinaryHeap.html");
    misc::fruit_salad_binary_heap(num_fruits);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fruit_salad() {
        assert_eq!(cli_salad::create_fruit_salad(5).len(), 5);
    }

    #[test]
    fn test_fruit_salad_sequence() {
        sequences::fruit_salad_sequence(5, util::fruits());
    }

    #[test]
    fn test_fruit_salad_deque() {
        sequences::fruit_salad_deque(5, util::fruits());
    }

    #[test]
    fn test_fruit_salad_linked_list() {
        sequences::fruit_salad_linked_list(5, util::fruits());
    }

    #[test]
    fn test_fruit_prices_hash_map() {
        maps::fruit_prices_hash_map(util::fruits());
    }

    #[test]
    fn test_fruit_prices_b_tree_map() {
        maps::fruit_prices_b_tree_map(util::fruits());
    }

    #[test]
    fn test_fruit_salad_hash_set() {
        sets::fruit_salad_hash_set(5, util::fruits());
    }

    #[test]
    fn test_fruit_salad_b_tree_set() {
        sets::fruit_salad_b_tree_set(5, util::fruits());
    }

    #[test]
    fn test_fruit_salad_binary_heap() {
        misc::fruit_salad_binary_heap(5, util::fruits());
    }
}
