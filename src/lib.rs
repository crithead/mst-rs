/// Minimum Spanning Tree library

use rand::Rng;

#[derive(Copy, Clone)]
pub struct Vertex {
    pub x: i32,
    pub y: i32,
}

impl Vertex {
    pub fn new(x: i32, y: i32) -> Self {
        Self { x, y }
    }

    pub fn distance(&self, v: &Vertex) -> f32 {
        (((self.x - v.x) * (self.x - v.x) +
          (self.y - v.y) * (self.y - v.y)) as f32).sqrt()
    }
}

#[derive(Copy, Clone)]
pub struct Edge {
    pub u: Vertex,
    pub v: Vertex,
    length: f32,
}

impl Edge {
    /// Create a new Edge from two coordinate pairs.
    pub fn new(x0: i32, y0: i32, x1: i32, y1: i32) -> Self {
        assert!(x0 != x1);
        assert!(y0 != y1);
        let u = Vertex::new(x0, y0);
        let v = Vertex::new(x1, y1);
        let length = u.distance(&v);

        Self { u, v, length }
    }

    /// Create a new Edge from a pairs of distinct Vertices.
    pub fn from_vertices(u: Vertex, v: Vertex) -> Self {
        assert!(u.x != v.x);
        assert!(u.y != v.y);
        Self { u, v, length: u.distance(&v) }
    }

    /// Get the length of the Edge.
    pub fn len(&self) -> f32 {
        self.length
    }
}


/// Generate a set of N points within the rectangle bound by min_x, min_y,
/// max_x, max_y with a min_d minimum distance between points.
// TODO Convert return value to Result<Vec<Vertex>, std::?::Error>
pub fn generate(n: i32, min_d: f32,
        min_x: i32, min_y: i32, max_x: i32, max_y: i32) -> Vec<Vertex> {
    println!("mst::generate");

    // Check for reasonable parameters
    assert!(n > 0);
    assert!(min_d > 1.0_f32);
    assert!(min_x < max_x);
    assert!(min_y < max_y);
    // Make sure there is enough room for all the points
    assert!(max_x - min_x > n * min_d.ceil() as i32);
    assert!(max_y - min_y > n * min_d.ceil() as i32);

    let mut points = Vec::new();
    let mut rng = rand::thread_rng();

    //while points.len() < n.try_into().unwrap() {
    while points.len() < n as usize {
        let x = rng.gen_range(min_x..=max_x);
        let y = rng.gen_range(min_y..=max_y);
        let v = Vertex::new(x, y);
        if minimum_distance(&v, &points) >= min_d {
            points.push(v);
        }
    }

    points
}

/// Find the minimum distance between a point and a set of points.
fn minimum_distance(v: &Vertex, points: &Vec<Vertex>) -> f32 {
    let mut min_d = f32::MAX;
    for p in points {
        let d = v.distance(&p);
        if d < min_d {
            min_d = d;
        }
    }
    min_d
}

// TODO Convert return value to Result<Vec<Edge>, std::?::Error>
/// Find the Minimum Spanning Tree for a set of points.
pub fn minimum_spanning_tree(points: &Vec<Vertex>) -> Vec<Edge> {
    println!("mst::minimum_spanning_tree");
    let edges = Vec::new();

    // TODO Apply Prim's minimum spanning tree algorithm to the set of points
    //      to produce a set pf edges.

    edges
}

/// Plot the graph and write to a PNG file.
pub fn plot(graph: &Vec<Edge>, output_file: &str) { // -> Result
    println!("mst::plot");
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_vertex_new() {
        let v = Vertex::new(0, 0);
        assert_eq!(v.x, 0);
        assert_eq!(v.y, 0);
    }
    #[test]
    fn test_edge_new() {
        let e = Edge::new(0, 0, 1, 1);
        assert_eq!(e.u.x, 0);
        assert_eq!(e.u.y, 0);
        assert_eq!(e.v.x, 1);
        assert_eq!(e.v.y, 1);
    }
}
