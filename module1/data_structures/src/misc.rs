// Fruit Salad binary heap example
// A binary heap implemented as a priority queue.
pub fn fruit_salad_binary_heap(n: usize, fruits: Vec<&'static str>) {
    let mut fruit_salad = std::collections::BinaryHeap::new();
    for _ in 0..n {
        let fruit = fruits[rand::random::<usize>() % fruits.len()];
        fruit_salad.push(fruit);
    }
    println!("Fruit Salad: {:?}", fruit_salad)
}
