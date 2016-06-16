use nest::net::*;

#[test]
pub fn test_NsIpAddr() {
    let ip = NsIpAddr::V4(NsIpv4Addr::new(127, 0, 0, 1));

    assert_eq!(ip.to_string(), "127.0.0.1".to_string());
}
