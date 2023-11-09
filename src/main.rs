use std::fs::File;
use std::io::BufReader;

use xml::reader::{EventReader, XmlEvent};

// Base code from Reading XML documents
// See: https://github.com/kornelski/xml-rs

fn main() -> std::io::Result<()> {
    let file = File::open("./examples/demo1.xml")?;
    let file = BufReader::new(file); // Buffering is important for performance

    let parser = EventReader::new(file);
    let mut depth = 0;
    for e in parser {
        match e {
            Ok(XmlEvent::StartElement { name, attributes, .. }) => {
                let local_name = name.local_name;
                println!("{:spaces$}+{local_name}", "", spaces = depth * 3);
                for a in attributes {
                    let name = a.name;
                    let value = a.value;
                    println!("{:spaces$}> {name} = {value}", "", spaces = depth * 4);
                }
                depth += 1;
            }
            Ok(XmlEvent::EndElement { name }) => {
                depth -= 1;
                let local_name = name.local_name;
                println!("{:spaces$}-{local_name}", "", spaces = depth * 3);
            }
            Err(e) => {
                eprintln!("Error: {e}");
                break;
            }
            // There's more: https://docs.rs/xml-rs/latest/xml/reader/enum.XmlEvent.html
            _ => {}
        }
    }

    Ok(())
}