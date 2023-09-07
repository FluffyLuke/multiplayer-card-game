#[derive(Debug)]
pub struct IdGenerator {
    current_id: u64,
}

impl IdGenerator {
    pub fn new() -> IdGenerator {
        IdGenerator { 
            current_id: 0,
        }
    }
    pub fn generate_id(&mut self) -> Id {
        let new_id = Id {
            id: self.current_id
        };
        self.current_id = self.current_id + 1;
        new_id
    }
}

#[derive(Debug, PartialEq)]
pub struct Id {
    id: u64,
}