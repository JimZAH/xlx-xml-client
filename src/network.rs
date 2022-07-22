use std::io::prelude::*;
use std::net::TcpStream;
use std::time::Duration;

const BUFF_MAX: usize = 1204;

#[derive(Clone, Copy, PartialEq)]
pub enum PType {
    Ping,
    Data,
    Command,
}

pub struct Stream {
    pub pt: PType,
    pub data: [u8; BUFF_MAX],
    crc: u8,
}

impl Stream {
    pub fn new() -> Self {
        Self {
            pt: PType::Ping,
            data: [0; BUFF_MAX],
            crc: 0,
        }
    }

    fn connect(host: &str) -> std::io::Result<TcpStream> {
        let stream = TcpStream::connect(host)?;
        stream.set_read_timeout(Some(Duration::from_millis(5000)))?;
        stream.set_write_timeout(Some(Duration::from_millis(10000)))?;
        Ok(stream)
    }

    pub fn write(&mut self) -> bool {
        let mut connection = match Stream::connect("localhost:1234") {
            Ok(c) => c,
            Err(_) => {
                eprintln!("Error with TCP");
                std::process::exit(2);
            }
        };

        self.data[0] = self.pt as u8;

        // A horrible simple data check. We are using TCP so I don't think a true
        // crc is really needed.
        // The crc byte is placed on end of buffer.
        self.crc = self.data[5];
        self.data[BUFF_MAX - 1] = self.crc;

        // Send ping
        if self.pt == PType::Ping {
            let keepalive = [0x0, 0x0, 0x9, 0x0, 0x50, 0x49, 0x4e, 0x47, self.crc];
            if let Ok(bytes) = connection.write(&keepalive) {
                println!("Keepalive sent: {}", bytes);
            }

            if let Ok(bytes) = connection.read(&mut self.data) {
                println!("Received: {} bytes", bytes);
                for i in &mut self.data[0..10] {
                    println!("{:x}", i);
                }
            }
            return true;
        }

        if let Ok(bytes) = connection.write(&self.data) {
            println!("{}", bytes);
            return true;
        }

        false
    }
}
