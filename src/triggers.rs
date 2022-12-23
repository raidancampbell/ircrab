use super::irc;
use once_cell::sync::Lazy;
use std::sync::mpsc::Sender;
pub mod heartbeat;
pub mod on_connect;
pub mod ping;

pub trait Trigger {
    fn condition(&self, _: &Sender<irc::Message>, _: &irc::Message) -> bool;
    fn action(&self, _: &Sender<irc::Message>, _: &irc::Message) -> bool;
}

pub struct SyncTrigger {
    // Returns true if this trigger applies to the passed in message
    pub cond: Lazy<Box<dyn Fn(&Sender<irc::Message>, &irc::Message) -> bool + Send + Sync>>, // TODO(raidancampbell): why was +send +sync needed here?

    // The action to perform if cond is true
    // return true if processing should continue
    pub act: Lazy<Box<dyn Fn(&Sender<irc::Message>, &irc::Message) -> bool + Send + Sync>>,
}

impl Trigger for SyncTrigger {
    fn condition(&self, tx: &Sender<irc::Message>, msg: &irc::Message) -> bool {
        (self.cond)(tx, msg)
    }

    fn action(&self, tx: &Sender<irc::Message>, msg: &irc::Message) -> bool {
        (self.act)(tx, msg)
    }
}
