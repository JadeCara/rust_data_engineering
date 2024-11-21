extern crate rasciigraph;

use petgraph::algo::dijkstra;
use petgraph::prelude::*;
use rasciigraph::{plot, Config};

fn main() {
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    let belem_tower = graph.add_node("Belem Tower");
    let monastery = graph.add_node("Jerónimos Monastery");
    let lx_factory = graph.add_node("LX Factory");
    let commerce_square = graph.add_node("Commerce Square");
    let lisbon_cathedral = graph.add_node("Lisbon Cathedral");

    graph.extend_with_edges([
        (belem_tower, monastery, 1), // The distance from Belem Tower to Jerónimos Monastery is 1 km
        (belem_tower, lx_factory, 3), // The distance from Belem Tower to LX Factory is 3 km
        (belem_tower, commerce_square, 7), // The distance from Belem Tower to Commerce Square is 7 km
        (monastery, lx_factory, 3), // The distance from Jerónimos Monastery to LX Factory is 3 km
        (monastery, commerce_square, 6), // The distance from Jerónimos Monastery to Commerce Square is 6 km
        (lx_factory, commerce_square, 5), // The distance from LX Factory to Commerce Square is 5 km
        (commerce_square, lisbon_cathedral, 1), // The distance from Commerce Square to Lisbon Cathedral is 1 km
    ]);

    // Print the graph
    for edge in graph.edge_references() {
        let source = graph[edge.source()];
        let target = graph[edge.target()];
        let weight = edge.weight();
        println!("{} -> {} ({} km)", source, target, weight);
    }

    let node_map = dijkstra(&graph, belem_tower, Some(lisbon_cathedral), |e| *e.weight());

    if let Some(distance) = node_map.get(&lisbon_cathedral) {
        println!(
            "The shortest distance from Belem Tower to Lisbon Cathedral is {} km",
            distance
        );
    } else {
        println!("No route found from Belem Tower to Lisbon Cathedral.");
    }

    // vector of distances from Belem to every other node
    let mut b_distances = node_map
        .into_iter()
        .map(|(node, distance)| (graph[node], distance))
        .collect::<Vec<_>>();
    b_distances.sort_by(|a, b| a.1.partial_cmp(&b.1).unwrap());
    println!("{:?}", b_distances);

    // get just the distance values without the name with type f64 and sort them
    let mut b_distances = b_distances
        .into_iter()
        .map(|(_, distance)| distance as f64)
        .collect::<Vec<_>>();
    b_distances.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let res = plot(
        b_distances,
        Config::default()
            .with_height(10)
            .with_offset(10)
            .with_width(21)
            .with_caption("Distance from Belem Tower to other Lisbon landmarks (km)".to_string()),
    );
    print!("{}", res);
}
