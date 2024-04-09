use crate::player::Players;
use futures::SinkExt;
use std::error::Error;
use tokio::net::TcpStream;
use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use crate::connection::{Prompt, TelnetCodec};

pub async fn login_prompt(
    telnet: &mut Framed<TcpStream, TelnetCodec>,
    players: &Players,
) -> Result<Option<String>, Box<dyn Error + Send>> {
    loop {
        // TODO: We actually need to write our own LinesCodec so we can do things like send without
        // a newline for prompts
        // TODO: In that, we should pay close attention to how we're converting from utf8
        let _prompt = telnet.send(Prompt::new("Enter a username: ")).await;

        // Read the first line from the `LinesCodec` stream to get the username
        let username = match telnet.next().await {
            Some(Ok(line)) => line,
            // We didn't get a line so we return early here.
            _ => {
                tracing::error!("Client disconnected during login");
                return Ok(None);
            }
        };

        if players.read().values().any(|p| p.username == username) {
            tracing::warn!("Client attempted to use existing username {}", username);
            let _ = telnet.send("Username already taken. Try again.").await;
        } else {
            tracing::info!("Welcoming new player {}", username);
            let welcome = format!("Welcome, {}", username);
            let _ = telnet.send(welcome).await;
            return Ok(Some(username));
        }
    }
}
