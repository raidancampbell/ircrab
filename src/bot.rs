use crate::triggers::Trigger;
use crate::{irc, triggers, Config};
use mpsc::{Receiver, Sender};
use std::io::{BufRead, BufReader, LineWriter, Write};
use std::net::TcpStream;
use std::sync::mpsc;
use std::thread;

pub struct Bot {
    cfg: Config,
    tx: Sender<irc::Message>,
    rx: Receiver<irc::Message>,
}

pub(crate) fn new(cfg: Config) -> Bot {
    let (tx, rx): (Sender<irc::Message>, Receiver<irc::Message>) = mpsc::channel();
    Bot { cfg, tx, rx }
}

impl Bot {
    pub fn run(self) -> std::io::Result<()> {
        let stream = TcpStream::connect((self.cfg.network.host, self.cfg.network.port))?;
        let mut writer = LineWriter::new(stream.try_clone()?);
        let mut reader = BufReader::new(stream);

        thread::spawn(move || Self::do_write(&mut writer, self.rx));

        self.tx
            .send(irc::Message {
                command: irc::Command::USER,
                params: vec![
                    self.cfg.my_nick.clone(),
                    "0".to_string(),
                    "*".to_string(),
                    "rust-irc-bot".to_string(),
                ],
            })
            .unwrap();

        self.tx
            .send(irc::Message {
                command: irc::Command::NICK,
                params: vec![self.cfg.my_nick.clone()],
            })
            .unwrap();
        loop {
            Bot::do_read(&self.tx, &mut reader);
        }
    }

    fn do_read(tx: &Sender<irc::Message>, reader: &mut BufReader<TcpStream>) {
        loop {
            let mut line = String::new();
            reader.read_line(&mut line).unwrap();
            line.pop(); // Remove trailing \n
            let msg = irc::parse_message(&line);
            match msg {
                Ok(m) => {
                    if m.command != irc::Command::PING {
                        println!("Received message: {}", &line);
                    }
                    for trigger in &[
                        &triggers::on_connect::ON_CONNECT,
                        &triggers::heartbeat::HEARTBEAT,
                        &triggers::ping::PING,
                    ] {
                        // only run the action if the condition matches
                        // if the action returns false, no need to run other actions on this message
                        if trigger.condition(tx, &m) && !trigger.action(tx, &m) {
                            continue;
                        }
                    }
                }
                Err(e) => {
                    println!(
                        "failed to parse incoming message: {} with error {}. continuing...",
                        &line, e
                    );
                    continue;
                }
            }
        }
    }

    fn do_write(writer: &mut LineWriter<TcpStream>, rx: Receiver<irc::Message>) {
        loop {
            match rx.recv() {
                Ok(msg) => {
                    // format into 'COMMAND ARG1 ARG2', truncate to 510, and append '/r/n'
                    let mut output = msg.command.as_str().to_string();
                    // TODO(raidancampbell): there's gotta be an easier/cheaper way to build the byte output
                    output.push(' ');
                    output.push_str(msg.params.join(" ").as_str());

                    output.truncate(510);

                    output.push('\r');
                    output.push('\n');
                    if msg.command != irc::Command::PONG {
                        println!("Writing message: {}", &output);
                    }
                    writer.write_all(output.as_bytes()).unwrap();
                }
                Err(e) => {
                    println!("Channel closed with error {}", e);
                    break;
                }
            }
        }
        println!("receiver closing...");
    }
}
