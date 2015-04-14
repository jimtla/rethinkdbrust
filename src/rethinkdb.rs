
use std::net::SocketAddrV4;
use std::net::Ipv4Addr;

pub struct Field(String, String);

pub struct Element {
    fields: Vec<Field>
}

pub struct Connection {
    url: String,
    port: String,
    socket: SocketAddrV4
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

    pub fn connect(url: String , port: u16)->Connection {

        let ip = Ipv4Addr.from_str(url).OK();

        let sock = SocketAddrV4.new(ip , port);

        Connection{url:url, port: port, socket:sock}
    }

    /*pub fn use(&self, dbname: &str)-> Db {

    }*/
}
