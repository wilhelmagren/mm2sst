pub mod trie;
pub use trie::IpTrie;
pub use trie::Ipv4CidrRange;

use rand::Rng;

use std::net::Ipv4Addr;

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

fn main() {
    let mut t: IpTrie<u32> = IpTrie::new();
    println!("{:?}", t);

    let (ip, mask) = xd("10.1.1.0/24");
    let (ip2, _) = xd("10.1.1.54/10");
    let (ip3, _) = xd("10.1.1.23/10");

    println!("{}, {}", ip, ip2);

    // t.add_ip_and_mask(ip, mask, 123);
    t.add_cidr(Ipv4CidrRange::new(ip, mask), 123);
    println!("trie contains? {}", t.contains_ip(ip2.into()));
    let v1 = t.get(ip2.into());
    let v2 = t.get(ip3.into());

    println!("{:?} {:?}", v1, v2);
}
