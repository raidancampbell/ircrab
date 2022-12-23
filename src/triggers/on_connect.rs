use super::SyncTrigger;
use crate::irc;
use crate::irc::Command;
use once_cell::sync::Lazy;
use std::sync::mpsc::Sender;
use std::{process, thread, time};

pub static ON_CONNECT: SyncTrigger = SyncTrigger {
    cond: Lazy::new(|| {
        Box::new(|_: &Sender<irc::Message>, msg: &irc::Message| msg.command == Command::RPL_WELCOME)
    }),
    act: Lazy::new(|| {
        Box::new(|tx: &Sender<irc::Message>, _: &irc::Message| {
            thread::sleep(time::Duration::from_millis(1000));
            let resp = irc::Message {
                command: Command::JOIN,
                // TODO(raidancampbell): find a way to access the configuration
                //  from here instead of hardcoding
                params: vec!["#cwru".to_string()],
            };
            tx.send(resp).unwrap_or_else(|err| {
                eprintln!("Problem sending onConnect command: {err}");
                process::exit(1);
            });

            false
        })
    }),
};
