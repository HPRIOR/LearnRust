// Enums allow you to define a type by enumerating its possible variants. These are
// essentially an algebraic data type from functional program


// Example... we need to work with IP addresses, and there are two possible variants
// v4, v6

// Any IP address can be either a version four or six address but not at the same time

enum IpAddrKind {
    V4,
    V6,
}

fn enums() {
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;
}

// these two types can now be used interchangeably
// when passed to a function

fn route(ip_kind: IpAddrKind) {
    // do something w/ ip address
}

fn call_route() {
    route(IpAddrKind::V4);
    route(IpAddrKind::v6);
}

// we would need some data to associate with each type. One solution would be to
// use structs
struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

// or we can be more concise with enums

enum IpAddrWithTypes {
    V4(String),
    V6(String),
}

fn use_enum_with_data() {
    let home = IpAddrWithTypes::V4(String::from("127.0.0.1"));
    let home = IpAddrWithTypes::V6(String::from("::1"));
}

// aside from being concise the enum is more flexible that the struct option
// each value can have different types

enum IpAddrWithDiffTypes {
    V4(u8, u8, u8, u8),
    V6(String),
}

// any type can be used as values, even other enums
struct IpV4Addr {}

struct IpV6Addr {}

enum IpAddrWithStructs {
    V4(IpV4Addr),
    V6(IpV6Addr),
}

// a more complex enum
enum Message {
    Quit,
    // no associated data
    Move { x: i32, y: i32 },
    // includes an anonymous struct
    Write(String),
    // includes a single string
    ChangeColour(i32, i32, i32),    // includes three i32 values
}

// this is similar to defining various kinds of structs, except they are all grouped
// under the message type

// we can also define methods for enums
impl Message {
    fn call(&self) {
        // method body
        match self {
            Message::Quit => {}
            Message::Move { x, y } => {}
            Message::Write(message) => { println!("hello") }
            Message::ChangeColour(x, y, z) => {}
        }
    }
}


fn using_message_method() {
    // m can be one of four options
    let m_write = Message::Write(String::from("hello"));
    let m_move = Message::Move { x: 0, y: 0 };
    let m_quit = Message::Quit;
    let m_change_colour = Message::ChangeColour(0, 0, 0);
    // call method can be called on any of these, with match performing appropriate
    // action
    m_write.call();
}

// fn consume_message(message: Message) -> &str {
//     let result = match message { // will return one type
//         Message::Quit => "",
//         Message::Move { .. } => "",
//         Message::Write(_) => "",
//         Message::ChangeColour(_, _, _) => ""
//     };
//     result
// }


