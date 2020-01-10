use tokio::net::TcpStream;
use tokio::prelude::*;

#[tokio::main]
async fn main() {
    // Connect to port 6142 on localhost
    println!("What address would you like to connect to?");
    let addr = "gopher.quux.org:70";
    println!("Assuming port 70 as gopher connection port...");
    let mut stream = TcpStream::connect(&addr).await.unwrap();
    println!("Connected to {:?}. Sending initializer newline", addr);
    stream.write(b"\n").await.unwrap();
    let mut buf = String::new();
    stream.read_to_string(&mut buf).await;

    println!("Received: \n{}", buf);
    // Following snippets come here...
}




fn get_user_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
