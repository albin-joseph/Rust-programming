fn main() {
    println!("Enum program");

    enum IPAddrKind {
        V4,
        V6,
    } // trailing comma is conventional in Rust

    let four = IPAddrKind::V4;  // you can remove underscore if using the variables
    let six = IPAddrKind::V6;

    fn route(ip_kind: IPAddrKind) {
        match ip_kind {
            IPAddrKind::V4 => println!("IP version 4"),
            IPAddrKind::V6 => println!("IP version 6"),
        }
    }

    route(four);  // use the variables
    route(six);
    // or keep your direct approach: route(IPAddrKind::V4);

    route(IPAddrKind::V4);
    route(IPAddrKind::V6);

    struct IPAddr {
        kind: IPAddrKind,
        address: String
    }

    let home: IPAddr = IPAddr {
        kind: IPAddrKind::V4,
        address: String::from("192.10.0.1"),
    };


    let loopback: IPAddr = IPAddr {
        kind: IPAddrKind::V6,
        address: String::from("::1"),
    };

    enum IPAddress {
        V4(String),
        V6(String),
    }

    let home:IPAddress = IPAddress::V4(String::from("127.0.0.1"));
    let loopback:IPAddress = IPAddress::V4(String::from("::1"));
}