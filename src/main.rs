extern crate svg;
use svg::node::element::path::{Command, Data};
use svg::node::element::tag::Path;
use svg::parser::Event;

fn main() {

    let path = "hydra.svg";
    for event in svg::open(path).unwrap() {
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
