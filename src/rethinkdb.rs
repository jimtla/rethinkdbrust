use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;

pub struct Field(String, String);

pub struct Element {
    fields: Vec<Field>
}

pub struct Connection {
    pub host: String,
    pub port: u16,
    stream: BufStream<TcpStream>
}

pub struct Db {
    name: String
}

pub struct Query {
    stmt: String
}

pub struct Result {
    pub status: i32,
    pub message: String,
    pub data: Vec<(String, String)>
}

impl Element {
    fn add_field(&self, field: Field) {

    }
}

impl Db {

    fn new(name: &str)-> Db {
        Db{name:name.to_string()}
    }

    fn table_create(name: &str)-> Query {
        Query{stmt:name.to_string()}
    }

    fn table(name: &str)-> Query {
        Query{stmt:name.to_string()}
    }
}

/*impl Query {
    pub fn run<F>(&self, Connection &con, callback: F)
    where F : Fn(Result) {
        //executa a query e chama callback(res)
    }
}*/

impl Connection {

    pub fn connect(host: &str , port: u16)->Connection {

        let stream = TcpStream::connect((host, port)).ok().unwrap();

        let conn = Connection{host   : host.to_string(),
                   port   : port,
                   stream : BufStream::new(stream)};

        conn.handshake();


        conn

    }

    fn handshake(&self)-> Db {

    }

    /*pub fn use(&self, dbname: &str)-> Db {

    }*/
}
