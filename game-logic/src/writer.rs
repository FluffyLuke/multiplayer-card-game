use std::fmt;
use tokio::sync::broadcast::{Receiver, Sender};

pub trait Displayer<T: Communicator> {
    fn new(communicator: T);
    fn display() -> Option<DisplayError>;
}
pub struct Writer;

pub struct DisplayError;

impl fmt::Display for DisplayError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Cannot write to the standart output")
    }
}

pub trait Communicator {
    fn send(&self);
    fn receive(&self);
}
pub struct BackendConnector<T: Clone> {
    tx: Receiver<T>,
    rx: Sender<T>,
}

impl <T: Clone> Communicator for BackendConnector<T> {
    fn send(&self) {
        todo!()
    }
    fn receive(&self) {
        todo!()
    }
}

impl <T: Clone> BackendConnector<T> {
    pub fn new(tx: Receiver<T>, rx: Sender<T>) -> BackendConnector<T> {
        BackendConnector { rx, tx }
    }
}