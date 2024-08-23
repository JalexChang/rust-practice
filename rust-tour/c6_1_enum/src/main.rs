struct V4Addr {
    s: [u8; 4],
}

impl V4Addr {
    fn new(a: u8, b: u8, c: u8, d: u8) -> Self {
        Self { s: [a, b, c, d] }
    }

    fn address(&self) -> String {
        format!("{}.{}.{}.{}", self.s[0], self.s[1], self.s[2], self.s[3])
    }
}

struct V6Addr {
    s: String,
}

impl V6Addr {
    fn new(s: &str) -> Self {
        Self { s: String::from(s) }
    }

    fn address(&self) -> String {
        format!("{}", self.s)
    }
}


enum IpAddrKind {
    V4(V4Addr),
    V6(V6Addr),
    // None is a placeholder for no IP address
    None,
}

impl IpAddrKind {
    // route prints the address of the IP address, depending on the kind of IP address.
    fn route(&self) {
        match self {
            // Destructure the enum, and call the address method of the V4Addr instance.
            IpAddrKind::V4(v4_addr) => println!("V4: {}", v4_addr.address()),
            // Destructure the enum, and call the address method of the V6Addr instance.
            IpAddrKind::V6(v6_addr) => println!("V6: {}", v6_addr.address()),
            // No address to print.
            IpAddrKind::None => println!("None"),
        }
    }
}

fn main() {
    // Enum
    let four = IpAddrKind::V4(V4Addr::new(127, 0, 0, 1));
    let six = IpAddrKind::V6(V6Addr::new("::1"));
    let none = IpAddrKind::None;

    four.route();
    six.route();
    none.route();

    // Option
    let some_number = Some(5);
    let some_string = Some("a string");
    let absent_number: Option<i32> = None;
    println!("{some_number:?}, {some_string:?}, {absent_number:?}");

    // match
    let x: i32 = 5;
    let y: Option<i32> = Some(6);
    match y {
        Some(i) if i == x => println!("equal"),
        None => println!("None"),
        // default case
        _ => println!("not equal"),
    }
}

