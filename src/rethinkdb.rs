extern crate byteorder;


use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::rc::Rc;
use ql2::*;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::num::ToPrimitive;
use std::string::String;
use std::collections::BTreeMap;



/* Structs to manage databse */
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : TcpStream,
    auth     : String
}

pub struct Db {
    term : Term_TermType,
    stm  : String,
    name : String
}

pub struct TableCreate<'a> {
    term : Term_TermType,
    stm  : String,
    db   : &'a Db,
    name : String
}

///////////////////
/* Module fns */
fn db(name : &str) -> Db {
    Db {
        term : Term_TermType::DB,
        stm  : "db".to_string(),
        name : name.to_string()
    }
}


///////////////////
/* Module Traits */
trait RQLQuery<'a> {

    fn run(&'a self, conn : &mut Connection) -> bool {
        conn.send(Json::Array(vec![Json::I64(1), self.to_query_types()]));
        true
    }
    fn to_query_types(&'a self) -> json::Json;

}


impl<'a> RQLQuery<'a> for TableCreate<'a> {
    fn to_query_types(&'a self) -> json::Json {
    
        let mut j = Vec::new();
        j.push(Json::I64(self.term as i64));
        
        let mut jd = Vec::new();
        jd.push(self.db.to_query_types());
        jd.push(Json::String(self.name.clone()));

        let mut d = BTreeMap::new();
        d.insert("primary_key".to_string(), Json::String("id".to_string()));
        d.insert("shards".to_string(), Json::I64(1 as i64));
        d.insert("replicas".to_string(), Json::I64(1 as i64));
        j.push(Json::Array(jd));
        j.push(Json::Object(d));
        Json::Array(j)

    }
}


impl<'a> RQLQuery<'a> for Db {
    fn to_query_types(&'a self) -> json::Json {
        // [1,[39,[[15,[[14,["blog"]],"users"]],{"name":"Michel"}]],{}]
        // [1,[60,[[14,["test"]],"person",{"primary_key":"id","replicas":1,"shards":1}]]]
        
        let mut j = Vec::new();
        j.push(Json::I64(self.term as i64));
        
        let mut jd = Vec::new();
        jd.push(Json::String(self.name.clone()));
        j.push(Json::Array(jd));
        
        Json::Array(j)
        
    }
}

impl Db {
    pub fn table_create (&self, name : &str) -> TableCreate {
        let db = Rc::new(self);
        TableCreate {
            term : Term_TermType::TABLE_CREATE,
            stm  : "table_create".to_string(),
            db   : self,
            name : name.to_string()
        }
    }

}


impl Connection {

    pub fn connect(host: &str , port: u16, auth : &str) -> Connection {

        let stream = TcpStream::connect((host, port)).ok().unwrap();

        let mut conn = Connection{
                    host    : host.to_string(),
                    port    : port,
                    stream  : stream,
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
        
        let mut res = Response::new();
        let mut reader = ::protobuf::stream::CodedInputStream::new(&mut self.stream);
        res.merge_from(&mut reader);
        println!("$$$$$$$$${:?}", res.get_field_type());
        println!("$$$$$$$$${:?}", res.get_response().len());


    }

    fn send(&mut self, json : Json) {

        self.stream.write_i64::<LittleEndian>(1i64);
        let message = json.to_string();
        let len = message.len();
        self.stream.write_i32::<LittleEndian>(len as i32);
        println!("{}",message);
        write!(self.stream, "{}", message);


        let mut res = Response::new();
        let mut reader = ::protobuf::stream::CodedInputStream::new(&mut self.stream);
        res.merge_from(&mut reader);
        println!("$$$$$$$$${:?}", res.get_field_type());
        println!("$$$$$$$$${:?}", res.get_response().len());

    }

}

    use ::protobuf::Message;

// socat  -v -x TCP4-LISTEN:7888,fork,reuseaddr TCP4:localhost:28015
#[test]
fn test_connect() {
    // struct Person {
    //     name : String,
    //     age : i8
    // };
    
    // let person = Person {
    //     name : "Nacho".to_string(),
    //     age : 6
    // };
    
    let mut conn = Connection::connect("localhost", 7888, "");
    let db = db("test");
    assert_eq!("db", db.stm);
    let tc = db.table_create("person").run(&mut conn); 
    assert_eq!(1,2);

}
