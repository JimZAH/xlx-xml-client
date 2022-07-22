mod network;
mod xlxparse;

use xml;

use std::fs::File;
use std::io::BufReader;
use std::io::BufWriter;
use std::io::Read;
use std::str;

use xml::reader::{EventReader, XmlEvent};

fn main() {
    let mut buf = [0; 1024 * 16];
    let f = File::open("/home/james/Downloads/xlxd.xml").unwrap();
    let mut f = BufReader::new(f);
    if let Ok(b) = f.read(&mut buf) {
        println!("Bytes: {}", b);
    }
    
    xlxparse::parse(&mut buf);

    let p = EventReader::new(buf.as_slice());

    for e in p {
        println!("{:?}", e);
    }
    let mut nc: network::Stream = network::Stream::new();
    nc.pt = network::PType::Ping;
    nc.write();
}
