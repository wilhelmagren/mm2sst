use maxminddb::Reader;

fn main() {
    let mut args = std::env::args().skip(1);
    let reader: Reader<Vec<u8>> = Reader::open_readfile(
        args.next()
            .ok_or("First argument must be the path to the .mmdb file").unwrap(),
    )
        .unwrap();
    println!("{:?}", reader.metadata);
}
