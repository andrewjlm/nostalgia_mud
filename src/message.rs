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
                    "say" => {
                        let rest = parts.collect::<Vec<_>>().join(" ");
                        if rest.is_empty() {
                            //TODO: Should this alert somehow?
                            None
                        } else {
                            Some(PlayerMessage::Say(rest.to_string()))
                        }
                    }
                    "l" | "look" => {
                        // TODO: Right now this is just a generic 'look' command for the current
                        // room but ultimately it should allow you to look at specific things as
                        // well
                        Some(PlayerMessage::Look)
                    }
                    "n" | "north" => Some(PlayerMessage::Move(Direction::North)),
                    "s" | "south" => Some(PlayerMessage::Move(Direction::South)),
                    "e" | "east" => Some(PlayerMessage::Move(Direction::East)),
                    "w" | "west" => Some(PlayerMessage::Move(Direction::West)),
                    "u" | "up" => Some(PlayerMessage::Move(Direction::Up)),
                    "d" | "down" => Some(PlayerMessage::Move(Direction::Down)),
                    _ => {
                        // If no explicit command is matched, flag for possible contextual commands (such as an
                        // exit to a room that isn't one of the explicitly defined ones)
                        let rest = parts.collect::<Vec<_>>().join(" ");
                        Some(PlayerMessage::Contextual(cmd, rest.to_string()))
                    }
                }
            }
            // TODO: Under what circumstances can this happen?
            _ => None,
        }
    }
}

// Room exits that we explicitly check for
#[derive(Debug, strum_macros::Display, Clone, Copy)]
#[strum(serialize_all = "lowercase")]
pub enum Direction {
    North,
    South,
    East,
    West,
    Up,
    Down,
}

// Messages that connections can send to the game loop
pub enum PlayerMessage {
    // Global Chat
    Gossip(String),
    // Room Chat
    Say(String),
    // Look
    Look,
    // Possible Contextual command w/ Arguments
    Contextual(String, String),
    // Predefined Movements
    Move(Direction),
}

// Messages that the game loop can send to connections
#[derive(Debug, Clone)]
pub enum GameMessage {
    // Global Chat
    Gossip(String, String),
    Say(String, String),
    Look(String),
    NoExit(Direction),
    NotParsed,
}

impl GameMessage {
    pub fn to_response(&self) -> String {
        match self {
            GameMessage::Gossip(content, sending_user) => {
                format!("Gossip <{}>: {}\r\n", sending_user, content)
            }
            GameMessage::Say(content, sending_user) => {
                format!("Say <{}>: {}\r\n", sending_user, content)
            }
            GameMessage::Look(description) => {
                format!("{}\r\n", description)
            }
            GameMessage::NotParsed => String::from("Arglebargle, glop-glyf!?!?!\r\n"),
            GameMessage::NoExit(direction) => {
                // TODO: This will read sort of awkward (eg "You don't see an
                // exit north from here" when we'd probably say "north of
                // here"). Should figure out a way to get consistent.
                format!("You don't see an exit {} from here\r\n", direction)
            }
        }
    }

    pub fn to_bytes_response(&self) -> Vec<u8> {
        self.to_response().into_bytes()
    }
}
