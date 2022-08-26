/// Minimum Spanning Tree library

extern crate image;
extern crate imageproc;
use image::Rgb;
use image::RgbImage;
use imageproc::drawing::{draw_filled_rect_mut,draw_line_segment_mut};
use imageproc::rect::Rect;

use rand::Rng;

mod error;
use crate::error::Error;

// Table markers
const FMARK: f32 = f32::INFINITY;
const UMARK: usize = usize::MAX;

/// A 2D Vertex
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

/// A graph edge.
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

/// Vertex Table Item
#[derive(Copy, Clone)]
struct Item {
    /// Index is the position in the Vec
    index: usize,
    /// Index of the nearest Vertex
    near: usize,
    /// Distance to the nearest Vertex
    cost: f32,
    /// This Vertex
    vertex: Vertex,
}

impl Item {
    pub fn new(index: usize, vertex: Vertex) -> Self {
        Self { index, near: UMARK, cost: FMARK, vertex }
    }
}

/// Generate a set of N points within the rectangle bound by min_x, min_y,
/// max_x, max_y with a min_d minimum distance between points.
pub fn generate(n: i32, min_d: f32,
                min_x: i32, min_y: i32, max_x: i32, max_y: i32
    ) -> Result<Vec<Vertex>,Error> {

    println!("mst::generate");

    // Check for reasonable parameters
    assert!(n > 0);
    assert!(min_d > 1.0_f32);
    assert!(min_x < max_x);
    assert!(min_y < max_y);
    // Make sure there is enough room for all the points
    //assert!(max_x - min_x > n * min_d.ceil() as i32);
    //assert!(max_y - min_y > n * min_d.ceil() as i32);
    // TODO Improve to allow dense graphs but not impossible ones
    assert!(max_x - min_x > n * min_d.ceil() as i32 / 20);
    assert!(max_y - min_y > n * min_d.ceil() as i32 / 20);

    let mut points = Vec::new();
    let mut rng = rand::thread_rng();

    while points.len() < n as usize {
        let x = rng.gen_range(min_x..=max_x);
        let y = rng.gen_range(min_y..=max_y);
        let v = Vertex::new(x, y);
        if minimum_distance(&v, &points) >= min_d {
            points.push(v);
        }
    }

    Ok(points)
}

/// Find the minimum distance between a point and a set of points.
fn minimum_distance(v: &Vertex, points: &Vec<Vertex>) -> f32 {
    let mut min_d = FMARK;
    for p in points {
        let d = v.distance(&p);
        if d < min_d {
            min_d = d;
        }
    }
    min_d
}

/// Find the minimum spanning tree of a set of points.
/// Assumes a completely connected, bidirectional graph.
///
/// 1. Select a point and add it to the tree.
/// 2. Find the nearest point not in the tree to a point in the tree
///    and add it to the tree.
/// 3. Repeat #2 until all points are in the tree.
///
pub fn minimum_spanning_tree(points: &Vec<Vertex>) -> Result<Vec<Edge>,Error> {
    println!("mst::minimum_spanning_tree");

    let mut vertex_table = Vec::<Item>::new();
    let mut table_index = 0;

    // Initialize the vertex table from the list of points
    for p in points {
        vertex_table.push(Item::new(table_index, *p));
        table_index += 1;
    }
    // Verify table index is correct
    for i in 0..vertex_table.len() {
        assert_eq!(i, vertex_table[i].index);
    }

    // Put vertex 0 in the tree
    vertex_table[0].near = 0;
    vertex_table[0].cost = 0.0;

    while vertices_available(&vertex_table) {
        let mut tree_index = UMARK;
        let mut index = UMARK;
        let mut cost = FMARK;

        // Find the shortest distance between a vertex in the tree to a
        // vertex not in the tree.
        // Vertices in the tree have a valid 'near' value, not UMARK.
        for t in &vertex_table {
            // Skip vertices not in tree
            if t.near == UMARK {
                continue;
            }
            for i in &vertex_table {
                if i.near == UMARK {
                    // vertex 'v' is available (not in tree)
                    let length = t.vertex.distance(&i.vertex);
                    if length < cost {
                        tree_index = t.index;
                        index = i.index;
                        cost = length;
                    }
                }
            }
        }
        // Connect this point to the tree
        vertex_table[index].cost = cost;
        vertex_table[index].near = tree_index;
    }

    // Build the list of edges making up the spanning tree
    let mut edges = Vec::new();
    for i in 0..vertex_table.len() {
        // The first vertex connected to the tree has itself as its nearest
        // vertex so skip it
        if vertex_table[i].near == i {
            continue;
        }
        // Add a edge for each vertex to the nearest other vertex
        edges.push(Edge {
                u: vertex_table[i].vertex,
                v: vertex_table[vertex_table[i].near].vertex,
                length: vertex_table[i].cost });
    }

    Ok(edges)
}

