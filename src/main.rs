use shortest_path_algorithm_in_rust::takes_user_input;
use petgraph::algo::dijkstra;
use petgraph::prelude::*;

fn main() {
    println!("Welcome to the Shortest Path Algorithm in Rust in Kenya!");

    let mut graph: Graph<&str, u32, Undirected> = Graph::<&str, u32, Undirected>::new_undirected();

    // Add nodes to the graph
    let nairobi = graph.add_node("Nairobi");
    let kisumu = graph.add_node("Kisumu");
    let kisii = graph.add_node("Kisii");
    let nakuru = graph.add_node("Nakuru");
    let busia = graph.add_node("Busia");
    let kericho = graph.add_node("Kericho");

    // Add edges (distances)
    graph.extend_with_edges([
        (nairobi, kisumu, 100),
        (nairobi, kisii, 150),
        (kisumu, kisii, 50),
        (nairobi, kericho, 144),
        (kisumu, nakuru, 200),
        (kisii, nakuru, 100),
        (nakuru, busia, 300),
        (busia, kericho, 250),
    ]);

    loop {
        println!("\nChoose a route:");
        println!("1. Nairobi to Kericho");
        println!("2. Kisumu to Busia");
        println!("3. Kisii to Nakuru");
        println!("4. Nakuru to Kisumu");
        println!("5. Exit\n");

        let choice = takes_user_input();

        let (start_node, end_node) = match choice {
            1 => (nairobi, kericho),
            2 => (kisumu, busia),
            3 => (kisii, nakuru),
            4 => (nakuru, kisumu),
            5 => break,
            _ => {
                println!("Invalid choice!");
                continue;
            }
        };

        let node_map = dijkstra(&graph, start_node, Some(end_node), |e| *e.weight());

        if let Some(distance) = node_map.get(&end_node) {
            println!("Shortest distance is: {} km", distance);
        } else {
            println!("No path found.");
        }
    }
}
