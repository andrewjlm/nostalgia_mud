// Messages that connections can send to the game loop
pub enum PlayerMessage {
    // Global Chat
    Gossip(String),
}

// Messages that the game loop can send to connections
pub enum GameMessage {
    // Global Chat
    Gossip(String),
}
