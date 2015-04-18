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

macro_rules! json_array {
    ( $( $e:expr ),* )  => {{
        let mut a = Vec::new();
        $(
            a.push($e);
        )*
        Json::Array(a)
    }}
}

macro_rules! json_string {
    ($s:expr) => { Json::String($s.clone()) }
}

macro_rules! json_opts {
    ( $( $k:expr => $v:expr),* ) => {{
        let mut d = BTreeMap::new();
        $(
            d.insert($k.to_string(), $v);
        )*
        Json::Object(d)


    }}
}

macro_rules! json_i64 {
    ($s:expr) => { Json::I64($s) }
}


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
    name : String,
    primary_key : String,
    replicas : i32,
    shards   : i32,
    primary_replica_tag : String
}

impl<'a> TableCreate<'a> {
    fn new(db : &'a Db, name : &str) -> TableCreate<'a> {
        TableCreate {
            term : Term_TermType::TABLE_CREATE,
            stm  : "table_create".to_string(),
            db   : db,
            name : name.to_string(),
            primary_key : "id".to_string(),
            replicas    : 1i32,
            shards      : 1i32,
            primary_replica_tag : "".to_string()
        }
    }

    fn replicas(&mut self, total : i32) {
        self.replicas = total;
    }
}



pub struct Table<'a> {//TODO criar um so struct ( Command? )
    term : Term_TermType,
    stm  : String,
    db   : &'a Db,
    name : String
}

pub struct TableInsert<'a> {
    term    : Term_TermType,
    stm     : String,
    table   : &'a Table<'a>,
    object  : BTreeMap<String, json::Json>
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

        json_array![
            Json::I64(self.term.clone() as i64),
            json_array![
                self.db.to_query_types(),
                json_string!(self.name.clone())
            ],
            json_opts![
                   "primary_key" => json_string!(self.primary_key.clone()),
                   "shards"      => json_i64!(self.shards as i64),
                   "replicas"    => json_i64!(self.replicas as i64)]
                   // TODO LAST PARAM PENDING : TAG
        ]

    }
}

impl<'a> RQLQuery<'a> for Table<'a> {
    fn to_query_types(&'a self) -> json::Json {

        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.db.to_query_types(),
                json_string!(self.name.clone())
            ],
            json_opts![
                "use_outdated" => Json::Boolean(true),
                "identifier_format".to_string() => json_string!("name".to_string())
            ]
        ]

    }
}

impl<'a> RQLQuery<'a> for TableInsert<'a> {
    fn to_query_types(&'a self) -> json::Json {

        let mut j = Vec::new();
        j.push(Json::I64(self.term.clone() as i64));

        let mut jd = Vec::new();
        jd.push(self.table.to_query_types());

        jd.push(Json::Object(self.object.clone()));

        let mut d = BTreeMap::new();
        d.insert("conflict".to_string(), Json::String("update".to_string()));
        d.insert("durability".to_string(), Json::String("hard".to_string()));//?
        d.insert("return_changes".to_string(), Json::Boolean(true));//?
        j.push(Json::Array(jd));
        j.push(Json::Object(d));
        Json::Array(j)

    }
}

impl<'a> RQLQuery<'a> for Db {
    fn to_query_types(&'a self) -> json::Json {

        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                json_string!(self.name.clone())
            ]
        ]
    }
}


impl<'a> Table<'a> {

    pub fn insert (&'a self, object : BTreeMap<String, json::Json>) -> TableInsert {
        //let db = Rc::new(self);
        TableInsert::new(self, object)
    }
}

impl<'a> TableInsert<'a> {
    fn new(table: &'a Table, object: BTreeMap<String, json::Json>) -> TableInsert<'a> {
        TableInsert {
            term    : Term_TermType::INSERT,
            stm     : "insert".to_string(),
            table   : table,
            object  : object
        }
    }
}

impl Db {
    pub fn table_create (&self, name : &str) -> TableCreate {
        TableCreate::new(self, name)
    }

    pub fn table (&self, name : &str) -> Table {
        Table {
            term : Term_TermType::TABLE,
            stm  : "table".to_string(),
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

        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        let mut buf = BufStream::new(&self.stream);
        buf.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => print!("{:?}", "OK, foi"),
            _ => print!("{:?}", "Unable to connect")
        }

        // let mut res = Response::new();
        // let mut reader = ::protobuf::stream::CodedInputStream::new(&mut self.stream);
        // res.merge_from(&mut reader);
        // println!("$$$$$$$$${:?}", res.get_field_type());
        // println!("$$$$$$$$${:?}", res.get_response().len());


    }

    fn send(&mut self, json : Json) {

        self.stream.write_i64::<LittleEndian>(1i64);
        let message = json.to_string();
        let len = message.len();
        self.stream.write_i32::<LittleEndian>(len as i32);
        println!("{}",message);
        write!(self.stream, "{}", message);

        let mut recv = Vec::new();
        let null_s = b"\0"[0];
        let mut buf = BufStream::new(&self.stream);
        buf.read_until(null_s, &mut recv);

        match recv.pop() {
            Some(null_s) => print!("{:?}", "OK, foi"),
            _ => print!("{:?}", "Unable to connect")
        }


        // let mut res = Response::new();
        // let mut reader = ::protobuf::stream::CodedInputStream::new(&mut self.stream);
        // res.merge_from(&mut reader);
        // println!("$$$$$$$$${:?}", res.get_field_type());
        // println!("$$$$$$$$${:?}", res.get_response().len());

    }

}

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
    assert_eq!(1, 2);

}
#[test]
fn test_insert() {
    let mut conn = Connection::connect("localhost", 7888, "");
    let mut nachoData = BTreeMap::new();
    nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
    nachoData.insert("age".to_string(), Json::I64(6i64));
    let db = db("test");
    let tc = db.table("person").insert(nachoData).run(&mut conn);

    println!("{:?}", nachoData);

    assert_eq!(1,2);


}
