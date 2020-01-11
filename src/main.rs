use tokio::net::TcpStream;
use tokio::prelude::*;

mod response_parser;
mod session;
use response_parser::{GopherResponse, ItemType};
use session::Session;

#[tokio::main]
async fn main() {
    // Connect to port 6142 on localhost
    let mut session = Session::new("gopher.quux.org", 70usize).await;
    loop {
        println!("{}", session.show_options());
        println!("Where would you like to browse to?");
        let input = get_user_input();
        let selection = str::parse::<usize>(&input.trim()).unwrap();
        session.select_option(selection).await;
    }
}

fn get_user_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
