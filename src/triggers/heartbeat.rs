use super::SyncTrigger;
use crate::irc;
use crate::irc::Command;
use once_cell::sync::Lazy;
use std::sync::mpsc::Sender;

// TODO(raidancampbell): can Lazy be removed here and still retain this single instance usage?
pub static HEARTBEAT: SyncTrigger = SyncTrigger {
    cond: Lazy::new(|| {
        Box::new(|_: &Sender<irc::Message>, msg: &irc::Message| {
            msg.command == Command::PRIVMSG && msg.params.len() > 1 && msg.params[1] == *"!ping"
        })
    }),
    act: Lazy::new(|| {
        Box::new(|tx: &Sender<irc::Message>, msg: &irc::Message| {
            let resp = irc::Message {
                command: Command::PRIVMSG,
                params: vec![msg.params[0].clone(), "pong!".to_string()],
            };
            tx.send(resp).unwrap();
            false
        })
    }),
};
