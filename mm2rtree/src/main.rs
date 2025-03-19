pub mod trie;
pub use trie::IpTrie;

fn main() {
    println!("Hello, world!");
    let mut t: IpTrie<u32> = IpTrie::new();
    println!("{:?}", t);

    t.add_ip_and_mask(12848778, 0xff, 123);
    println!("{:?}", t);

}
