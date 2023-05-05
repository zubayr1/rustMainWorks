use std::error::Error;
use std::io::Read;
use tokio::net::TcpListener;
use std::collections::HashSet;

#[tokio::main]
pub async fn handle_server(server_type: String, ip_address: Vec<String>, args: Vec<String>, self_ip: String, port: u32, epoch: i32, mut blacklisted: HashSet<String>) -> Result<(), Box<dyn Error>> {
    let mut data = [0u8; 12];
    println!("server");
    let mut count=0;
    loop{
        count+=1;
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await?;
    let (tokio_tcp_stream, _) = listener.accept().await?;
    println!("server done");
    let mut std_tcp_stream = tokio_tcp_stream.into_std()?;
    std_tcp_stream.set_nonblocking(false)?;
    std_tcp_stream.read_exact(&mut data)?;
    println!("{:?}", data);

    if count==4
    {
        break;
    }
    }
    Ok(())
}