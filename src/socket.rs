use std::{
    io::{Error, Read},
    net::{Ipv4Addr, Shutdown, TcpStream},
};

pub struct Socket {
    stream: TcpStream,
    buffer: Vec<u8>,
}

impl Drop for Socket {
    fn drop(&mut self) {
        if let Err(e) = self.stream.shutdown(Shutdown::Both) {
            eprintln!("Failed to shutdown socket: {}", e);
        }
    }
}

impl Socket {
    pub fn new(address: (Ipv4Addr, u16)) -> Result<Self, Error> {
        let stream = TcpStream::connect(address)?;

        Ok(Socket {
            stream,
            buffer: vec![0; 0x1000],
        })
    }

    pub fn read(&mut self) -> Result<&[u8], Error> {
        let read = self.stream.read(&mut self.buffer)?;
        Ok(&self.buffer[..read])
    }
}
