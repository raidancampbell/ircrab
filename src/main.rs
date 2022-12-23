mod bot;
mod irc;
mod triggers;

pub struct Config {
    my_nick: String,
    network: Network,
}
pub struct Network {
    host: String,
    port: u16,
    ssl: bool,
    channel: String,
}

fn initialize() -> Config {
    Config {
        my_nick: "ircrab".to_string(),
        network: Network {
            host: "irc.libera.chat".to_string(),
            port: 6667,
            ssl: false,
            channel: "##cwru-testing".to_string(),
        },
    }
}

fn main() {
    println!("Initializing...");
    let cfg = initialize();
    let b = bot::new(cfg);
    b.run().unwrap();
}
