use pool::Pool;
use std::thread;
use std::sync::{Arc, Mutex};
use rustc_serialize::json;
use rustc_serialize::json::Json;
use std::io::{BufStream, Error, Write, Read, BufRead};
use std::net::TcpStream;
use byteorder::{ReadBytesExt, WriteBytesExt, LittleEndian};
use ql2::*;
use std::str;

/* Structs to manage databse */
pub struct Connection {
    pub host : String,
    pub port : u16,
    stream   : TcpStream,
    auth     : String
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

    }

    fn send(&mut self, json : Json) -> Json {

        self.stream.write_i64::<LittleEndian>(1i64);
        let message = json.to_string();
        let len = message.len();
        self.stream.write_i32::<LittleEndian>(len as i32);
        println!("{}",message);
        write!(self.stream, "{}", message);
        self.stream.flush();

        //Read result. Should go into a different method?

        let recv_token = self.stream.read_i64::<LittleEndian>().ok().unwrap();
        let recv_len = self.stream.read_i32::<LittleEndian>().ok().unwrap();

        let mut buf = BufStream::new(&self.stream);
        
        let mut c = Vec::with_capacity(recv_len as usize);
        buf.read(&mut c);
        let json_recv = str::from_utf8(&c).ok().unwrap();

        
        let mut recv_json = json::Json::from_str(json_recv);
        println!("{:?}", json_recv);
        recv_json.ok().unwrap()

    }

}

pub struct RethinkDB {
    pool : Arc<Mutex<Pool<Connection>>>
}

impl RethinkDB {
    pub fn connect(host: &str , port: u16, auth : &str, pool_size : usize) -> RethinkDB {
        let mut pool = Pool::with_capacity(pool_size, 0, || {
            println!("{:?}#####", 3);
            Connection::connect(host, port, auth) }
            );
        RethinkDB {
            pool : Arc::new(Mutex::new(pool))
        }
    }

    #[inline(always)]
    pub fn send(&self, message : Json) -> Json {
        let con_arc = self.pool.clone();
        let mut pool = con_arc.lock().unwrap();
        let mut conn = &mut pool.checkout().unwrap();

        conn.send(message.clone())
    }

}