use crate::response_parser::{self, GopherMap, GopherParseError, GopherResponse, ItemType};
use tokio::net::TcpStream;
use tokio::prelude::*;
/// Represents a browsing session, complete with history.
/// Eventually, these types will be more rich and have more context. For now, they are just strings
/// representing the various paths that have been visited.
pub struct Session {
    /// The current location of the browser
    selector: String,
    /// The most recent [GopherMap].
    options: GopherMap,
    /// The current hostname
    hostname: String,
    /// The current port
    port: usize,
}

impl Session {
    pub async fn new(hostname: &str, port: usize) -> Session {
        let mut sess = Session {
            selector: "\n".to_string(),
            options: GopherMap::new(),
            hostname: hostname.to_string(),
            port,
        };
        println!("Connecting to {}:{}", hostname, port);
        sess.connect().await;
        println!("Connected.");
        return sess;
    }

    /// Right now this can only browse to other gopher maps
    pub async fn connect(&mut self) {
        let mut stream = TcpStream::connect(&format!("{}:{}", self.hostname, self.port))
            .await
            .unwrap();
        stream
            .write(format!("{}", self.selector).as_bytes())
            .await
            .unwrap();
        let mut buf = String::new();
        let byte_count = stream.read_to_string(&mut buf).await.unwrap();
        println!(
            "Read {} bytes from {}:{}",
            byte_count, self.hostname, self.port
        );
        stream
            .shutdown(std::net::Shutdown::Both)
            .expect("failed to end tcp stream");
        self.options = GopherMap::from_str(&buf).unwrap();
    }

    pub async fn browse_to(
        &mut self,
        hostname: String,
        port: usize,
        selector: String,
        item_type: ItemType,
    ) -> Result<(), GopherParseError> {
        println!(
            "Browsing to {}:{}{} ({})",
            hostname,
            port,
            selector,
            item_type.to_string()
        );
        self.hostname = hostname;
        self.port = port;
        self.selector = format!("{}\n", selector);
        self.connect().await;
        Ok(())
    }
    pub fn show_options(&self) -> String {
        println!("Your options are:");
        self.options.render()
    }
    pub async fn select_option(&mut self, selection: usize) {
        let new_selection = self.options.select_option(selection);
        println!(
            "Browsing to: {} ({}:{}{})",
            new_selection.display_string,
            new_selection.hostname,
            new_selection.port,
            new_selection.selector
        );
        self.browse_to(
            new_selection.hostname.clone(),
            new_selection.port.clone(),
            new_selection.selector.clone(),
            new_selection.item_type.clone(),
        )
        .await
        .unwrap();
    }
}
