pub mod btreemap;
pub mod hashmap;

pub use btreemap::BTreeMapIpSearcher;
pub use hashmap::HashMapIpSearcher;

use bincode::{Decode, Encode};
use chrono::prelude::*;
use clap::{ArgAction, Parser, ValueEnum, value_parser};
use rand::Rng;

use std::net::Ipv4Addr;
use std::time::Instant;

#[derive(Debug, Decode, Encode)]
pub struct NodeRecord {
    pub geo_location: String,
    pub utc: String,
    pub is_latest: bool,
}

pub fn generate_ips(n: usize) -> Vec<Ipv4Addr> {
    let mut thread_rng = rand::rng();
    (0..n)
        .map(|_| Ipv4Addr::from_bits(thread_rng.random_range(0u32..u32::MAX)))
        .collect()
}

#[derive(ValueEnum, Clone)]
pub enum Algorithm {
    Btreemap,
    Hashmap,
}

#[derive(Parser)]
#[command(
    name = "ips",
    author,
    version,
    about,
    long_about = None,
)]
pub struct Cli {
    #[arg(
        short = 'n',
        long = "n-queries",
        action = ArgAction::Set,
        default_value = "1000000",
        value_parser = value_parser!(usize),
        required = false,
    )]
    n_queries: usize,

    #[arg(
        short = 'a',
        long = "algorithm",
        action = ArgAction::Set,
        default_value = "hashmap",
        value_parser = value_parser!(Algorithm),
        required = false,
    )]
    algorithm: Algorithm,
}

impl Cli {
    pub fn run(&self) {
        let n_queries: usize = self.n_queries;
        println!("Generating {} random Ipv4 addresses...", n_queries);
        let queries = generate_ips(n_queries);

        match &self.algorithm {
            Algorithm::Btreemap => {
                let mut algo = BTreeMapIpSearcher::new();
                println!("(BTreeMap) Inserting Ipv4 addresses...");
                for ip in &queries {
                    algo.insert(
                        *ip,
                        NodeRecord {
                            geo_location: "Stockholm, Sweden".to_string(),
                            utc: Utc::now().to_string(),
                            is_latest: true,
                        },
                    );
                }

                println!("(BTreeMap) Making {} queries...", n_queries);
                let start = Instant::now();
                let _ = algo.queries(&queries);
                let elapsed = start.elapsed();
                println!(
                    "(BTreeMap) Elapsed {:?} ms for {:?} ip lookups, {:?} ns per lookup",
                    elapsed.as_millis() as f64,
                    queries.len(),
                    elapsed.as_nanos() as f64 / queries.len() as f64,
                );
            }
            Algorithm::Hashmap => {
                println!("(HashMap) reading existing data from `hashmap.bin`...");
                let mut algo = HashMapIpSearcher::from_file("hashmap.bin");
                println!("(HashMap) Ok, inserting Ipv4 addresses...");
                for ip in &queries {
                    algo.insert(
                        *ip,
                        NodeRecord {
                            geo_location: "Stockholm, Sweden".to_string(),
                            utc: Utc::now().to_string(),
                            is_latest: true,
                        },
                    );
                }

                println!("(HashMap) Making {} queries...", n_queries);
                let start = Instant::now();
                let _ = algo.queries(&queries);
                let elapsed = start.elapsed();
                println!(
                    "(HashMap) Elapsed {:?} ms for {:?} ip lookups, {:?} ns per lookup",
                    elapsed.as_millis() as f64,
                    queries.len(),
                    elapsed.as_nanos() as f64 / queries.len() as f64,
                );

                println!("(HashMap) writing data to binary file...");
                algo.write_to_file("hashmap.bin");
                println!("(HashMap) Ok, wrote to `hashmap.bin`!");
            }
        }
    }
}

fn main() {
    Cli::parse().run();
}
