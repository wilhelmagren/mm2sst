pub mod trie;
pub use trie::IpTrie;
pub use trie::Ipv4CidrRange;

use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

use std::net::Ipv4Addr;
use std::time::Instant;

pub fn cidr2u32range(cidr: &str) -> (u32, u32) {
    let mut parts = cidr.split("/");
    let ip = parts.next().unwrap();
    let prefix: u32 = parts.next().unwrap().parse().unwrap();

    let ip_parts: Vec<u32> = ip.split(".").into_iter()
        .map(|p| p.parse().unwrap()).collect();

    let mut ipint: u32 = 0;

    for (i, num) in ip_parts.iter().enumerate() {
        ipint += num * (256u32.pow(3u32 - (i as u32)) as u32);
    }

    (ipint, ipint + 2u32.pow(prefix))
}

pub fn xd(cidr: &str) -> (u32, u32) {
    let mut parts = cidr.split("/");
    let ip = parts.next().unwrap();
    let prefix: u32 = parts.next().unwrap().parse().unwrap();

    let ip_parts: Vec<u32> = ip.split(".").into_iter()
        .map(|p| p.parse().unwrap()).collect();

    let mut ipint: u32 = 0;

    for (i, num) in ip_parts.iter().enumerate() {
        ipint += num * (256u32.pow(3u32 - (i as u32)) as u32);
    }

    (ipint, prefix)
}

pub fn generate_ips(n: usize) -> Vec<Ipv4Addr> {
    let mut thread_rng = rand::rng();
    (0..n)
        .map(|_| Ipv4Addr::from_bits(thread_rng.random_range(0u32..u32::MAX)))
        .collect()
}

pub fn generate_cidrs(n: usize) -> Vec<Ipv4CidrRange> {
    let mut thread_rng = rand::rng();
    (0..n)
        .map(|_| Ipv4CidrRange::new(
            thread_rng.random_range(0u32..u32::MAX),
            thread_rng.random_range(4u32..24u32),
        ))
        .collect()
}

fn main() {
    let mut t: IpTrie<u32> = IpTrie::new();
    let n_cidrs: usize = 10_000;

    println!("Generating {} CIDRS", n_cidrs);
    let cidrs = generate_cidrs(n_cidrs);
    // println!("{:?}", cidrs);

    println!("Inserting CIDRS to Trie...");
    // insert the cidrs
    let mut thread_rng = rand::rng();
    for cidr in cidrs.into_iter() {
        t.add_cidr(cidr, thread_rng.random());
    }
    println!("OK!");

    let n_ips: usize = 50_000_000;
    println!("Generating {} ips for lookup...", n_ips);
    let ips = generate_ips(n_ips);

    let start = Instant::now();

    let count: usize = ips.par_iter()
        .map(|ip| {
            let c = t.contains_ip(*ip);
            if c {
                1
            } else {
                0
            }
            }).collect::<Vec<usize>>().into_iter().sum();

    let elapsed = start.elapsed();
    println!(
        "(Trie) Elapsed {:?} ms for {:?} ip lookups, {:?} ns per lookup",
        elapsed.as_millis() as f64,
        ips.len(),
        elapsed.as_nanos() as f64 / ips.len() as f64,
    );

    println!("{} hits", count);

    println!("got {:?} from {}",
        t.get(ips[10]),
        ips[10],
    );
}
