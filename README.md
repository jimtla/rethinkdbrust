# RethinkDB 2.0 Rust Driver

This is a very early stage WIP driver for json protocol. Compatible with Rust beta.

# Example

````rust
	// Creating new Table
    let mut conn = Connection::connect("localhost", 7888, "auth-token");
    let db = db("test");
    let tc = db.table_create("person_create").replicas(1i32).run(&mut conn);

    //
    struct Person {
    	name : String,
    	age  : u32,
    }
    let nacho = Person{name : "nacho".to_string(), age : 6};

    db("test").table("person").insert(json::encode(nacho).run(&mut conn);


````

# Messages implemented
   - DB
   - TABLE_CREATE
   - TABLE
   - TABLE_DROP

# Contributing
By now it is just a PR with the project bein able to build.


# Licence
MIT
