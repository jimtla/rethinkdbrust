pub struct Field(&str,&str)

pub struct Element {
    fields: Vec<Field>
}

pub struct Connection {
    url: str,
    port: str
}

pub struct Db {
    name: str
}

pub struct Query {
    stmt: str
}

pub struct Result {
    status: i32
}

impl Element {
    fn add_field(&self, field: Field) {

    }
}

impl DB {
    fn table_create(name: &str)-> Query {

    }
}

impl Table {
    fn insert(elements: &Vec<Element>)-> Query {

    }
}

impl Query {
    fn run(Connection &con)->Result {

    }
}

impl Connection {

    pub fn connect()->Connection {

    }

    pub fn use(&self, table: &str)-> Connection {

    }

    pub fn db(&self, name: str)-> DB {

    }
}
