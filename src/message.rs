#[derive(Debug)]
pub struct RawCommand {
    sender_id: u32,
    command: String,
}

impl RawCommand {
    pub fn new(sender_id: u32, command: String) -> RawCommand {
        RawCommand { sender_id, command }
    }

    pub fn sender(&self) -> u32 {
        self.sender_id
    }

    pub fn interpret(&self) -> Option<PlayerMessage> {
        // Get the first word (assumed to be the command)
        let mut parts = self.command.split_whitespace();

        match parts.next() {
            Some(cmd) => {
                // NOTE: We don't care about casing of our command
                let cmd = cmd.to_lowercase();
                match cmd.as_str() {
                    "." | "gossip" => {
                        let rest = parts.collect::<Vec<_>>().join(" ");
                        if rest.is_empty() {
                            //TODO: Should this alert somehow?
                            None
                        } else {
                            Some(PlayerMessage::Gossip(rest.to_string()))
                        }
                    }
                    // TODO Under what circumstances can this happen?
                    _ => None,
                }
            }
            // TODO: Under what circumstances can this happen?
            _ => None,
        }
    }
}

// Messages that connections can send to the game loop
pub enum PlayerMessage {
    // Global Chat
    Gossip(String),
}

// Messages that the game loop can send to connections
pub enum GameMessage {
    // Global Chat
    Gossip(String, String),
    NotParsed,
}
