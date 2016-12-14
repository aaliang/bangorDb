extern crate bangor;

use bangor::db::DBIO;

fn main() {
    let mut db = DBIO::new();
    let result = db.append("/tmp/test_bangor_file", "hello, world".as_bytes());
    println!("{:?}", result);
}
