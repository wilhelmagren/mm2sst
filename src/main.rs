pub mod hashmap;
pub use hashmap::{HashMapIpSearcher, NodeRecord};

use chrono::prelude::*;
use rand::Rng;
use serde::{Serialize, Deserialize};

use std::net::Ipv4Addr;
use std::time::Instant;

pub fn generate_ips(n: usize) -> Vec<Ipv4Addr> {
    let mut thread_rng = rand::rng();
    (0..n)
        .map(|_| Ipv4Addr::from_bits(thread_rng.random_range(0u32..u32::MAX)))
        .collect()
}

fn main() {
    println!("Creating new IpSearcher...");
    let mut hm_ip_s = HashMapIpSearcher::new();

    println!("Generating 10_000_000 random Ipv4 addresses...");
    let qs = generate_ips(10_000_000);
    println!("Ok, inserting to tree...");

    for ip in &qs {
        hm_ip_s.insert(
            *ip,
            NodeRecord {
                geo_location: "Stockholm, Sweden".to_string(),
                utc: Utc::now(),
                is_latest: true,
            },
        );
    }
    println!("Ok, now querying tree...");

    let start = Instant::now();
    let _ = hm_ip_s.queries(&qs);
    let elapsed = start.elapsed();
    println!(
        "Elapsed {:?} ms for {:?} ip lookups, {:?} ns per lookup",
        elapsed.as_millis() as f64,
        qs.len(),
        elapsed.as_nanos() as f64 / qs.len() as f64,
    );
}
