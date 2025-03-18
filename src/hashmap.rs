use crate::NodeRecord;

use bincode::{Decode, Encode, config};
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

use std::collections::HashMap;
use std::fs::OpenOptions;
use std::io::{BufReader, BufWriter};
use std::net::Ipv4Addr;

#[derive(Debug, Decode, Encode)]
pub struct HashMapIpSearcher {
    pub inner: HashMap<Ipv4Addr, Vec<NodeRecord>>,
}

impl HashMapIpSearcher {
    pub fn new() -> Self {
        HashMapIpSearcher {
            inner: HashMap::new(),
        }
    }

    pub fn from_hashmap(hm: HashMap<Ipv4Addr, Vec<NodeRecord>>) -> Self {
        HashMapIpSearcher { inner: hm }
    }

    pub fn insert(&mut self, ip: Ipv4Addr, n: NodeRecord) {
        if let Some(a) = self.inner.get_mut(&ip) {
            a.push(n);
        } else {
            self.inner.insert(ip, vec![n]);
        }
    }

    pub fn query(&self, q: Ipv4Addr) -> Option<&Vec<NodeRecord>> {
        self.inner.get(&q)
    }

    pub fn queries(&self, qs: &Vec<Ipv4Addr>) -> Vec<Option<&Vec<NodeRecord>>> {
        qs.par_iter().map(|q| self.query(*q)).collect()
    }

    pub fn write_to_file(&self, p: &str) {
        let config = config::standard();
        let file = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(true)
            .open(p)
            .expect("(HashMap) Couldn't open file in write mode...");

        let mut writer = BufWriter::new(file);
        bincode::encode_into_std_write(&self, &mut writer, config).unwrap();
    }

    pub fn from_file(p: &str) -> Self {
        let config = config::standard();
        let file = match OpenOptions::new().read(true).write(false).open(p) {
            Ok(f) => f,
            Err(_) => {
                println!(
                    "(HashMap) {} did not exist, creating empty HashMapIpSearcher...",
                    p
                );
                return HashMapIpSearcher {
                    inner: HashMap::new(),
                };
            }
        };

        let mut reader = BufReader::new(file);
        bincode::decode_from_std_read(&mut reader, config).unwrap()
    }
}
