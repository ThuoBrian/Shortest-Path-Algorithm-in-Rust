// Import the necessary modules from the petgraph crate
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() {
    let mut graph = Graph::<&str, u32, Undirected>::new_undirected();

    // Add nodes to the graph
    let nairobi = graph.add_node("Nairobi");
    let kisumu = graph.add_node("Kisumu");
    let kisii = graph.add_node("Kisii");
    let nakuru = graph.add_node("nakuru");
    let busia = graph.add_node("Busia");
    let kericho = graph.add_node("Kericho");

    // Add edges with weights to the graph
    graph.extend_with_edges(&[
        (nairobi, kisumu, 100),
        (nairobi, kisii, 150),
        (kisumu, kisii, 50),
        (nairobi, kericho, 144),
        (kisumu, nakuru, 200),
        (kisii, nakuru, 100),
        (nakuru, busia, 300),
        (busia, kericho, 250),
    ]);


    let node_map = dijkstra(&graph, nairobi, Some(busia), |e| *e.weight());

    if let Some(distance) = node_map.get(&kericho) {
        println!(
            "\nShortest distance from Nairobi to Kericho is: {}\n",
            distance
        );
    } else {
        println!("\nNo path found from Nairobi to Busia.\n");
    }
}
