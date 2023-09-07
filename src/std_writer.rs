use std::fmt::Display;
use std::sync::{Mutex, Arc};

pub struct StdWriter {
    closure_message: Arc<Mutex<Option<String>>>
}

impl StdWriter {
    pub fn new() -> StdWriter{
        StdWriter { closure_message: Arc::new(Mutex::new(None))}
    }

    //TODO make that when some closure is running, other closures passed must wait
    pub fn ask_user(&self, closure_message: String, closure: fn() -> Result<(), UserInquryError>) -> Result<(), UserInquryError> {
        {
            let closure_message_arc = Arc::clone(&self.closure_message);
            let mut closure_message_lock = closure_message_arc.lock().unwrap();
            if *closure_message_lock != None {
                panic!("Tried to ask user for input from two threads.")
            }
            *closure_message_lock = Some(closure_message.clone());
        }
        println!("{}", closure_message);
        let result = closure();
        {
            let closure_message_arc = Arc::clone(&self.closure_message);
            let mut closure_message_lock = closure_message_arc.lock().unwrap();
            *closure_message_lock = None;
        }
        result
    }

    pub fn println<T: Display>(&self, message: T) {
        println!("{}", message);
        let closure_message_arc = Arc::clone(&self.closure_message);
        let closure_message_lock = closure_message_arc.lock().unwrap();
        if let Some(closure_message) = &*closure_message_lock {
            println!("{}", closure_message);
        } 
    }
}

use std::fmt;

#[derive(Debug, Clone)]
pub struct UserInquryError;
impl fmt::Display for UserInquryError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "User fucked up when was asked about something.")
    }
}

#[derive(Debug, Clone, PartialEq)]
pub struct Message {
    contents: String,
}



pub struct GameData {}
