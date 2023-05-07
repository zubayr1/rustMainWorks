use tokio::net::TcpStream;
use tokio::io::AsyncWriteExt;
use std::error::Error;
use std::{ time};
use tokio::time::{ sleep, Duration};
use std::panic;
use std::format;
use std::net::{IpAddr, Ipv4Addr, SocketAddr};

#[tokio::main]
pub async fn match_tcp_client(address: String, self_ip: String) -> bool {

    if TcpStream::connect(address.clone()).await.is_ok()
    {
        
    }
}