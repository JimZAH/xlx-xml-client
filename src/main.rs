mod network;
mod xlxparse;

use xml;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::str;

use xml::reader::{EventReader, XmlEvent};

struct Station {
    Callsign: String,
    Node: String,
}

impl Station {
    fn new() -> Self {
        Self {
            Callsign: "".to_string(),
            Node: "".to_string(),
        }
    }
}

fn main() {
    let mut calls: Vec<Station> = Vec::new();
    let mut buf = [0; 1024 * 16];
    let f = File::open("/home/james/Downloads/xlxd.xml").unwrap();
    let mut f = BufReader::new(f);
    if let Ok(b) = f.read(&mut buf) {
        println!("Bytes: {}", b);
    }

    xlxparse::parse(&mut buf);

    let mut p = EventReader::new(buf.as_slice());

    loop {
        match p.next() {
            Ok(r) => {
                match r {
                    XmlEvent::StartElement { name, .. } => {
                        //println!("Start: {}", name);
                        if &name.local_name[..] == "Callsign" {
                            let mut s: Station = Station::new();

                            if let Ok(XmlEvent::Characters(c)) = p.next() {
                                s.Callsign = c;
                                calls.push(s);
                            }
                        }
                    }
                    XmlEvent::Characters(c) => {}
                    _ => {}
                }
            }
            Err(_) => return,
        }
        for c in &calls {
            println!("{}", c.Callsign);
        }
    }
    let mut nc: network::Stream = network::Stream::new();
    nc.pt = network::PType::Ping;
    nc.write();
}
