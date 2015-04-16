extern crate byteorder;


use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use std::rc::Rc;
use ql2::*;
use rustc_serialize::json;
use rustc_serialize::json::{ToJson, Json};
use std::num::ToPrimitive;
use std::string::String;
use std::collections::BTreeMap;

enum QueryTypes {
    Query(Term_TermType, Vec<QueryTypes>),
    QueryWithArgs(Term_TermType, Vec<QueryTypes>, BTreeMap<String, json::Json>),
    Data(String)
}

/* Structs to manage databse */
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : BufStream<TcpStream>,
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
        let query = self.to_query_types();
        let token = 5u64;
        let as_json = json::encode(&query.to_json()).unwrap();
        print!("{:?}, {:?}, {:?}", token, as_json.len().to_u32().unwrap(), as_json);
        conn.stream.write_u64::<LittleEndian>(token);
        conn.stream.write_u32::<LittleEndian>(as_json.len().to_u32().unwrap());
        //conn.stream.write(json_bytes);
        write!(conn.stream, "{:?}", as_json);
        conn.stream.flush();

        let mut recv = Vec::new();
        //unsafe { recv.set_len(12) };
        let null_s = b"}"[0];
        conn.stream.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => println!("{:?}, {:?}", "CRIOU TABEL, foi\n", String::from_utf8(recv)),
            _ => panic!("{:?}", "Unable to connect\n")
        }

        true
    }
    fn to_query_types(&'a self) -> QueryTypes;

}


impl<'a> RQLQuery<'a> for TableCreate<'a> {
    fn to_query_types(&'a self) -> QueryTypes {
        let args : BTreeMap<String, json::Json> = BTreeMap::new();
        QueryTypes::QueryWithArgs(self.term, 
                                  vec![self.db.to_query_types(), 
                                       QueryTypes::Data(self.name.clone())], 
                                  args)
    }
}


impl<'a> RQLQuery<'a> for Db {
    fn to_query_types(&'a self) -> QueryTypes {
        QueryTypes::Query(self.term, vec![QueryTypes::Data(self.name.clone())])
    }
}

impl ToJson for QueryTypes {
    fn to_json(&self) -> Json {
        match *self  {
            QueryTypes::QueryWithArgs(t, ref v, ref a) => {
                let child = v.to_json();
                let mut me = Vec::new();
                me.push(Json::U64(t as u64));
                me.push(child);
                me.push(a.to_json());
                Json::Array(me)
            }
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
                    stream  : BufStream::new(stream),
				    auth    : auth.to_string()
                };

        conn.handshake();
        conn

    }

    fn handshake(&mut self)  {
        self.stream.write_u32::<LittleEndian>(VersionDummy_Version::V0_4 as u32);
        self.stream.write_u32::<LittleEndian>(0);
        self.stream.write_u32::<LittleEndian>(0x7e6970c7);
        self.stream.flush();
        
        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        self.stream.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => println!("{:?}", "OK, foi\n"),
            _ => panic!("{:?}", "Unable to connect\n")
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
    
    let mut conn = Connection::connect("localhost", 28015, "");
    let db = db("test");
    assert_eq!("db", db.stm);
    //let qd = db.table_create("person").to_query_types();
    db.table_create("person").run(&mut conn);

    //print!("{:?}", json::encode(&qd.to_json()));

    // let db = QueryTypes::Query(Term_TermType::DB, vec![QueryTypes::Data("FOO".to_string())]);
    // let table = QueryTypes::Query(Term_TermType::TABLE, vec!(db, QueryTypes::Data("users".to_string())));
    // let filter = QueryTypes::Query(Term_TermType::FILTER, vec![table, QueryTypes::Data("{name : \"Paulo\"}".to_string())]);
    // print!("\n{:?}", json::encode(&filter.to_json()).unwrap());

    assert_eq!(1,2);
    //conn.db("foo").insert(person).run();



}
