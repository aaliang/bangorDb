pub mod format;

use std::io::{Read, BufReader, Write, BufWriter, Error};
use std::fs::{create_dir_all, File, OpenOptions};
use std::collections::HashMap;
use std::path::Path;

pub struct DBIO {
    fd_cache: LRUCache<BufWriter<File>>
}

impl DBIO {
    pub fn new() -> DBIO {
        DBIO {
            fd_cache: LRUCache::new()
        }
    }

    /// appends data to the file at [`path`]. The file and any parent directories will be created if they
    /// do not exist.
    pub fn append(&mut self, path: &str, data: &[u8]) -> Result<(), Error> {
        if let Some(ref mut fd) = self.fd_cache.get_mut(path) {
            return fd.write_all(data)
        }

        let p = Path::new(path);
        let file = try!(match p.exists() {
            false => {
                let parent = p.parent().unwrap();
                try!(create_dir_all(parent));
                File::create(path)
            },
            true => OpenOptions::new().append(true).open(path)
        });

        let mut bw = BufWriter::new(file);
        try!(bw.write_all(data));
        Ok(self.fd_cache.insert(path.to_owned(), bw))
    }
}

//not actually LRU... just stores things indefinitely for now
struct LRUCache<A> {
    map: HashMap<String, A>
}

impl <A> LRUCache<A> {

    pub fn new() -> LRUCache<A> {
        LRUCache {
            map: HashMap::new()
        }
    }

    pub fn get_mut(&mut self, name: &str) -> Option<&mut A> {
        self.map.get_mut(name)
    }
    pub fn insert(&mut self, name: String, val: A) {
        self.map.insert(name, val);
    }

    pub fn rm(&mut self, name: &str) {
        self.map.remove(name);
    }
}
