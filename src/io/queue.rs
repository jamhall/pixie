use std::collections::VecDeque;
use std::sync::{Arc, Mutex};

use crate::io::Command;

#[derive(Clone, Debug)]
pub struct Queue {
    events: Arc<Mutex<VecDeque<Command>>>,
}

impl Queue {
    pub fn new() -> Self {
        Self {
            events: Arc::new(Mutex::new(VecDeque::new()))
        }
    }

    pub fn push(&self, command: Command) {
        if let Ok(mut events) = self.events.lock() {
            events.push_back(command);
        }
    }

    pub fn pop(&self) -> Option<Command> {
        if let Ok(mut events) = self.events.lock() {
            return events.pop_front();
        }
        None
    }
}

impl Default for Queue {
    fn default() -> Self {
        Self::new()
    }
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_create() {
        let queue = Queue::default();

        let command = queue.pop();
        assert_eq!(command.is_none(), true);

        queue.push(Command::Clear);

        let command = queue.pop();

        assert_eq!(command.is_some(), true);
    }
}
