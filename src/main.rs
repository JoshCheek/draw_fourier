extern crate svg2polylines;
use std::io::{self, Read};
use std::process::exit;
use svg2polylines::{Polyline};

fn main() {
    let mut buffer = String::new();
    io::stdin().read_to_string(&mut buffer).unwrap();

    let polylines: Vec<Polyline> = svg2polylines::parse(&buffer).unwrap_or_else(|e| {
        println!("Error: {}", e);
        exit(2);
    });

    println!("Found {} polylines.", polylines.len());
    for line in polylines {
        for coord in line {
            println!("- {:?}", coord);
        }
    }
}
