
use tokio::net::TcpListener;
use tokio::net::tcp::ReadHalf;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use tokio::fs::OpenOptions;

use data_encoding::BASE64;



#[tokio::main]
pub async fn handle_server( _ip_address: Vec<&str>, port: u32, testport: u32) -> String{
   // loop{
    let listener = TcpListener::bind(["0.0.0.0".to_string(), port.to_string()].join(":")).await.unwrap(); // open connection
    
    let test_listener = TcpListener::bind(["0.0.0.0".to_string(), testport.to_string()].join(":")).await.unwrap();

    let (_, _) = test_listener.accept().await.unwrap();

    let (mut socket, socket_addr) = listener.accept().await.unwrap(); // accept listening
    
    socket.set_nodelay(true).expect("Failed to enable TCP_NODELAY");
    
    let (reader, _) = socket.split(); // tokio socket split to read and write concurrently
        
    let mut reader: BufReader<ReadHalf> = BufReader::new(reader);
    let mut line: String  = String :: new();

    let mut file = OpenOptions::new().append(true).open("output.log").await.unwrap();

    let text;

    text = ["server at port".to_string(), port.to_string()].join(": ");

    file.write_all(text.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    let mut _decoded_data = Vec::new(); // Declare decoded_data outside the loop

    loop 
    {         
        let _bytes_read: usize = reader.read_line(&mut line).await.unwrap();

        // if _bytes_read == 0 {
        //     // End of stream, the client has closed the connection.
        //     break;
        // }
    
        if line.contains("EOF")  
        {
            line = line.replace("EOF", "");

            // Decode the Base64 data back to binary format.
            _decoded_data = BASE64.decode(line.trim().as_bytes()).unwrap();                    
            
            break;
        }

        line.clear();
                
    }


    let decoded_string: String = String::from_utf8_lossy(&_decoded_data).to_string();

    file.write_all(decoded_string.as_bytes()).await.unwrap();
    file.write_all(b"\n").await.unwrap();

    let socket_addr_string = socket_addr.to_string();
    let socket_ip: Vec<&str> = socket_addr_string.split(":").collect();


    line = [decoded_string, socket_ip[0].to_string()].join("/");    

    println!("server hit");
    return line;
    
//}
}