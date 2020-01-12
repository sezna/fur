use crate::response_parser::{GopherMap, GopherParseError, GopherResponse, ItemType};
use tokio::net::TcpStream;
use tokio::prelude::*;
/// Represents a browsing session, complete with history.
/// Eventually, these types will be more rich and have more context. For now, they are just strings
/// representing the various paths that have been visited.
pub struct Session {
    /// The current location of the browser
    selector: String,
    /// Stack of addresses that have been visited (hostname, port, selector)
    history: Vec<(String, usize, String)>,
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
            history: Vec::new(),
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
            .expect("failed to connect to stream");
        stream
            .write(format!("{}", self.selector).as_bytes())
            .await
            .expect("failed to write to stream");
        let mut buf = String::new();
        let byte_count = stream.read_to_string(&mut buf).await.expect("Failed to read from stream");
        println!(
            "Read {} bytes from {}:{}",
            byte_count, self.hostname, self.port
        );
        stream
            .shutdown(std::net::Shutdown::Both)
            .expect("failed to end tcp stream");
        // TODO this needs to switch depending on the item type, not just work on every page.
        self.options = GopherMap::from_str(&buf).expect("failed to parse menu");
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
        self.history
            .push((self.hostname.clone(), self.port, self.selector.clone()));
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
    pub async fn go_back(&mut self) {
        let (hostname, port, selector) = if let Some(stuff) = self.history.pop() {
            stuff
        } else {
            println!("Cannot go back in empty history.");
            return;
        };
        self.hostname = hostname;
        self.port = port;
        self.selector = selector;
        self.connect().await;
        return;
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
        .expect("failed to browse");
    }
}
