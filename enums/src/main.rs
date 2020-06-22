// enum IpAddrKind {
//     V4,
//     V6
// }

// struct IpAddr {
//     kind: IpAddrKind,
//     address: String
// }

// enum IpAddrKind {
//     V4(String),
//     V6(String)
// }

enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

struct Ipv4Addr {}
struct Ipv6Addr {}

enum IpAddress {
    V4(Ipv4Addr),
    V6(Ipv6Addr)
}

enum Message {
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32)
}

impl Message {
    fn call(&self) {}
}

fn main() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // let home = IpAddr {
    //     kind: IpAddrKind::V4,
    //     address: String::from("127.0.0.1")
    // };

    // let loopback = IpAddr {
    //     kind: IpAddrKind::V6,
    //     address: String::from("::1")
    // };

    // let home = IpAddrKind::V4(String::from("127.0.0.1"));
    let home = IpAddrKind::V4(127, 0, 0, 1);
    let loopback = IpAddrKind::V6(String::from("::1"));

    let mess = Message::Write(String::from("I am a message"));
    mess.call();

    let some_number = Some(10);
    let some_string = Some("String!");

    let absent_number: Option<i32> = None;



}

fn route(ip_kind: IpAddrKind) {}
