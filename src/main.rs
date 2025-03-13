use chrono::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use std::collections::HashMap;
use std::net::Ipv4Addr;
use std::time::Instant;

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

    pub fn queries(&self, q: &Vec<Ipv4Addr>) -> Vec<Option<&IpNode>> {
        q.iter().map(|q| self.query(*q)).collect()
    }
}

pub fn generate_ips(n: usize) -> Vec<Ipv4Addr> {
    let mut thread_rng = rand::rng();
    (0..n)
        .map(|_| Ipv4Addr::from_bits(thread_rng.random_range(0u32..u32::MAX)))
        .collect()
}

fn main() {
    println!("Creating new IpSearcher...");
    let mut ips = IpSearcher::new();

    println!("Inserting one record...");
    ips.insert(
        Ipv4Addr::new(10, 51, 3, 41),
        NodeRecord {
            geo_location: "Stockholm, Sweden".to_string(),
            utc: Utc::now(),
            is_latest: true,
        },
    );

    println!("Generating 10_000_000 random Ipv4 addresses...");
    let qs = generate_ips(10_000_000);
    println!("Ok, now querying tree...");

    let start = Instant::now();
    let _ = ips.queries(&qs);
    let elapsed = start.elapsed();
    println!(
        "Elapsed {:?} ms for {:?} ip lookups, {:?} ns per lookup",
        elapsed.as_millis() as f64,
        qs.len(),
        elapsed.as_nanos() as f64 / qs.len() as f64,
    );
}
