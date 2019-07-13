extern crate svg;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;
use std::io::{self};


fn main() {
    for event in svg::read(io::stdin()).unwrap() {
        match event {
            Event::Tag(Path, _, attributes) => {
                let data = attributes.get("d").unwrap();
                let data = Data::parse(data).unwrap();
                for command in data.iter() {
                    match command {
                        &Command::Move(..) => println!("Move!"),
                        &Command::Line(..) => println!("Line!"),
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }
}
