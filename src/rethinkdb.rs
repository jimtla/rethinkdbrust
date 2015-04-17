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
    fn to_query_types(&'a self) -> Term;

}


macro_rules! datum { //TODO: reduce repetition
    (NUM => $value:expr) => {{
            let mut datum = Datum::new();
            datum.set_field_type(Datum_DatumType::R_NUM);
            datum.set_r_num($value);
            datum
    }};

    (STR => $value:expr) => {{
            let mut datum = Datum::new();
            datum.set_field_type(Datum_DatumType::R_STR);
            datum.set_r_str($value.clone());
            datum
    }};

}


macro_rules! term_query {
    ($ty:path, $args:expr, $opts:expr) => {{
        let mut term = Term::new();
        term.set_field_type($ty);
        term.set_args(::protobuf::RepeatedField::from_vec($args));
        term.set_optargs(::protobuf::RepeatedField::from_vec($opts));
        term.compute_size();
        term
    }};
    ($ty:path, $args:expr) => {{
        let mut term = Term::new();
        term.set_field_type($ty);
        term.set_args(::protobuf::RepeatedField::from_vec($args));
        term.compute_size();
        term
    }};

}

#[macro_export]
macro_rules! term_datum {
    ($datum:expr) => {{
        let mut datum_term = Term::new();
        datum_term.set_field_type(Term_TermType::DATUM);
        datum_term.set_datum($datum);
        datum_term.compute_size();
        datum_term
    }};
    (STR => $value:expr) => {{
       
        let mut datum = datum!(STR => $value);
        term_datum!(datum)

    }};

    (NUM => $value:expr) => {{
       
        let mut datum = datum!(NUM => $value); // uses forwarded type
        term_datum!(datum)


    }};


}

macro_rules! term_assoc_pair {
    ($key:expr, $val:expr) => {{
        let mut pair = Term_AssocPair::new();
        pair.set_key($key.to_string());
        pair.set_val($val);
        pair.compute_size();
        pair

    }};
}
impl<'a> RQLQuery<'a> for TableCreate<'a> {
    fn to_query_types(&'a self) -> Term {
    
    //    let table_crate_datum = str_datum!(self.name);

    // let mut table_crate_datum = Datum::new();
    // table_crate_datum.set_field_type(Datum_DatumType::R_STR);
    // table_crate_datum.set_r_str(self.name.clone());
    // table_crate_datum.compute_size();

    let mut primary_key_term = term_datum!(STR => "id".to_string());
    let mut shars_term = term_datum!(NUM => 1f64);
    let mut reps_term = term_datum!(NUM => 1f64);
    let mut prims_term = term_datum!(STR => "X".to_string());

    let mut opts_a = term_assoc_pair!("primary_key", primary_key_term);
    let mut opts_b = term_assoc_pair!("shards", shars_term);
    let mut opts_c = term_assoc_pair!("replicas", reps_term);
    let mut opts_d = term_assoc_pair!("primary_replica_tag", prims_term);
  

    let table_create_datum_term = term_datum!(STR => self.name);

    let mut table_create_term = term_query!(Term_TermType::TABLE_CREATE, 
                                            vec![self.db.to_query_types(), table_create_datum_term],
                                            vec![opts_a, opts_b, opts_c]);
    
    table_create_term
    }
}


impl<'a> RQLQuery<'a> for Db {
    fn to_query_types(&'a self) -> Term {
        let mut db_datum_term = term_datum!(STR => self.name);
        term_query!(Term_TermType::DB, vec![db_datum_term])
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
        self.stream.write_u32::<LittleEndian>(VersionDummy_Protocol::PROTOBUF as u32);
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
    let db = db("test");
    // assert_eq!("db", db.stm);
    // //let qd = db.table_create("person").to_query_types();
    let tc = db.table_create("person"); //.run(&mut conn);

    // let mut db_datum = Datum::new();
    // db_datum.set_field_type(Datum_DatumType::R_STR);
    // db_datum.set_r_str("test".to_string());
    // db_datum.compute_size();

    // let mut db_datum_term = Term::new();
    // db_datum_term.set_field_type(Term_TermType::DATUM);
    // db_datum_term.set_datum(db_datum);
    // db_datum_term.compute_size();
    
    // let mut db_term = Term::new();
    // db_term.set_field_type(Term_TermType::DB);
    // db_term.set_args(::protobuf::RepeatedField::from_vec(vec![db_datum_term]));
    // db_term.compute_size();




    let mut query = Query::new();
    query.set_field_type(Query_QueryType::START);
    query.set_token(2i64);
    query.set_query(tc.to_query_types());
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
