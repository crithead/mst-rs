/// MST Demo
/// Generates a random set of points in a plane, calculates the minumum
/// spanning tree from those points, and plots the result.
///
/// demo -n N -m M -x x0,x1 -y y0,y1 -o output_file -c config_file
///
/// Read the values from the configuration file, then from the command line
/// with command line values overriding configuration file values.

use mst::{generate,minimum_spanning_tree,plot};

fn main() {
    println!("MST Demo");
    // TODO Get graph parameters from command line arguments or a file.

    let num_points = 10;
    let min_dist = 4.0;
    let points = generate(num_points, min_dist, 0, 0, 100, 100);
    let tree = minimum_spanning_tree(&points);
    plot(&tree, "mst-demo.png");
}
