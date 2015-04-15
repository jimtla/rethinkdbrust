mod rethinkdb;

use rethinkdb::Connection;

#[test]
fn it_works() {
    Connection::connect("localhost", 28015, "OK");
}
