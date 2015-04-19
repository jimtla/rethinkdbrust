 #![feature(core)]
 #[warn(unused_imports)]
extern crate byteorder;
extern crate rustc_serialize;
extern crate pool;

mod api;
mod rethinkdb;
mod ql2;
mod test;

use rethinkdb::*;
use api::*;
use ql2::*;
