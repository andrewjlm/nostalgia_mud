use crate::{
    message::ConnectionMessage,
    player::{Player, Players},
};
use tokio::{net::TcpStream, sync::mpsc};

use tokio_stream::StreamExt;
use tokio_util::codec::Framed;

use futures::SinkExt;

mod login;
use login::login_prompt;

mod telnet_codec;
use telnet_codec::{Prompt, TelnetCodec};

#[tracing::instrument(skip_all,
                      fields(peer_addr = %stream.peer_addr().unwrap(),
                      username = tracing::field::Empty),)]
pub async fn handle_connection(
    players: Players,
    stream: TcpStream,
    game_sender: mpsc::Sender<ConnectionMessage>,
) {
    // Generate a communication channel
    let (player_sender, mut player_receiver) = mpsc::unbounded_channel();

    // Setup a Framed LinesCodec to read/write lines to/from the connection
    // TODO: with_max_length so we don't get blasted
    let mut telnet = Framed::new(stream, TelnetCodec::new());

    tracing::info!("Client connected");

    // Dispatch to the login flow
    if let Ok(Some(username)) = login_prompt(&mut telnet, &players).await {
        // Start logging events with the player name after login
        tracing::Span::current().record("username", &username);

        // Create a new player instance and send it to the game_loop to add to the list of current
        // players
        // TODO: Right now everyone ALWAYS starts in the same room. Also, if Room 1 doesn't
        // exist... not sure what happens but presumably bad
        let player = Player::new(username, &players, player_sender, 1);
        // Reserve a copy of the ID for downstream usage
        let player_id = player.id;
        let create_player_command = ConnectionMessage::AddPlayer(player);
        let _ = game_sender.send(create_player_command).await;

        loop {
            // Once the connection is established, send/receive messages from the player
            tokio::select! {
                read_line = telnet.next() => {
                    match read_line {
                        Some(Ok(message)) => {
                            // If we get a message from the client, send it to the server as a
                            // potential command to process, annotated with the sender id
                            let _ = game_sender.send(ConnectionMessage::PlayerCommand(player_id, message)).await;
                        }
                        _ => {
                            // Otherwise, the client has disconnected and we'll remove them from
                            // the active players list
                            tracing::info!("Player disconnected");
                            let _ = game_sender.send(ConnectionMessage::RemovePlayer(player_id)).await;
                            return
                        }
                    }
                }
                game_message = player_receiver.recv() => {
                    if let Some(message) = game_message {
                        // If we get a message from the server, send it to the client
                        let _ = telnet.send(&message).await;
                    }
                }
            }
        }
    }
}
