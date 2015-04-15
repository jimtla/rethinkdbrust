use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use std::ascii::OwnedAsciiExt;

pub struct Field(String, String);

pub struct Element {
    fields: Vec<Field>
}

pub struct Connection {
    pub host: String,
    pub port: u16,
    stream: BufStream<TcpStream>,
	auth : String
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

    pub fn connect(host: &str , port: u16, auth : &str)->Connection {

        let stream = TcpStream::connect((host, port)).ok().unwrap();

        let conn = Connection{host   : host.to_string(),
                   port   : port,
                   stream : BufStream::new(stream),
				   auth : auth.to_string()};

        conn.handshake();


        conn

    }

    fn handshake(&self)-> Db {
/* 		let inner = Vec::new(); */
		
		let auth_ascii = self.auth.into_ascii_lowercase();
		
/*         let mut writer = BufWriter::with_capacity(2, inner);
		writer.write(b"V0_4");
		writer.write() //aqui pre cisa escrever a auth key */
		self.stream.write(b"v0_4");
		self.stream.write(auth_ascii.len());
		self.stream.write(auth_ascii);
		self.stream.write(b"JSON");
		self.stream.flush();
    }

    /*pub fn use(&self, dbname: &str)-> Db {

    }*/
}
