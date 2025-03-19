use maxminddb::geoip2;
use rand::Rng;
use rayon::iter::ParallelIterator;
use rayon::prelude::IntoParallelRefIterator;

use std::net::{IpAddr, Ipv4Addr};

pub fn generate_ips(n: usize) -> Vec<IpAddr> {
    let mut thread_rng = rand::rng();
    (0..n)
        .map(|_| Ipv4Addr::from_bits(thread_rng.random_range(0u32..u32::MAX)).into())
        .collect()
}

fn main() {
    let mut args = std::env::args().skip(1);
    let reader: maxminddb::Reader<_> = maxminddb::Reader::open_readfile(
        args.next()
            .ok_or("First argument must be the path to the IP database").unwrap(),
        )
        .unwrap();

    let ips: Vec<IpAddr> = generate_ips(10000000);
    ips.par_iter().for_each(|ip| {
        match reader.lookup::<geoip2::City>(*ip) {
            Ok(c) => println!("[OK ]  ip: {:?}  city: {:?}", ip, c),
            Err(_) => {
                // println!("[ERR]  ip: {:?} {:?}", ip, e);
            },
        };
    });
}
