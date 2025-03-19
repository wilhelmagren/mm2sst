pub fn cidr2u64range(cidr: &str) -> (u64, u64) {
    let mut parts = cidr.split("/");
    let ip = parts.next().unwrap();
    let prefix: u32 = parts.next().unwrap().parse().unwrap();

    println!("{ip}, {prefix}");

    let ip_parts: Vec<u64> = ip.split(".").into_iter()
        .map(|p| p.parse().unwrap()).collect();

    println!("{:?}", ip_parts);

    let mut ipint: u64 = 0;

    for (i, num) in ip_parts.iter().enumerate() {
        ipint += num * (256u64.pow(3u32 - (i as u32)) as u64);
    }

    (ipint, ipint + 2u64.pow(prefix))
}

fn main() {
    println!("{}, {:?}", "202.196.224.0/20", cidr2u64range("202.196.224.0/20"));
}
