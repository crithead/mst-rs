//! MST Data
//! Generates a random set of points in a plane.
//!
//! data -n N -m M -w W -h H -O 0,0 -o output_file | -c config_file
//!
//! Read the values from the configuration file, then from the command line
//! with command line values overriding configuration file values.

use clap::{App,Arg};
use std::fs::File;
use std::io::prelude::*;
use std::path::Path;
use std::rc::Rc;
use mst::{self,generate,Vertex};

/// Graph creation options
#[derive(Clone)]
struct Options {
    /// Print usage
    print_help: bool,
    /// Print extra messages
    verbose: bool,
    /// Number of vertices
    num_points: i32,
    /// Minimum distance between vertices
    min_distance: f32,
    /// Origin X
    origin_x: i32,
    /// Origin Y
    origin_y: i32,
    /// Width
    width: i32,
    /// Height
    height: i32,
    /// Separator string
    separator: Rc<String>,
    /// The output file name
    output: Rc<String>,
}

/// Default number of points to generate
const DEFAULT_NUM_POINTS: i32 = 200;
/// Default minumum distance between points
const DEFAULT_MIN_DISTANCE: f32 = 4.0;
/// Default x-value of origin (lower left)
const DEFAULT_ORIGIN_X: i32 = 0;
/// Default y-value of origin (lower left)
const DEFAULT_ORIGIN_Y: i32 = 0;
/// Default graph area width
const DEFAULT_WIDTH: i32 = 200;
/// Default graph area height
const DEFAULT_HEIGHT: i32 = 200;

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

    let points = match generate(opts.num_points, opts.min_distance,
            opts.origin_x, opts.origin_y,
            opts.origin_x + opts.width, opts.origin_y + opts.height) {
        Ok(points) => points,
        Err(e) => {
            println!("{}", e);
            std::process::exit(2);
        }
    };

    if opts.output.len() > 0 {
        vwrite(&points, &opts.output, &opts.separator);
    } else {
        vprint(&points, &opts.separator);
    }
}

