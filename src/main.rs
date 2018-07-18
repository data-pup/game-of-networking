#[macro_use]
extern crate lazy_static;
extern crate tokio;

use std::net::{IpAddr, Ipv4Addr, SocketAddr};

// use tokio::io;
// use tokio::net::TcpListener;
// use tokio::prelude::*;

static PORT_RANGE_BASE: u16 = 5000;

struct Coordinates {
    x: u8,
    y: u8,
}
struct Dimensions {
    width: u8,
    height: u8,
}

fn _get_coords_addr(coords: &Coordinates, dims: &Dimensions) -> SocketAddr {
    let Coordinates { x, y } = coords;
    let Dimensions { width, .. } = dims;
    let offset: u16 = (x * width + y).into();
    let port = PORT_RANGE_BASE + offset;
    SocketAddr::new(IpAddr::V4(Ipv4Addr::new(127, 0, 0, 1)), port)
}

fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn get_coords_addr_works() {
        let dims = Dimensions {
            width: 8,
            height: 8,
        };
        let test_cases = &[
            (Coordinates { x: 0, y: 0 }, "127.0.0.1:5000"),
            (Coordinates { x: 1, y: 1 }, "127.0.0.1:5009"),
            (Coordinates { x: 7, y: 7 }, "127.0.0.1:5063"),
        ];

        test_cases
            .iter()
            .map(|(coords, addr)| (_get_coords_addr(coords, &dims), addr))
            .map(|(actual, addr)| (actual, addr.parse::<SocketAddr>().unwrap()))
            .for_each(|(actual, expected)| assert_eq!(actual, expected));
    }
}
