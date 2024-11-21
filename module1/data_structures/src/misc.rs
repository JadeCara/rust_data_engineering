use rand::seq::SliceRandom;
use rand::thread_rng;
use std::cmp::Ord;
use std::collections::BinaryHeap;

// Fruit Salad binary heap example
// A binary heap implemented as a priority queue.
pub fn fruit_salad_binary_heap(n: usize) {
    let mut fruit_salad = std::collections::BinaryHeap::new();
    for _ in 0..n {
        let fruit = crate::util::generate_fruit();
        fruit_salad.push(fruit);
    }
    println!("Fruit Salad: {:?}", fruit_salad)
}

// Fruit enum with Fig as the highest priority
#[derive(Eq, PartialEq)]
enum Fruit {
    Fig,
    Other(String),
}

// We define Figs as the highest priority by implementing Ord
impl Ord for Fruit {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match (self, other) {
            (Fruit::Fig, Fruit::Fig) => std::cmp::Ordering::Equal,
            (Fruit::Fig, Fruit::Other(_)) => std::cmp::Ordering::Greater,
            (Fruit::Other(_), Fruit::Fig) => std::cmp::Ordering::Less,
            (Fruit::Other(_), Fruit::Other(_)) => std::cmp::Ordering::Equal,
        }
    }
}

impl PartialOrd for Fruit {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn generate_fruit_salad() -> BinaryHeap<Fruit> {
    let mut rng = thread_rng();
    let fruits = [
        "Apple", "Orange", "Pear", "Peach", "Banana", "Fig", "Fig", "Fig", "Fig",
    ];
    let mut fruit_salad = BinaryHeap::new();

    let mut figs_count = 0;
    while figs_count < 2 {
        let fruit = fruits.choose(&mut rng).unwrap();
        if *fruit == "Fig" {
            figs_count += 1;
            fruit_salad.push(Fruit::Fig);
        } else {
            fruit_salad.push(Fruit::Other(fruit.to_string()));
        }
    }

    fruit_salad
}

pub fn binary_heap_fig_salad() {
    let fruit_salad = generate_fruit_salad();
    println!("Random Fruit Salad With Two Servings of Figs:");
    for fruit in fruit_salad.into_sorted_vec() {
        match fruit {
            Fruit::Fig => println!("Fig"),
            Fruit::Other(fruit_name) => println!("{}", fruit_name),
        }
    }
}
