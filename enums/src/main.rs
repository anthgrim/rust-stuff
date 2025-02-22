#[derive(Debug)]
enum IpAddressKind {
    V4,
    V6,
}

struct IpAddress {
    kind: IpAddressKind,
    address: String,
}

fn main() {
    let four = IpAddressKind::V4;
    let six = IpAddressKind::V6;

    route(four);
    route(six);

    // Using structs

    let home = IpAddress {
        kind: IpAddressKind::V4,
        address: String::from("127.0.0.1"),
    };

    retrieve_ip_address(home)
}

fn route(address: IpAddressKind) {
    println!("routing for {:?}", address);
}

fn retrieve_ip_address(address: IpAddress) {
    println!("kind {:?}", address.kind);
    println!("address {}", address.address);
}
