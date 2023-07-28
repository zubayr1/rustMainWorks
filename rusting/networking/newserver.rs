use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;

#[tokio::main]
pub async fn handle_server(_ip_address: Vec<&str>, port: u32, testport: u32) -> (TcpListener, TcpListener) {
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":"))
        .await
        .unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":"))
        .await
        .unwrap();
    
    
    
    return (listener, test_listener) // Return the listener
}

pub async fn handle_communication(listener: &TcpListener, test_listener: &TcpListener) -> String
{
    let (_, _) = test_listener.accept().await.unwrap();
    // Accept the first client connection
    let (socket, socket_addr) = listener.accept().await.unwrap();
    let socket_addr_string = socket_addr.to_string();
    let socket_ip: Vec<&str> = socket_addr_string.split(":").collect();
    
    // Perform the connection setup and keep the socket
    let mut socket = setup_connection(socket).await;
    
    // Process multiple requests over the same connection
    let response = handle_client_request(&mut socket, &socket_ip[0]).await;

    return response;
}

async fn setup_connection(socket: tokio::net::TcpStream) -> tokio::net::TcpStream {
    // Set TCP_NODELAY and any other necessary setup
    socket.set_nodelay(true).expect("Failed to enable TCP_NODELAY");
    socket
}

async fn handle_client_request(socket: &mut tokio::net::TcpStream, client_ip: &str) -> String {
    let mut reader = BufReader::new(socket);
    let mut line = String::new();

    // Open the file for writing (you may need to handle file access concurrently in production code)
    let mut file = OpenOptions::new()
        .append(true)
        .open("output.log")
        .await
        .unwrap();

    loop 
    {
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

        if line.contains("EOF") {
            line = line.replace("EOF", "");
            break;
        }

        file.write_all(line.as_bytes()).await.unwrap();
        file.write_all(b"\n").await.unwrap();

        line = [line.clone(), client_ip.to_string()].join("/");
    }

    let serialized_data = serde_json::to_string(&line).unwrap();
    let _message_size = serialized_data.len();

    return line;
}
