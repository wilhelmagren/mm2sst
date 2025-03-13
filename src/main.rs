use chrono::prelude::*;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::net::Ipv4Addr;

#[derive(Debug, Deserialize, Serialize)]
pub struct NodeRecord {
    geo_location: String,
    utc: DateTime<Utc>,
    is_latest: bool,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpNode {
    records: Vec<NodeRecord>,
}

#[derive(Debug, Deserialize, Serialize)]
pub struct IpSearcher {
    inner: HashMap<Ipv4Addr, IpNode>,
}

impl IpSearcher {
    pub fn new() -> Self {
        IpSearcher { inner: HashMap::new() }
    }

    pub fn insert(&mut self, ip: Ipv4Addr, n: NodeRecord) {
        if let Some(_) = self.inner.get(&ip) {
            self.inner.get_mut(&ip).unwrap().records.push(n);
        } else {
            self.inner.insert(ip, IpNode { records: vec![n] });
        }
    }

    pub fn from_hashmap(hm: HashMap<Ipv4Addr, IpNode>) -> Self {
        IpSearcher { inner: hm }
    }

    pub fn query(&self, q: Ipv4Addr) -> Option<&IpNode> {
        self.inner.get(&q)
    }

    pub fn queries(&self, q: Vec<Ipv4Addr>) -> Vec<Option<&IpNode>> {
        q.iter().map(|q| self.query(*q)).collect()
    }
}

fn main() {
    println!("Hello, world!");

    let mut ips = IpSearcher::new();
    ips.insert(
        Ipv4Addr::new(10, 51, 3, 41),
        NodeRecord {
            geo_location: "Stockholm, Sweden".to_string(),
            utc: Utc::now(),
            is_latest: true,
        },
    );
    println!("{:?}", ips);

    let q = vec![Ipv4Addr::new(10, 51, 3, 41), Ipv4Addr::new(127, 0, 0, 1)];
    let r = ips.queries(q);
    println!("{:?}", r);

    let ser = serde_json::to_string(&ips).unwrap();
    println!("{}", ser);
}
