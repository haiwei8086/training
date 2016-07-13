extern crate nest;

//mod signal;
//mod atomic;

//use std::net;


fn main() {
    // nest_test();
    //signal::run();
    // atomic::run();


    let mut config = nest::NsConfig::new();

    let mut nest = nest::new(&mut config);
    nest.modules(1);

    match nest.listen() {
        Ok(_) => println!("Listening on port: {}", 0),
        _ => println!("Listen failed!"),
    };
}

/*
fn nest_test() {
    println!("Version: {:?}", nest::version());

    let ip = NsIpAddr::V4(NsIpv4Addr::new(127, 0, 0, 1));

    println!("IP: {:?}", ip);
    println!("IP str: {:?}", ip.to_string());

    let add = NsInetAddr::new(ip, 5000);

    let ns_addr = NsSocketAddr::Inet(NsInetAddr::new(ip, 5000));
    let str_addr = NsSocketAddr::Inet(NsInetAddr::from_std(&"127.0.0.1:5000".parse::<net::SocketAddr>().unwrap()));

    println!("Inet addr: {:?}", add);
    println!("Net Socket addr: {:?}", ns_addr);
    println!("Socket addr: {:?}", str_addr);

    let fd = nest::net::socket(NsAddressFamily::Inet, NsSocketTypes::Stream, 0).unwrap();

    println!("Socket Fd: {:?}", fd);

    println!("Get Socket Fd: {:?}", get_flags(fd).unwrap());

    set_nonblocking(fd).unwrap();

    println!("Get Socket Fd: {:?}", get_flags(fd).unwrap());

}
*/

/*
fn stain_test() {
    stain::version();

    let ip = ip::IpAddr::V4(ip::Ipv4Addr::new(127, 0, 0, 1));
    println!("ip v4: {:?}", ip);

    let addr = addr::InetAddr::new(ip, 5000);
    println!("addr v4: {:?}", addr.to_str());

    stain::server::Server::new();
    //stain::server::listen("127.0.0.1:3000");
}
*/
