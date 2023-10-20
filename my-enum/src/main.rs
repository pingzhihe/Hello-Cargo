enum ipAddKind {
    V4,
    V6,
}
struct IpAddr {
    kind: ipAddKind,
    address: String,
}

fn main() {
    let four = ipAddKind::V4;
    let six = ipAddKind::V6;

    let home = IpAddr {
        kind: ipAddKind::V4,
        address: String::from("127.0.0.1"),
    };
    
}