/// Get program options
fn get_options() -> Option<Options> {
    let mut options = Options {
        print_help: false,
        verbose: false,
        num_points: DEFAULT_NUM_POINTS,
        min_distance: DEFAULT_MIN_DISTANCE,
        origin_x: DEFAULT_ORIGIN_X,
        origin_y: DEFAULT_ORIGIN_Y,
        width: DEFAULT_WIDTH,
        height: DEFAULT_HEIGHT,
        separator: Rc::new("\t".to_string()),
        output: Rc::new("".to_string()),
    };

    let matches = App::new("MST Data")
        .arg(Arg::with_name("help")
            .short("?")
            .long("help")
            .help("Print usage and exit"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose")
            .help("Enable extra messages"))
        .arg(Arg::with_name("num-points")
            .short("n")
            .long("num-points")
            .takes_value(true)
            .help("The number of points to generate"))
        .arg(Arg::with_name("min-distance")
            .short("m")
            .long("min-distance")
            .takes_value(true)
            .help("Minimum distance between points"))
        .arg(Arg::with_name("origin")
            .short("O")
            .long("origin")
            .takes_value(true)
            .help("The lower left corner of the graph area, as X,Y"))
        .arg(Arg::with_name("width")
            .short("w")
            .long("width")
            .takes_value(true)
            .help("Width of the area in which to generate points"))
        .arg(Arg::with_name("height")
            .short("h")
            .long("height")
            .takes_value(true)
            .help("Height of the area in which to generate points"))
        .arg(Arg::with_name("separator")
            .short("s")
            .long("separator")
            .takes_value(true)
            .help("OUtput value separator string"))
        .arg(Arg::with_name("output-file")
            .short("o")
            .long("output")
            .takes_value(true)
            .help("Name of output file"))
        .get_matches();

    if matches.is_present("help") {
        options.print_help = true;
    }

    //if let Some(s) = matches.value_of("configuration-file") {
    //    *Rc::make_mut(&mut options.config) = s.to_string();
    //}

    if matches.is_present("verbose") {
        options.verbose = true;
    }

    if let Some(s) = matches.value_of("num-points") {
        let value = s.parse::<i32>().unwrap_or(DEFAULT_NUM_POINTS);
        if value >= mst::MINIMUM_NUM_POINTS {
            options.num_points = value;
        } else {
            println!("ERROR: invalid number of points (< {})",
                     mst::MINIMUM_NUM_POINTS);
            return None;
        }
    }

    if let Some(s) = matches.value_of("min-distance") {
        let value = s.parse::<f32>().unwrap_or(DEFAULT_MIN_DISTANCE);
        if value >= mst::MINIMUM_MIN_DISTANCE {
            options.min_distance = value;
        } else {
            println!("ERROR: invalid minimum distance (< {})",
                     mst::MINIMUM_MIN_DISTANCE);
            return None;
        }
    }

    if let Some(s) = matches.value_of("width") {
        let value = s.parse::<i32>().unwrap_or(DEFAULT_WIDTH);
        if value > mst::MINIMUM_WIDTH {
            options.width = value;
        } else {
            println!("ERROR: invalid width (< {})", mst::MINIMUM_WIDTH);
            return None;
        }
    }

    if let Some(s) = matches.value_of("height") {
        let value = s.parse::<i32>().unwrap_or(DEFAULT_HEIGHT);
        if value >= mst::MINIMUM_HEIGHT {
            options.height = value;
        } else {
            println!("ERROR: invalid height (< {})", mst::MINIMUM_HEIGHT);
            return None;
        }
    }

    if let Some(s) = matches.value_of("origin") {
        let mut i = s.split(",");
        if let Some(x) = i.next() {
            options.origin_x = x.parse::<i32>().unwrap_or(DEFAULT_ORIGIN_X);
        }
        if let Some(y) = i.next() {
            options.origin_y = y.parse::<i32>().unwrap_or(DEFAULT_ORIGIN_Y);
        }
    }

    if let Some(s) = matches.value_of("separator") {
        *Rc::make_mut(&mut options.separator) = s.to_string();
    }

    if let Some(s) = matches.value_of("output-file") {
        *Rc::make_mut(&mut options.output) = s.to_string();
    }

    Some(options)
}

/// Print a usage message
fn print_help() {
    println!("\nMST Data\n\n\
\tGenerate a set of random points in an area of a plane.  Write the points\n\
\tto the output file or the console.\n\n\
OPTIONS\n\n\
\t-?,--help             Print usage an exit\n\
\t-v,--verbose          Enable extra messages\n\
\t-n,--num-points N     Number of points (vertices) to generate\n\
\t-m,--min-distance N   Minumum distance between points\n\
\t-O,--origin X,Y       Lower left corner of the graph area\n\
\t-w,--width N          Width of the graph area\n\
\t-h,--height N         Height of the graph area\n\
\t-s,--separator S      Separtor string between x and Y values\n\
\t-o,--output FILENAME  Output file name\n\
    ");
}

/// Print options
fn print_options(opts: &Options) {
    eprintln!("print_help   : {}", opts.print_help);
    eprintln!("verbose      : {}", opts.verbose);
    eprintln!("num_points   : {}", opts.num_points);
    eprintln!("min_distance : {}", opts.min_distance);
    eprintln!("origin_x     : {}", opts.origin_x);
    eprintln!("origin_y     : {}", opts.origin_y);
    eprintln!("width        : {}", opts.width);
    eprintln!("height       : {}", opts.height);
    eprintln!("output       : {}", opts.output);
}

/// Print vertices to the console.
fn vprint(points: &Vec<Vertex>, separator: &str) {
    for p in points {
        println!("{}{}{}", p.x, separator, p.y);
    }
}

/// Print vertices to  a file.
fn vwrite(points: &Vec<Vertex>, output_file: &str, separator: &str) {
    let path = Path::new(output_file);
    let mut f = match File::create(&path) {
        Err(e) => {
            eprintln!("Failed to open file '{}': {}", path.display(), e);
            return;
        },
        Ok(file) => file,
    };

    for p in points {
        let line = format!("{}{}{}\n", p.x, separator, p.y);
        match f.write_all(line.as_bytes()) {
            Err(e) =>
                eprintln!("Filed to write to file '{}': {}", path.display(), e),
            Ok(_) => {},
        }
    }
}

