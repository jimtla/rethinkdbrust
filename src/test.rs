#[cfg(test)]
#[warn(unused_imports)]
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::string::String;
use std::collections::BTreeMap;
use rethinkdb::{Connection, RethinkDB};
use api::*;


// socat  -v -x TCP4-LISTEN:7888,fork,reuseaddr TCP4:localhost:28015
#[test]
fn test_create() {
    let rethinkdb = RethinkDB::connect("localhost", 7888, "", 3);
    //let mut conn = Connection::connect("localhost", 7888, "");
    let db = db("test");
    let tc = db.table_create("person_create").replicas(1i32).run(&rethinkdb);
    let td = db.table_drop("person_create").run(&rethinkdb);
    assert_eq!(1, 2);

}

// #[test]
// fn test_insert() {
//     let mut conn = Connection::connect("localhost", 7888, "");
//     let mut nachoData = BTreeMap::new();
//     nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
//     nachoData.insert("age".to_string(), Json::I64(6i64));
//     let db = db("test");
//     let tc = db.table("person").insert(nachoData).run(&mut conn);

// }

// #[test]
// fn test_insert_option_conflict_update() {//TODO get last inserted and try to update it
//     let mut conn = Connection::connect("localhost", 7888, "");
//     let mut nachoData = BTreeMap::new();
//     nachoData.insert("name".to_string(), Json::String("Nacho".to_string()));
//     nachoData.insert("age".to_string(), Json::I64(8i64));
//     let db = db("test");
//     let tc = db.table("person").insert(nachoData).conflict("update").run(&mut conn);

//     assert_eq!(1,2);
// }


