use crate::test::tsa;
use crate::*;

#[test]
fn to_socket_addr_ipaddr_u16() {
    let a = Ipv4Addr::new(77, 88, 21, 11);
    let p = 12345;
    let e = SocketAddr::V4(SocketAddrV4::new(a, p));
    assert_eq!(Ok(vec![e]), tsa((a, p)));
}

#[test]
fn set_ip() {
    fn ip4(low: u8) -> Ipv4Addr {
        Ipv4Addr::new(77, 88, 21, low)
    }
    fn ip6(low: u16) -> Ipv6Addr {
        Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, low)
    }

    let mut v4 = SocketAddrV4::new(ip4(11), 80);
    assert_eq!(v4.ip(), &ip4(11));
    v4.set_ip(ip4(12));
    assert_eq!(v4.ip(), &ip4(12));

    let mut addr = SocketAddr::V4(v4);
    assert_eq!(addr.ip(), IpAddr::V4(ip4(12)));
    addr.set_ip(IpAddr::V4(ip4(13)));
    assert_eq!(addr.ip(), IpAddr::V4(ip4(13)));
    addr.set_ip(IpAddr::V6(ip6(14)));
    assert_eq!(addr.ip(), IpAddr::V6(ip6(14)));

    let mut v6 = SocketAddrV6::new(ip6(1), 80, 0, 0);
    assert_eq!(v6.ip(), &ip6(1));
    v6.set_ip(ip6(2));
    assert_eq!(v6.ip(), &ip6(2));

    let mut addr = SocketAddr::V6(v6);
    assert_eq!(addr.ip(), IpAddr::V6(ip6(2)));
    addr.set_ip(IpAddr::V6(ip6(3)));
    assert_eq!(addr.ip(), IpAddr::V6(ip6(3)));
    addr.set_ip(IpAddr::V4(ip4(4)));
    assert_eq!(addr.ip(), IpAddr::V4(ip4(4)));
}

#[test]
fn set_port() {
    let mut v4 = SocketAddrV4::new(Ipv4Addr::new(77, 88, 21, 11), 80);
    assert_eq!(v4.port(), 80);
    v4.set_port(443);
    assert_eq!(v4.port(), 443);

    let mut addr = SocketAddr::V4(v4);
    assert_eq!(addr.port(), 443);
    addr.set_port(8080);
    assert_eq!(addr.port(), 8080);

    let mut v6 = SocketAddrV6::new(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 80, 0, 0);
    assert_eq!(v6.port(), 80);
    v6.set_port(443);
    assert_eq!(v6.port(), 443);

    let mut addr = SocketAddr::V6(v6);
    assert_eq!(addr.port(), 443);
    addr.set_port(8080);
    assert_eq!(addr.port(), 8080);
}

#[test]
fn set_flowinfo() {
    let mut v6 = SocketAddrV6::new(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 80, 10, 0);
    assert_eq!(v6.flowinfo(), 10);
    v6.set_flowinfo(20);
    assert_eq!(v6.flowinfo(), 20);
}

#[test]
fn set_scope_id() {
    let mut v6 = SocketAddrV6::new(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 80, 0, 10);
    assert_eq!(v6.scope_id(), 10);
    v6.set_scope_id(20);
    assert_eq!(v6.scope_id(), 20);
}

#[test]
fn is_v4() {
    let v4 = SocketAddr::V4(SocketAddrV4::new(Ipv4Addr::new(77, 88, 21, 11), 80));
    assert!(v4.is_ipv4());
    assert!(!v4.is_ipv6());
}

#[test]
fn is_v6() {
    let v6 = SocketAddr::V6(SocketAddrV6::new(
        Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1),
        80,
        10,
        0,
    ));
    assert!(!v6.is_ipv4());
    assert!(v6.is_ipv6());
}

#[test]
fn socket_v4_to_str() {
    let socket = SocketAddrV4::new(Ipv4Addr::new(192, 168, 0, 1), 8080);

    assert_eq!(format!("{}", socket), "192.168.0.1:8080");
    assert_eq!(format!("{:<20}", socket), "192.168.0.1:8080    ");
    assert_eq!(format!("{:>20}", socket), "    192.168.0.1:8080");
    assert_eq!(format!("{:^20}", socket), "  192.168.0.1:8080  ");
    assert_eq!(format!("{:.10}", socket), "192.168.0.");
}

#[test]
fn socket_v6_to_str() {
    let mut socket = SocketAddrV6::new(Ipv6Addr::new(0x2a02, 0x6b8, 0, 1, 0, 0, 0, 1), 53, 0, 0);

    assert_eq!(format!("{}", socket), "[2a02:6b8:0:1::1]:53");
    assert_eq!(format!("{:<24}", socket), "[2a02:6b8:0:1::1]:53    ");
    assert_eq!(format!("{:>24}", socket), "    [2a02:6b8:0:1::1]:53");
    assert_eq!(format!("{:^24}", socket), "  [2a02:6b8:0:1::1]:53  ");
    assert_eq!(format!("{:.15}", socket), "[2a02:6b8:0:1::");

    socket.set_scope_id(5);

    assert_eq!(format!("{}", socket), "[2a02:6b8:0:1::1%5]:53");
    assert_eq!(format!("{:<24}", socket), "[2a02:6b8:0:1::1%5]:53  ");
    assert_eq!(format!("{:>24}", socket), "  [2a02:6b8:0:1::1%5]:53");
    assert_eq!(format!("{:^24}", socket), " [2a02:6b8:0:1::1%5]:53 ");
    assert_eq!(format!("{:.18}", socket), "[2a02:6b8:0:1::1%5");
}

#[test]
fn compare() {
    let v4_1 = "224.120.45.1:23456".parse::<SocketAddrV4>().unwrap();
    let v4_2 = "224.210.103.5:12345".parse::<SocketAddrV4>().unwrap();
    let v4_3 = "224.210.103.5:23456".parse::<SocketAddrV4>().unwrap();
    let v6_1 = "[2001:db8:f00::1002]:23456"
        .parse::<SocketAddrV6>()
        .unwrap();
    let v6_2 = "[2001:db8:f00::2001]:12345"
        .parse::<SocketAddrV6>()
        .unwrap();
    let v6_3 = "[2001:db8:f00::2001]:23456"
        .parse::<SocketAddrV6>()
        .unwrap();

    // equality
    assert_eq!(v4_1, v4_1);
    assert_eq!(v6_1, v6_1);
    assert_eq!(SocketAddr::V4(v4_1), SocketAddr::V4(v4_1));
    assert_eq!(SocketAddr::V6(v6_1), SocketAddr::V6(v6_1));
    assert!(v4_1 != v4_2);
    assert!(v6_1 != v6_2);

    // compare different addresses
    assert!(v4_1 < v4_2);
    assert!(v6_1 < v6_2);
    assert!(v4_2 > v4_1);
    assert!(v6_2 > v6_1);

    // compare the same address with different ports
    assert!(v4_2 < v4_3);
    assert!(v6_2 < v6_3);
    assert!(v4_3 > v4_2);
    assert!(v6_3 > v6_2);

    // compare different addresses with the same port
    assert!(v4_1 < v4_3);
    assert!(v6_1 < v6_3);
    assert!(v4_3 > v4_1);
    assert!(v6_3 > v6_1);

    // compare with an inferred right-hand side
    assert_eq!(v4_1, "224.120.45.1:23456".parse().unwrap());
    assert_eq!(v6_1, "[2001:db8:f00::1002]:23456".parse().unwrap());
    assert_eq!(SocketAddr::V4(v4_1), "224.120.45.1:23456".parse().unwrap());
}
