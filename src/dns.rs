use crate::error;
use std::net::{self, ToSocketAddrs};

pub fn dns_query(domain: &str) -> net::SocketAddr {
    let mut addrs_iter = format!("{}:443", domain)
        .to_socket_addrs()
        .unwrap_or_else(|_| {
            error::error_and_quit(format!("nameserver parse into SocketAddr failed").as_ref())
        });
    while let Some(sock) = addrs_iter.next() {
        match sock {
            net::SocketAddr::V4(_) => return sock,
            net::SocketAddr::V6(_) => continue,
        }
    }
    error::error_and_quit(format!("nameserver answer failed").as_ref());
}
