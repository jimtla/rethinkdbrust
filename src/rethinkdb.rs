
pub struct Field(String, String);

pub struct Element {
    fields: Vec<Field>
}

pub struct Connection {
    url: String,
    port: String
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

    pub fn connect(url: String , port: String)->Connection {
        Connection{url:url, port: port}
    }

    /*pub fn use(&self, dbname: &str)-> Db {

    }*/
}
