extern crate byteorder;

use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use ql2::*;


pub struct Connection {
    pub host: String,
    pub port: u16,
    stream: BufStream<TcpStream>,
	auth : String
}



impl Connection {

    pub fn connect(host: &str , port: u16, auth : &str)->Connection {

        let stream = TcpStream::connect((host, port)).ok().unwrap();

        let mut conn = Connection{
                    host    : host.to_string(),
                    port    : port,
                    stream  : BufStream::new(stream),
				    auth    : auth.to_string()
                };

        conn.handshake();
        conn

    }



    fn handshake(&mut self)  {
        use ql2::VersionDummy_Version;
        let V0_4 =  0x400c2d20;
        let JSON =  0x7e6970c7;
        self.stream.write_u32::<LittleEndian>(VersionDummy_Version::V0_4 as u32);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(VersionDummy_Protocol::JSON as u32);
        self.stream.flush();
        
        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        self.stream.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => print!("{:?}", "OK, foi"),
            _ => print!("{:?}", "Unable to connect")
        }
        
    }

}

#[test]
fn test_connect() {
    Connection::connect("localhost", 28015, "");
}