/// Plot the graph and write to a PNG file.
pub fn plot(edges: &Vec<Edge>, output_file: &str) -> Result<(), std::io::Error>
{
    println!("mst::plot");

    let margin = 10;
    let (width, height, x0, y0) = plot_dimensions(edges, margin);
    //let mut image = ImageBuffer::new(width, height);
    let mut image = RgbImage::new(width as u32, height as u32);
    draw_filled_rect_mut(&mut image,
                         Rect::at(0, 0).of_size(width as u32, height as u32),
                         Rgb([255u8, 255u8, 255u8]));

    // draw edges
    for e in edges {
        draw_line_segment_mut(&mut image,
            ((e.u.x - x0) as f32, (e.u.y - y0) as f32),
            ((e.v.x - x0) as f32, (e.v.y - y0) as f32),
            Rgb([0u8, 0u8, 0u8])
        );
    }
    // draw vertices
    for e in edges {
        plot_vertex(&mut image, (e.u.x - x0) as u32, (e.u.y - y0) as u32);
        plot_vertex(&mut image, (e.v.x - x0) as u32, (e.v.y - y0) as u32);
    }

    image.save(output_file).unwrap();
    Ok(())
}

/// Find the width and height of the (square) area containing all vertices and
/// the lower left corner of that area with margins.
/// Returns: width, height, min_x, min_y
fn plot_dimensions(edges: &Vec<Edge>, margin: i32) -> (i32, i32, i32, i32) {
    let mut min_x = i32::MAX;
    let mut max_x = i32::MIN;
    let mut min_y = i32::MAX;
    let mut max_y = i32::MIN;

    for e in edges {
        if e.u.x < min_x {
            min_x = e.u.x;
        }
        if e.u.y < min_y {
            min_y = e.u.y;
        }
        if e.u.x > max_x {
            max_x = e.u.x;
        }
        if e.u.y > max_y {
            max_y = e.u.y;
        }

        if e.v.x < min_x {
            min_x = e.v.x;
        }
        if e.v.y < min_y {
            min_y = e.v.y;
        }
        if e.v.x > max_x {
            max_x = e.v.x;
        }
        if e.v.y > max_y {
            max_y = e.v.y;
        }
    }

    let w = max_x - min_x + 2 * margin;
    let h = max_y - min_y + 2 * margin;
    let d = i32::max(w, h);
    min_x = (min_x + max_x) / 2 - d / 2;
    min_y = (min_y + max_y) / 2 - d / 2;
    (d, d, min_x, min_y)
}

/// Draw an X centered on the given coordinates.
fn plot_vertex(image: &mut RgbImage, x: u32, y: u32) {
    //let color = [128u8, 0u8, 0u8];
    let color = [0u8, 128u8, 128u8];
    *image.get_pixel_mut(x - 1, y - 1) = Rgb(color);
    *image.get_pixel_mut(x - 1, y + 1) = Rgb(color);
    *image.get_pixel_mut(x, y) = Rgb(color);
    *image.get_pixel_mut(x + 1, y - 1) = Rgb(color);
    *image.get_pixel_mut(x + 1, y + 1) = Rgb(color);
}

/// Determine if any vertices in the table have not been added to the tree.
/// Available vertices don't have a nearest vertex (item.near == UMARK).
fn vertices_available(vertices: &Vec<Item>) -> bool {
    for v in vertices {
        if v.near == UMARK {
            return true;
        }
    }
    false
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
