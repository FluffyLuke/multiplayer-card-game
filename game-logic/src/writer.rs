use std::fmt;
use tokio::sync::broadcast::{Receiver, Sender};
pub struct Writer;

pub struct DisplayError;

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot write to the standart output")
    }
}

pub struct PlayerConnector<T: Clone> {
    tx: Sender<T>,
    rx: Receiver<T>,
}

impl <T: Clone> PlayerConnector<T> {
    pub fn new(tx: Sender<T>, rx: Receiver<T>) -> PlayerConnector<T> {
        PlayerConnector { rx, tx }
    }

    pub fn clone_tx_rx(&self) -> (Sender<T>, Receiver<T>) {
        (self.tx.clone(), self.tx.subscribe())
    }
}
