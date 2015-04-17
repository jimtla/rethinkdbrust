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
        // let query = self.to_query_types();
        // let token = 5u64;
        // let as_json = json::encode(&query.to_json()).unwrap();
        // print!("{:?}, {:?}, {:?}", token, as_json.len().to_u32().unwrap(), as_json);
        // conn.stream.write_u64::<LittleEndian>(token);
        // conn.stream.write_u32::<LittleEndian>(as_json.len().to_u32().unwrap());
        // //conn.stream.write(json_bytes);
        // write!(conn.stream, "{:?}", as_json);
        // conn.stream.flush();

        // let mut recv = Vec::new();
        // //unsafe { recv.set_len(12) };
        // let null_s = b"}"[0];
        // conn.stream.read_until(null_s, &mut recv);

        // match recv.pop() {
        //     Some(null_s) => println!("{:?}, {:?}", "CRIOU TABEL, foi\n", String::from_utf8(recv)),
        //     _ => panic!("{:?}", "Unable to connect\n")
        // }

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
        
        // let mut recv = Vec::new();
        // let null_s = b"\0"[0];
        // self.stream.read_until(null_s, &mut recv);

        // match recv.pop() {
        //     Some(null_s) => println!("{:?}", "OK, foi\n"),
        //     _ => panic!("{:?}", "Unable to connect\n")
        // }
    }

}

    use ::protobuf::Message;
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
    // let db = db("test");
    // assert_eq!("db", db.stm);
    // //let qd = db.table_create("person").to_query_types();
    // db.table_create("person").run(&mut conn);

    let mut db_datum = Datum::new();
    db_datum.set_field_type(Datum_DatumType::R_STR);
    db_datum.set_r_str("test".to_string());
    db_datum.compute_size();

    let mut db_datum_term = Term::new();
    db_datum_term.set_field_type(Term_TermType::DATUM);
    db_datum_term.set_datum(db_datum);
    db_datum_term.compute_size();
    
    let mut db_term = Term::new();
    db_term.set_field_type(Term_TermType::DB);
    db_term.set_args(::protobuf::RepeatedField::from_vec(vec![db_datum_term]));
    db_term.compute_size();

    let mut table_crate_datum = Datum::new();
    table_crate_datum.set_field_type(Datum_DatumType::R_STR);
    table_crate_datum.set_r_str("animals".to_string());
    table_crate_datum.compute_size();

    let mut primary_key_datum = Datum::new();
    primary_key_datum.set_field_type(Datum_DatumType::R_STR);
    primary_key_datum.set_r_str("id".to_string());
    primary_key_datum.compute_size();

    let mut primary_key_term = Term::new();
    primary_key_term.set_field_type(Term_TermType::DATUM);
    primary_key_term.set_datum(primary_key_datum);
    primary_key_term.compute_size();


    let mut shars_datum = Datum::new();
    shars_datum.set_field_type(Datum_DatumType::R_NUM);
    shars_datum.set_r_num(0.0);
    shars_datum.compute_size();

    let mut shars_term = Term::new();
    shars_term.set_field_type(Term_TermType::DATUM);
    shars_term.set_datum(shars_datum);
    shars_term.compute_size();

    let mut reps_datum = Datum::new();
    reps_datum.set_field_type(Datum_DatumType::R_NUM);
    reps_datum.set_r_num(0.0);
    reps_datum.compute_size();
    
    let mut reps_term = Term::new();
    reps_term.set_field_type(Term_TermType::DATUM);
    reps_term.set_datum(reps_datum);
    reps_term.compute_size();


    let mut prims_datum = Datum::new();
    prims_datum.set_field_type(Datum_DatumType::R_STR);
    prims_datum.set_r_str("x".to_string());
    prims_datum.compute_size();

    let mut prims_term = Term::new();
    prims_term.set_field_type(Term_TermType::DATUM);
    prims_term.set_datum(prims_datum);
    prims_term.compute_size();

    let mut opts_a = Term_AssocPair::new();
    opts_a.set_key("primary_key".to_string());
    opts_a.set_val(primary_key_term);
    opts_a.compute_size();
    
    let mut opts_b = Term_AssocPair::new();
    opts_b.set_key("shars".to_string());
    opts_b.set_val(shars_term);
    opts_b.compute_size();

    let mut opts_c = Term_AssocPair::new();
    opts_c.set_key("replicas".to_string());
    opts_c.set_val(reps_term);
    opts_c.compute_size();
    
    let mut opts_d = Term_AssocPair::new();
    opts_d.set_key("primary_replica_tag".to_string());
    opts_d.set_val(prims_term);
    opts_d.compute_size();
    

    let mut datum_term = Term::new();
    datum_term.set_field_type(Term_TermType::DATUM);
    datum_term.set_datum(table_crate_datum);
    datum_term.compute_size();

    let mut table_create_term = Term::new();
    table_create_term.set_field_type(Term_TermType::TABLE_CREATE);
    table_create_term.set_args(::protobuf::RepeatedField::from_vec(vec![db_term, datum_term]));
    table_create_term.set_optargs(::protobuf::RepeatedField::from_vec(vec![opts_a, opts_b, opts_c, opts_d]));
    table_create_term.compute_size();


    let mut query = Query::new();
    query.set_field_type(Query_QueryType::START);
    query.set_token(2i64);
    query.set_query(table_create_term);
    query.set_accepts_r_json(false);
    println!("{}", query.compute_size());


    {
    
        let mut writer = ::protobuf::stream::CodedOutputStream::new(&mut conn.stream);

        query.write_to_with_cached_sizes(&mut writer);
        writer.flush();
    }
    

    let mut res = Response::new();
    let mut reader = ::protobuf::stream::CodedInputStream::new(&mut conn.stream);
    res.merge_from(&mut reader);
    println!("$$$$$$$$${:?}", res.get_field_type());
    println!("$$$$$$$$${:?}", res.get_response().len());



    //print!("{:?}", json::encode(&qd.to_json()));

    // let db = QueryTypes::Query(Term_TermType::DB, vec![QueryTypes::Data("FOO".to_string())]);
    // let table = QueryTypes::Query(Term_TermType::TABLE, vec!(db, QueryTypes::Data("users".to_string())));
    // let filter = QueryTypes::Query(Term_TermType::FILTER, vec![table, QueryTypes::Data("{name : \"Paulo\"}".to_string())]);
    // print!("\n{:?}", json::encode(&filter.to_json()).unwrap());

    assert_eq!(1,2);
    //conn.db("foo").insert(person).run();



}
