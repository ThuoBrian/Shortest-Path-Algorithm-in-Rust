# 🚀 Shortest Path Algorithm in Rust (Kenyan Towns)

This is a simple Rust CLI program that uses **Dijkstra’s Algorithm** to compute the shortest path between towns in Kenya. It demonstrates how to use the [`petgraph`](https://docs.rs/petgraph) crate for graph data structures and algorithms.

---

## 🛠 Features

- Models Kenyan towns and routes as a weighted **undirected graph**
- Uses **Dijkstra’s algorithm** to find the shortest distance between two towns
- Command-line interface for selecting routes
- Clean and interactive output

---

## 📌 Towns & Routes

The graph includes the following towns and connections:

| From      | To       | Distance (km) |
|-----------|----------|----------------|
| Nairobi   | Kisumu   | 100           |
| Nairobi   | Kisii    | 150           |
| Nairobi   | Kericho  | 144           |
| Kisumu    | Kisii    | 50            |
| Kisumu    | Nakuru   | 200           |
| Kisii     | Nakuru   | 100           |
| Nakuru    | Busia    | 300           |
| Busia     | Kericho  | 250           |

---

## 📦 Dependencies

- [`petgraph`](https://crates.io/crates/petgraph) – for graph creation and Dijkstra’s algorithm
- Custom function: `takes_user_input()` for reading user input

---

## ▶️ Running the App

1. **Clone the repo**:
   ```bash
   git clone https://github.com/your-username/shortest-path-rust.git
   cd shortest-path-rust
