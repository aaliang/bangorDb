extern crate bangor;

use bangor::db::DBIO;

fn main() {
    let mut db = DBIO::new();

    for i in 0..1000 {
        let path = format!("/tmp/bangor/{}.db", i);
        db.append(&path, "abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ1234567890".as_bytes());
    }
}
