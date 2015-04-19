#[allow(unused_imports)]
extern crate byteorder;

use ql2::*;
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::collections::BTreeMap;
use std::str;
use client::RethinkDB;


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

/// Represents `db` command. Must be constructed with `rethinkdb::api::db`.
pub struct Db {
    term : Term_TermType,
    stm  : String,
    name : String
}

/// Represents `table_create` command. Must be constructed from a `Db`
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

pub struct TableDrop<'a> {
    term : Term_TermType,
    stm  : String,
    db   : &'a Db,
    name : String
}

impl<'a> TableDrop<'a> {
    fn new(db : &'a Db, name : &str) -> TableDrop<'a> {
        TableDrop {
            term : Term_TermType::TABLE_DROP,
            stm  : "table_drop".to_string(),
            db   : db,
            name : name.to_string()
        }
    }
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

    pub fn replicas(&'a mut self, total : i32) -> &mut TableCreate<'a> {
        self.replicas = total;
        self
    }

    pub fn shards(&'a mut self, total : i32) -> &mut TableCreate<'a> {
        self.shards = total;
        self
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
    object  : BTreeMap<String, json::Json>,
    conflict: String,
    durability: String,
    return_changes: bool
}

/// Producs a `Db` instance.
pub fn db(name : &str) -> Db {
    Db {
        term : Term_TermType::DB,
        stm  : "db".to_string(),
        name : name.to_string()
    }
}


/// All provides default `run` function for all RQLQueries.
pub trait RQLQuery<'a> {

    /// Takes a mutable reference of `RethinkDB` that handles the connection pool.
    fn run(&'a self, rethinkdb : &mut RethinkDB) -> bool {

        rethinkdb.send(Json::Array(vec![Json::I64(1), self.to_query_types()]));
        true
    }
    
    /// All implementations knows how to convert to the right Json protocol required by
    /// RethinkDB
    fn to_query_types(&'a self) -> json::Json;

}

impl<'a> RQLQuery<'a> for TableDrop<'a> {
    fn to_query_types(&'a self) -> json::Json {
        json_array![
            json_i64!(self.term.clone() as i64),
            json_array![
                self.db.to_query_types(),
                json_string!(self.name.clone())
            ]
        ]
    }
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

    pub fn insert (&'a self, object : BTreeMap<String, json::Json>) -> TableInsert { // TODO: fix this type. must be Json::Object
        TableInsert::new(self, object)
    }
}

impl<'a> TableInsert<'a> {
    fn new(table: &'a Table, object: BTreeMap<String, json::Json>) -> TableInsert<'a> {
        TableInsert {
            term    : Term_TermType::INSERT,
            stm     : "insert".to_string(),
            table   : table,
            object  : object,
            conflict: "error".to_string(),//default "error" accordingly rethinkdb documentation
            durability: "hard".to_string(),
            return_changes: true
        }
    }

    pub fn conflict(&mut self, value: &str) -> &TableInsert<'a> {//TODO: use methods that handle specific options.
        self.conflict = value.to_string();
        self
    }

    pub fn durability(&mut self, value: &str) -> &TableInsert<'a> {
        self.conflict = value.to_string();
        self
    }

    pub fn return_changes(&mut self, value: bool) -> &TableInsert<'a> {
        self.return_changes = value;
        self
    }
}

/// This is the main implementation of this API. All commands must be created from 
/// a `Db` instance;
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

    pub fn table_drop(&self, name : &str) -> TableDrop {
        TableDrop::new(self, name)
    }
}



