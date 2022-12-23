use super::SyncTrigger;
use crate::irc;
use crate::irc::Command;
use once_cell::sync::Lazy;
use std::sync::mpsc::Sender;

pub static PING: SyncTrigger = SyncTrigger {
    cond: Lazy::new(|| {
        Box::new(|_: &Sender<irc::Message>, msg: &irc::Message| msg.command == Command::PING)
    }),
    act: Lazy::new(|| {
        Box::new(|tx: &Sender<irc::Message>, msg: &irc::Message| {
            let resp = irc::Message {
                command: Command::PONG,
                params: msg.params.clone(),
            };
            tx.send(resp).unwrap();
            false
        })
    }),
};
