#[macro_use]
extern crate lazy_static;
extern crate tokio;

use std::net::{
    IpAddr,
    Ipv4Addr,
    // SocketAddr
};

// use tokio::io;
// use tokio::net::TcpListener;
// use tokio::prelude::*;

static _PORT_RANGE_BASE: u32 = 5000;

lazy_static! {
    static ref _LOCALHOST: IpAddr = IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1));
}

fn main() {
    println!("Hello, world!");
}
