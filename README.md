# MST in Rust

This project is a reimplementation of `github.com/crithead/mst` in Rust.

This project is an example implementation of Prim's algorithm for finding
a __minimum spanning tree__ of a completely connected, undirected graph. The
graph is a set of points where there is an edge from every vertex to every
other vertex. The goal is to find a set of edges that connect all points
with minimum length of the resulting edges. The weight of each edge is the
distance between vertices.

## Build

```shell
git clone https://github.com/crithead/mst-rs
cd mst-rs
cargo build
cargo doc --no-deps [ --open ]
```

## Demo

The `demo` tool combines the functions of _data_, _calc_, and _plot_ into a
single program that prints the minimum spanning tree of a set of random points
in a plane.  Takes input from the command line or a configuration file and
writes an image of the resulting minimum spanning tree.

```shell
cargo run
cargo run -- --help
data | calc | plot      # Final image in 'mst.png'
data -v -o data.csv && calc -v -i data.csv -o tree.csv && plot -v -i tree.csv -o mst.png
```

## Data

The `data` tool generates a set of points in a plane.
Writes the set of points to an output file or the console.

```shell
cargo run --bin data -o data.csv
cargo run --bin data > data.csv
```

## Calc

The `calc` tool calculates the _minimum spanning tree_ of a set of points in a
plane.  Reads the set of points from the input file and writes the set of edges
making the MST to the output file.

```shell
cargo run --bin calc -- -i data.csv -o graph.csv
cargo run --bin calc -- < data.csv > graph.csv
```

## Plot

The `plot` tool generates a graph of the _minimum spanning tree_.
Reads the set of edges from the input file and writes the resulting graph to
and the output file as a PNG.

```shell
cargo run --bin plot -- -i tree.csv -o mst.png
cargo run --bin plot -- -v -o mst.png < tree.csv
```

# To Do List

- `mst`
  - Move Options from `demo` to lib
  - Read options from a configuration file
  - Add parameters for point generation
  - Add parameters to set plot colors
  - More unit tests
- `demo`
  - Use configuration and options from lib
- `data`
  - Use configuration and options from lib
- `calc`
- `plot`
  - Use configuration and options from lib

