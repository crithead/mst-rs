//! MST Calc
//! Calculate the minimum spanning tree from a set of points in a plane.
//!
//! Read a set of points representing a completely connected undirected graph
//! from a file, calculate the minimum spanning tree, and write the edges to
//! a file.
//!
//! calc -i vertices.csv -o edges.csv   # required
//! calc < vertices.csv > edges.csv     # optional

use clap::{App,Arg};

use std::fs::File;
use std::io;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;

use mst::{self,minimum_spanning_tree,Edge,Vertex};

/// Graph creation options
#[derive(Clone)]
struct Options {
    /// Print usage
    print_help: bool,
    /// Print extra messages
    verbose: bool,
    /// The input file name
    input: Rc<String>,
    /// The output file name
    output: Rc<String>,
}

const FSEP: &str = mst::FIELD_SEPARATOR;
//const RSEP: &str = mst::RECORD_SEPARATOR;

/// Read command line options.
/// Generate a set of random points.
/// Write the points to a file.
fn main() {
    let opts = match get_options() {
        Some(opts) => opts,
        None => std::process::exit(1),
    };

    if opts.print_help {
        print_help();
        std::process::exit(0);
    }

    if opts.verbose {
        print_options(&opts);
    }

    // TODO
    // If opts.input is not empty, create a Path from it and open the file
    // for reading, Else use stdin

    // Open input
    let fin = io::stdin();

    // Read points
    let points = match ingest(fin.lock()) {
        Ok(points) => points,
        Err(e) => {
            println!("{}", e);
            std::process::exit(2);
        }
    };

    if opts.verbose {
        eprintln!("Read {} points", points.len());
    }

    // Construct the Minimum Spanning Tree from the set of points
    let tree = match minimum_spanning_tree(&points) {
        Ok(tree) => tree,
        Err(e) => {
            println!("{}", e);
            std::process::exit(3);
        }
    };

    if opts.verbose {
        eprintln!("Found {} edges", tree.len());
    }

    // TODO
    // If opts.output is not empty, create a Path from it and open the file
    // for writing, Else use stdin

    let mut fout = io::stdout();
    exhaust(&tree, &mut fout);
}

/// Get program options
fn get_options() -> Option<Options> {
    let mut options = Options {
        print_help: false,
        verbose: false,
        input: Rc::new("".to_string()),
        output: Rc::new("".to_string()),
    };

    let matches = App::new("MST Calc")
        .arg(Arg::with_name("help")
            .short("?")
            .long("help")
            .help("Print usage and exit"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enable debug messages"))
        .arg(Arg::with_name("input-file")
            .short("i")
            .long("input")
            .takes_value(true)
            .help("Name of input file"))
        .arg(Arg::with_name("output-file")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Name of output file"))
        .get_matches();

    if matches.is_present("help") {
        options.print_help = true;
    }

    if matches.is_present("verbose") {
        options.verbose = true;
    }

    if let Some(s) = matches.value_of("input-file") {
        *Rc::make_mut(&mut options.input) = s.to_string();
    }

    if let Some(s) = matches.value_of("output-file") {
        *Rc::make_mut(&mut options.output) = s.to_string();
    }

    Some(options)
}

/// Print a usage message
fn print_help() {
    println!("\nMST Data\n\n\
\tRead a set of points, find the minimum spanning tree of those points as\n\
\ta completely connected undirected graph, then write out the set of edges\n\
\tthat form a minimum spanning tree.\n\n\
USAGE\n\n\
\tcalc --help\n\
\tcalc -i data.csv -o tree.csv\n\
\tcalc < data.csv > tree.csv\n\n\
OPTIONS\n\n\
\t-?,--help             Print usage an exit\n\
\t-v,--verbose          Enable debug messages (to stderr)\n\
\t-i,--input FILENAME   Input file name (Default: stdin)\n\
\t-o,--output FILENAME  Output file name (Default: stdout)\n\
    ");
}

/// Print options
fn print_options(opts: &Options) {
    eprintln!("print_help   : {}", opts.print_help);
    eprintln!("verbose      : {}", opts.verbose);
    eprintln!("input        : {}", opts.input);
    eprintln!("output       : {}", opts.output);
}


/// Read points from a Reader
fn ingest<R>(reader: R) -> io::Result<Vec<Vertex>>
    where R: BufRead
{
    let mut points = Vec::<Vertex>::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap_or("".to_string());
        if line.len() < 3 || line.starts_with("#") {
            continue;
        }
        let mut i = line.split(FSEP);
        let x = if let Some(s) = i.next() {
            s.parse::<i32>().unwrap_or(0)
        } else {
            continue;   // malformed input line
        };
        let y = if let Some(s) = i.next() {
            s.parse::<i32>().unwrap_or(0)
        } else {
            continue;   // malformed input line
        };
        points.push(Vertex::new(x, y));
    }

    Ok(points)
}

/// Print edges to the a Writer.
fn exhaust<W>(edges: &Vec<Edge>, writer: &mut W)
    where W: Write
{
    for e in edges {
        match writeln!(writer, "{}{}{}{}{}{}{}", e.u.x, FSEP, e.u.y, FSEP,
                       e.v.x, FSEP, e.v.y) {
            Err(e) => eprintln!("calc::exhaust: {}", e),
            _ => {}
        }
    }
}

