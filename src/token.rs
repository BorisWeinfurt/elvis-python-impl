use std::sync::{Arc, Mutex};
use smoltcp::phy::{RxToken, TxToken};
use log::trace;
use crate::channel::Channel;

pub struct MyRxToken {
    pub(crate) channel: Arc<Mutex<Channel>>,
}

impl RxToken for MyRxToken {
    fn consume<R, F>(mut self, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        trace!("Consuming RxToken");
        // Reads and stores buffer from channel
        let mut recv = self.channel.lock().unwrap().read();
        // Gives the read buffer to the closure
        f(&mut recv)
    }
}
pub struct MyTxToken {
    pub(crate) channel: Arc<Mutex<Channel>>,
}

impl<'a> TxToken for MyTxToken {
    fn consume<R, F>(self, len: usize, f: F) -> R
    where
        F: FnOnce(&mut [u8]) -> R,
    {
        trace!("Consuming TxToken");
        let mut buffer = vec![0; len];
        // Give empty buffer to closure to fill
        let result = f(&mut buffer);
        // Sends filled buffer over the channel
        self.channel.lock().unwrap().write(buffer);
        result
    }
}