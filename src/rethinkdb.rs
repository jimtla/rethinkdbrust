extern crate byteorder;

use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;

use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};


pub struct Field(String, String);

pub struct Element {
    fields: Vec<Field>
}

pub struct Connection {
    pub host: String,
    pub port: u16,
    stream: BufStream<TcpStream>,
	auth : String
}

pub struct Db {
    name: String
}

pub struct Query {
    stmt: String
}

pub struct Result {
    pub status: i32,
    pub message: String,
    pub data: Vec<(String, String)>
}

impl Element {
    fn add_field(&self, field: Field) {

    }
}

impl Db {

    fn new(name: &str)-> Db {
        Db{name:name.to_string()}
    }

    fn table_create(name: &str)-> Query {
        Query{stmt:name.to_string()}
    }

    fn table(name: &str)-> Query {
        Query{stmt:name.to_string()}
    }
}

/*impl Query {
    pub fn run<F>(&self, Connection &con, callback: F)
    where F : Fn(Result) {
        //executa a query e chama callback(res)
    }
}*/

// fn to_array(value : u32) -> [u8;4] {

//     let fa : u32 = 255;
//     let mut x:u32 = value;
//     let mut res:u32 = 0;

//     let mut b1:u8 = 0;
//     let mut b2:u8 = 0;
//     let mut b3:u8 = 0;
//     let mut b4:u8 = 0;

//     res = x >> 24 & fa;
//     b1 = res.to_u8().unwrap_or(0);
//     res = x >> 16 & fa;
//     b2 = res.to_u8().unwrap_or(0);

//     res = x >> 8 & fa;
//     b3 = res.to_u8().unwrap_or(0);

//     b4 = x.to_u8().unwrap_or(0);

//     [b1, b2, b3, b4]
// }

impl Connection {

    pub fn connect(host: &str , port: u16, auth : &str)->Connection {

        let stream = TcpStream::connect((host, port)).ok().unwrap();

        let mut conn = Connection{host   : host.to_string(),
                   port   : port,
                   stream : BufStream::new(stream),
				   auth : auth.to_string()};

        conn.handshake();


        conn

    }



    fn handshake(&mut self)  {
        let V0_4 =  0x400c2d20;
        let JSON =  0x7e6970c7;
        self.stream.write_u32::<LittleEndian>(V0_4);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(JSON);
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
