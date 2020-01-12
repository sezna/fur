
mod response_parser;
mod session;
use session::Session;

#[tokio::main]
async fn main() {
    let mut session = Session::new("gopher.quux.org", 70usize).await;
    loop {
        println!("{}", session.show_options());
        let selection = loop {
            println!("Where would you like to browse to?");
            let input = get_user_input();
            if input.trim() == "back".to_string() {
                session.go_back().await;
                println!("{}", session.show_options());
            } else {
                if let Ok(selection) = str::parse::<usize>(&input.trim()) {
                    break selection;
                }
                println!("Invalid entry.");
            }
        };
        session.select_option(selection).await;
    }
}

fn get_user_input() -> String {
    let mut buffer = String::new();
    std::io::stdin().read_line(&mut buffer).unwrap();
    buffer
}
