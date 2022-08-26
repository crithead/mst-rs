/// MST Demo
/// Generates a random set of points in a plane, calculates the minumum
/// spanning tree from those points, and plots the result.
///
/// demo -n N -m M -x x0,x1 -y y0,y1 -o output_file -c config_file
///
/// Read the values from the configuration file, then from the command line
/// with command line values overriding configuration file values.

use std::rc::Rc;
use mst::{generate,minimum_spanning_tree,plot,Edge,Vertex};

#[derive(Clone)]
struct Options {
    /// Print extra messages
    verbose: bool,
    /// Only print error messages
    quiet: bool,
    /// The output file name
    output: Rc<String>,
}

fn main() {
    // TODO Get graph parameters from command line arguments or a file.
    let opts = get_options();

    if ! opts.quiet {
        println!("MST Demo");
    }

    let num_points = 1000;
    let min_dist = 4.0;
    let points = match generate(num_points, min_dist, 0, 0, 1000, 1000) {
        Ok(points) => points,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    if opts.verbose {
        vprint(&points);
    }

    let tree = match minimum_spanning_tree(&points) {
        Ok(tree) => tree,
        Err(e) => {
            println!("{}", e);
            std::process::exit(1);
        }
    };

    if opts.verbose {
        eprint(&tree);
    }

    plot(&tree, &opts.output).expect("write image failed");
}

/// Get program options
fn get_options() -> Options {
    Options {
        quiet: false,
        verbose: true,
        output: Rc::new("demo.png".to_string()),
    }
}

/// Print edges
fn eprint(edges: &Vec<Edge>) {
    for e in edges {
        println!("({},{}) -> ({},{}) [{}]", e.u.x, e.u.y, e.v.x, e.v.y, e.len());
    }
}

/// Print vertices
fn vprint(points: &Vec<Vertex>) {
    for p in points {
        println!("( {:2}, {:2} )", p.x, p.y);
    }
}
