use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;
use tokio::task;

use data_encoding::BASE64;

async fn communication_client(
    mut socket: tokio::net::TcpStream,
    line: &mut String,
    port: u32,
    socket_addr: String,
) -> String {
    let (reader, _) = socket.split(); // tokio socket split to read and write concurrently

    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let text = ["server at port".to_string(), port.to_string()].join(": ");

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    let mut _decoded_data = Vec::new(); // Declare decoded_data outside the loop

    loop {
        let _bytes_read: usize = reader.read_line(line).await.unwrap();

        if line.contains("EOF") {
            _decoded_data = BASE64.decode(line.trim().as_bytes()).unwrap();
            break;
        }

        line.clear();
    }

    let decoded_string: String = String::from_utf8_lossy(&_decoded_data).to_string();

    file.write_all(decoded_string.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    let socket_ip: Vec<&str> = socket_addr.split(":").collect();

    line.push_str("/");
    line.push_str(socket_ip[0]);

    line.clone()
}

#[tokio::main]
pub async fn handle_server(_ip_address: Vec<&str>, port: u32, testport: u32) -> String {
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":"))
        .await
        .unwrap(); // open connection

    let test_listener =
        TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":"))
            .await
            .unwrap();

    let (_, _) = test_listener.accept().await.unwrap();

    let mut line = String::new();

    loop {
        let (socket, socket_addr) = listener.accept().await.unwrap(); // accept listening
        let result = communication_client(socket, &mut line, port, socket_addr.to_string()).await;

        println!("Result: {}", result);
    }
}
