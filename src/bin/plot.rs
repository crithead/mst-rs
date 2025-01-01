//! MST Plot
//!
//! Plot a set of graph edges to a PNG file.
//!
//! plot [ -v ] -i tree.csv -o mst.png
//!

use clap::{App,Arg};
use std::io;
use std::rc::Rc;
use std::fs::File;
use std::path::Path;
use mst::{self,plot,Edge};

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

/// Alias the MST field seaprator
const FSEP: &str = mst::FIELD_SEPARATOR;

/// Read command line options.  Read a set of line segments (graph edges)
/// from a file or stdin. Plot the line segements and write to a PNG file.
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
        eprintln!("MST Plot");
        print_options(&opts);
    }

    // Open input
    let edges = if opts.input.len() > 0 {
        // Use the given input file
        let path = Path::new(&*opts.input);
        match File::open(&path) {
            Ok(f) => {
                if opts.verbose {
                    eprintln!("Opened input '{}'", path.display());
                }
                match ingest(io::BufReader::new(f)) {
                    Ok(edges) => edges,
                    Err(e) => {
                        println!("Error: {}", e);
                        std::process::exit(1);
                    }
                }
            },
            Err(e) => {
                eprintln!("Failed to open '{}': {}", path.display(), e);
                std::process::exit(1);
            }
        }
    } else {
        if opts.verbose {
            eprintln!("Reading from stdin");
        }
        match ingest(io::stdin().lock()) {
            Ok(edges) => edges,
            Err(e) => {
                println!("Error: {}", e);
                std::process::exit(1);
            }
        }
    };

    if opts.verbose {
        eprintln!("Read {} edges", edges.len());
    }

    if edges.len() > 0 {
        if opts.verbose {
            eprintln!("Plotting graph");
        }
        plot(&edges, &opts.output).expect("write image failed");
    } else {
        eprintln!("Nothing to plot");
    }

    if opts.verbose {
        eprintln!("Done");
    }
}

/// Get program options
fn get_options() -> Option<Options> {
    let mut options = Options {
        print_help: false,
        verbose: false,
        input: Rc::new("".to_string()),
        output: Rc::new("mst.png".to_string()),
    };

    let matches = App::new("MST Plot")
        .arg(Arg::with_name("help")
            .short("h")
            .long("help"))
        .arg(Arg::with_name("verbose")
            .short("v")
            .long("verbose"))
        .arg(Arg::with_name("input-file")
            .short("i")
            .long("input")
            .takes_value(true))
        .arg(Arg::with_name("output-file")
            .short("o")
            .long("output")
            .takes_value(true))
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
    println!("\nMST Plot\n\n\
\tRead a set of line segments (edges).\n\
\tDraw them on an image.
\tWrite the image to a PNG file.\n\n\
USAGE\n\n\
\tplot [ -v ] -i tree.csv -o mst.png\n\
\tplot -v < tree.png        # Image in default 'mst.png'\n\n\
OPTIONS\n\n\
\t-h,--help                 Print usage an exit\n\
\t-v,--verbose              Enable extra messages\n\
\t-i,--input FILENAME       Input file name (Default: stdin)\n\
\t-o,--output FILENAME      Output file name (Default: mst.png)\n\
    ");

}

/// Read points from a Reader.
fn ingest<R>(reader: R) -> io::Result<Vec<Edge>>
    where R: io::BufRead
{
    let mut edges = Vec::<Edge>::new();

    for line_result in reader.lines() {
        let line = line_result.unwrap_or("".to_string());
        if line.len() < 3 || line.starts_with("#") {
            continue;
        }
        let mut i = line.split(FSEP);
        let x0 = if let Some(s) = i.next() {
            s.parse::<i32>().unwrap_or(0)
        } else {
            eprintln!("malformed input line");
            continue;
        };
        let y0 = if let Some(s) = i.next() {
            s.parse::<i32>().unwrap_or(0)
        } else {
            eprintln!("malformed input line");
            continue;
        };
        let x1 = if let Some(s) = i.next() {
            s.parse::<i32>().unwrap_or(0)
        } else {
            eprintln!("malformed input line");
            continue;
        };
        let y1 = if let Some(s) = i.next() {
            s.parse::<i32>().unwrap_or(0)
        } else {
            eprintln!("malformed input line");
            continue;
        };
        edges.push(Edge::new(x0, y0, x1, y1));
    }

    Ok(edges)
}

/// Print options
fn print_options(opts: &Options) {
    eprintln!("Options");
    eprintln!("  print_help : {}", opts.print_help);
    eprintln!("  verbose    : {}", opts.verbose);
    eprintln!("  input      : {}", opts.input);
    eprintln!("  output     : {}", opts.output);
}
