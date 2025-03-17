use chrono::prelude::*;
use rayon::prelude::{IntoParallelRefIterator, IntoParallelRefMutIterator};
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};

use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeRecord {
    pub geo_location: String,
    pub utc: DateTime<Utc>,
    pub is_latest: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct HashMapIpSearcher {
    pub inner: HashMap<Ipv4Addr, Vec<NodeRecord>>,
}

impl HashMapIpSearcher {
    pub fn new() -> Self {
        HashMapIpSearcher { inner: HashMap::new() }
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
}
