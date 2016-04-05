extern crate xml;

use std::fs::File;
use std::io::BufReader;
use std::io;

use xml::reader::{EventReader, XmlEvent};

fn main() {
    println!("Enter keyword to search for:");
    let mut keyword = String::new();
    io::stdin().read_line(&mut keyword)
    	.expect("Failed to read line");
    let file = File::open("E:/file.xml").unwrap();
    let file = BufReader::new(file);

    let parser = EventReader::new(file);
    let mut count = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement {name, .. }) => {
                if  name.local_name == keyword.trim() {
                	count = count + 1;
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
            _ => {}
        }
    }
    println!("Counter: {}", count);
}
