use tokio::net::{TcpListener, TcpStream};
use tokio::io::{AsyncWriteExt,AsyncReadExt};
use std::str::FromStr;
use uuid::Uuid;

// constants
const ASYNC_SERVER_ADDRESS: &str = "127.0.0.1:8001";
const SYNC_SERVER_ADDRESS: &str = "127.0.0.1:8000";

#[tokio::main]
async fn main() {
    // starting
    println!("async server starting {}", ASYNC_SERVER_ADDRESS);
    
    // bind
    let listener = TcpListener::bind(ASYNC_SERVER_ADDRESS).await.unwrap();

    // starting
    println!("async server listening {}", ASYNC_SERVER_ADDRESS);

    // loop through incoming connections
    loop {
        // accept the connection
        let (socket, _) = listener.accept().await.unwrap();

        tokio::spawn(async move {
            handle_connection(socket).await;
        });
    }
}

async fn handle_connection(mut stream: TcpStream) {
    // create uuid
    let id = Uuid::new_v4();

    // create the buffer
    let mut buffer = [0; 1024];

    // read the stream
    let len = stream.read(&mut buffer).await.unwrap();

    // output the request
    let message = String::from_utf8_lossy(&buffer[..len]);
    println!("{} - received: {}", id, message);

    // call sirocco
    let sync_server_message = call_sync_server(id, message.to_owned().to_string()).await;
    let output = format!("sync server says: {}", sync_server_message);

    // send out message
    let _ = stream.write_all(output.as_bytes()).await;
    println!("{} - sent: {}", id, message);
}

async fn call_sync_server(id:Uuid, message: String) -> String {
    // connecting
    println!("{} - connecting to sirocco: {}",id, SYNC_SERVER_ADDRESS);

    // connected
    if let Ok(mut stream) = TcpStream::connect(SYNC_SERVER_ADDRESS).await {
        // connected message
        println!(
            "{} - connected to sync server: {}:{}",
            id,
            stream.local_addr().unwrap().ip(),
            stream.local_addr().unwrap().port()
        );

        // set our message as hello world
        let _ = stream.write_all(message.as_bytes()).await;
        println!("{} - sent: {}", id, message);
 
        // read the result
        let mut buffer = [0; 1024];
        let len = stream.read(&mut buffer).await.unwrap();
        let message = String::from_utf8_lossy(&buffer[..len]); 
        println!("{} - received from sync server: {}", id, message);

        // return the message
        return message.to_owned().to_string();
    } else {
        println!(
            "{} - couldn't connect to sirocco: {}",
            id,
            SYNC_SERVER_ADDRESS
        );

        // sirocco is not available
        return String::from_str("sync server is not available").unwrap();
    }
}