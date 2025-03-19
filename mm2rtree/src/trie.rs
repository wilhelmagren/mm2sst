use std::net::Ipv4Addr;

/// Implementation of a generic Trie node.
#[derive(Debug, Clone)]
pub struct IpTrieNode<V: Copy> {
    l: Option<Box<IpTrieNode<V>>>,
    r: Option<Box<IpTrieNode<V>>>,
    v: Option<V>,
}

impl<V: Copy> IpTrieNode<V> {
    /// Create a new empty node.
    pub fn new() -> Self {
        IpTrieNode { l: None, r: None, v: None }
    }
    /// Create a new empty node.
    pub fn empty() -> Self {
        IpTrieNode::new()
    }

    pub fn insert(&mut self, ip: u32, mask: u32, value: V) {
        // this is the mask `1000 0000 0000 0000 0000 0000 0000 0000` for the first bit
        let bit: u32 = 0x80000000;
        if mask == 0 {
            self.v = Some(value);
            return;
        };

        let next_node = if (ip & bit) == 0 { &mut self.l } else {&mut self.r };

        match next_node {
            Some(n) => n.insert(ip << 1, mask << 1, value),
            None => {
                // If there was no node then we need to create a new one and insert to,
                // also link it to the current node.
                let mut new_node = IpTrieNode::<V> { l: None, r: None, v: None };
                new_node.insert(ip << 1, mask << 1, value);
                *next_node = Some(Box::new(new_node));
            }
        };
    }

    pub fn get(&self, key: u32, mask: u32) -> Option<V> {
        self._get(key, mask, None)
    }

    fn _get(&self, ip: u32, mask: u32, value: Option<V>) -> Option<V> {
        let bit: u32 = 0x80000000;
        if mask == 0 {
            return self.v.or(value);
        };

        let next_node = if (ip & bit) == 0 { &self.l } else { &self.r };
        match next_node {
            Some(n) => n._get(ip << 1, mask << 1, self.v.or(value)),
            None => self.v.or(value),
        }
    }
}

impl<V: Copy> Default for IpTrieNode<V> {
    fn default() -> Self {
        IpTrieNode::new()
    }
}

/// THIS WAS OLD; I DID NOT DO A RADIX TRIE, ONLY A TRIE.
///
/// Implementation of a generic Radix Tree.
///
/// A `radix tree` is a data structure that represents a space-optimized trie (prefix tree)
/// in which each node that is the only child is merged with its parent. The result
/// is that the number of children of every internal node is at most the radix `r` of
/// the radix tree, where r=2^x for some integer x >= 1.
///
/// Unlike regular trees, edges can be labeled with sequences of elements as well as single
/// elements. This makes radix trees much more efficient for small sets and for sets of strings
/// that share long prefixes.
///
/// Unliky regular trees, the key at each node is compared chunk-of-bits by chunk-of-bits, where
/// the quantity of bits in that chunk at that node is the radix `r` of the radix trie. When `r` is
/// 2 the radix trie is binary which minimizes sparseness at the expense of maximizing trie depth.
///
/// As an optimization, edge labels can be stored in constant size by using two pointers to a
/// string (for the first and last elements).
///
/// Radix trees support insertion, deletion, and searching operations. Insertion adds a new string
/// to the trie while trying to minimize the amount of data stored. Deletion removes a string from
/// the trie. Searching operations include exact lookup, find predecessor, find successor, and find
/// all strings with a prefix. All of these operations are O(k) where k is the maximum length of
/// all strings in the set, where length is measured in the quantity of bits equal to the radix of
/// the radix trie. For IP addresses, we know that the maxmimum length k is either 32 or 128
/// (depending on IPv4 or IPv6), hence we have deterministic lookup time of O(1), however, lookup
/// will obviously take longer for IPv6 compared to IPv4, but they scale similarly.
///
///


#[derive(Debug, Clone)]
pub struct IpTrie<V: Copy> {
    root: IpTrieNode<V>,
}

impl<V: Copy> IpTrie<V> {
    /// Create a new empty trie.
    pub fn new() -> Self {
        IpTrie { root: IpTrieNode::new() }
    }

    pub fn add_ip_and_mask(&mut self, ip: u32, mask: u32, value: V) {
        self.root.insert(ip, mask, value);
    }

    pub fn add_cidr(&mut self, cidr: Ipv4CidrRange, value: V) {
        let mask: u32 = 0xffffffffu32 & !((1u32 << (32 - cidr.mask)) - 1);
        self.root.insert(cidr.ip, mask, value);
    }

    pub fn contains_ip(&self, addr: Ipv4Addr) -> bool {
        self.root.get(u32::from(addr), 0xffffffff).is_some()
    }

    pub fn get(&self, addr: Ipv4Addr) -> Option<V> {
        self.root.get(u32::from(addr), 0xffffffff)
    }
}

/// CIDR range for IPv4, so we work with unsigned 32 bit integers.
#[derive(Debug)]
pub struct Ipv4CidrRange {
    ip: u32,
    mask: u32,
}

impl Ipv4CidrRange {
    pub fn new(ip: u32, mask: u32) -> Self {
        Ipv4CidrRange { ip, mask, }
    }
}
