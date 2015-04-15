extern crate byteorder;

use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::rc::Rc;
use ql2::*;
use rustc_serialize::json;

pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : BufStream<TcpStream>,
	auth     : String
}

pub struct Db<'a> {
    term : Term_TermType,
    stm  : String,
    conn : &'a Connection
}

pub struct TableCreate<'a> {
    term : Term_TermType,
    stm  : String,
    db   : &'a  Db
}


impl Db {
    pub fn table_create (&self, name : &str) -> TableCreate {
        TableCreate {
            term : Term_TermType::TABLE_CREATE,
            stm  : "table_create",
            db   : self
        }

    }
}

impl TableCreate {
    pub fn run(&self) -> bool {
        true
    }
    
}



impl Connection {

    pub fn connect(host: &str , port: u16, auth : &str) -> Connection {

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
        self.stream.write_u32::<LittleEndian>(VersionDummy_Version::V0_4 as u32);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(VersionDummy_Protocol::JSON as u32);
        self.stream.flush();
        
        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        self.stream.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => print!("{:?}", "OK, foi"),
            _ => panic!("{:?}", "Unable to connect")
        }
    }

    fn db(&self, name : &str) -> Db {
        Db {
            term : Term_TermType::DB,
            stm  : "db".to_string(),
            conn : self
        }

    }


}

#[test]
fn test_connect() {
    struct Person {
        name : String,
        age : i8
    };
    
    let person = Person {
        name : "Nacho".to_string(),
        age : 6
    };
    
    let conn = Connection::connect("localhost", 28015, "");
    let db = conn.db("foo");
    db.table_create("person").run();
    assert_eq!("db", db.stm);
    //conn.db("foo").insert(person).run();
}
