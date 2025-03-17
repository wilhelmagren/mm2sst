use crate::NodeRecord;

use rayon::prelude::IntoParallelRefIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};

use std::collections::BTreeMap;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Serialize)]
pub struct BTreeMapIpSearcher {
    inner: BTreeMap<Ipv4Addr, Vec<NodeRecord>>,
}

impl BTreeMapIpSearcher {
    pub fn new() -> Self {
        BTreeMapIpSearcher { inner: BTreeMap::new() }
    }

    pub fn from_btreemap(btmap: BTreeMap<Ipv4Addr, Vec<NodeRecord>>) -> Self {
        BTreeMapIpSearcher { inner: btmap }
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
