extern crate byteorder;

use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use byteorder::{ReadBytesExt, WriteBytesExt, BigEndian, LittleEndian};
use std::rc::Rc;
use ql2::*;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};


trait ToQueryTypes<'a> {
    fn to_query_types(&'a self) -> QueryTypes;
}

pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : BufStream<TcpStream>,
	auth     : String
}

pub struct Db<'a> {
    term : Term_TermType,
    stm  : String,
    conn : Rc<&'a Connection>,
    name : String
}

pub struct TableCreate<'a> {
    term : Term_TermType,
    stm  : String,
    db   : Rc<&'a Db<'a>>,
    name : String
}

impl<'a> ToQueryTypes<'a> for TableCreate<'a> {
    fn to_query_types(&'a self) -> QueryTypes {
        QueryTypes::Query(self.term, vec![self.db.to_query_types(), QueryTypes::Data(self.name.clone())])
    }
}


impl<'a> Db<'a> {
    pub fn table_create (&'a self, name : &str) -> TableCreate {
        let db = Rc::new(self);
        TableCreate {
            term : Term_TermType::TABLE_CREATE,
            stm  : "table_create".to_string(),
            db   : db.clone(),
            name : name.to_string()
        }
    }

}

impl<'a> ToQueryTypes<'a> for Db<'a> {
    fn to_query_types(&'a self) -> QueryTypes {
        QueryTypes::Query(self.term, vec![QueryTypes::Data(self.name.clone())])
    }
}

impl<'a> TableCreate<'a> {
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
            Some(null_s) => print!("{:?}", "OK, foi\n"),
            _ => panic!("{:?}", "Unable to connect\n")
        }
    }

    fn db(&self, name : &str) -> Db {
        let conn = Rc::new(self);
        Db {
            term : Term_TermType::DB,
            stm  : "db".to_string(),
            conn : conn.clone(),
            name : name.to_string()
        }

    }


}

enum QueryTypes {
    Query(Term_TermType, Vec<QueryTypes>),
    Data(String)
}

impl ToJson for QueryTypes {
    fn to_json(&self) -> Json {
        match *self  {
            QueryTypes::Query(t, ref v) => { 
                let child = v.to_json();
                let mut me = Vec::new();
                me.push(Json::U64(t as u64));
                me.push(child);
                Json::Array(me)
            }
            QueryTypes::Data(ref s) => Json::String(s.clone())
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
    // assert_eq!("db", db.stm);
    let qd = db.table_create("person").to_query_types();

    print!("{:?}", json::encode(&qd.to_json()));

    // let db = QueryTypes::Query(Term_TermType::DB, vec![QueryTypes::Data("FOO".to_string())]);
    // let table = QueryTypes::Query(Term_TermType::TABLE, vec!(db, QueryTypes::Data("users".to_string())));
    // let filter = QueryTypes::Query(Term_TermType::FILTER, vec![table, QueryTypes::Data("{name : \"Paulo\"}".to_string())]);
    // print!("\n{:?}", json::encode(&filter.to_json()).unwrap());

    assert_eq!(1,2);
    //conn.db("foo").insert(person).run();



}
