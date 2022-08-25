# MST in Rust

This project is a reimplementation of `github.com/crithead/mst` in Rust.

This project is an example implementations of Prim's algorithm for finding
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
```

## Data

The `data` tool generates a set of points in a plane.

```shell
cargo run --bin=data -- -h
```

## Calc

The `calc` tool calculates the _minimum spanning tree_ of a set of points in a
plane.

```shell
cargo run --bin=calc -- -h
```

## Plot

The `plot` tool generates a graph of the _minimum spanning tree_.

```shell
cargo run --bin=plot -- -h
```

## Demo

The `demo` tool combines the functions of _data_, _calc_, and _plot_ into a
single program that prints a the minimum spanning tree of a set of random points
in a plane.

```shell
cargo run
cargo run -- --help
```

