use log::warn;
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

const BYTE_IAC: u8 = 255;
const BYTE_NOP: u8 = 241;
const BYTE_SB: u8 = 250;
const BYTE_SE: u8 = 240;
const BYTE_WILL: u8 = 251;
const BYTE_WONT: u8 = 252;
const BYTE_DO: u8 = 253;
const BYTE_DONT: u8 = 254;

#[derive(Debug)]
pub enum TelnetEvent {
    Data(Vec<u8>),
    IAC,
    NOP,
    SB,
    SE,
    WILL(u8),
    WONT(u8),
    DO(u8),
    DONT(u8),
}

pub fn parse_telnet_event(buffer: &mut Vec<u8>) -> Option<TelnetEvent> {
    if let Some(&byte) = buffer.first() {
        match byte {
            // Start of Telnet command sequence
            BYTE_IAC => {
                // We need to check the next byte to figure out what command
                buffer.remove(0);
                if let Some(&cmd) = buffer.first() {
                    match cmd {
                        BYTE_NOP => Some(TelnetEvent::NOP),
                        BYTE_SB => Some(TelnetEvent::SB),
                        BYTE_SE => Some(TelnetEvent::SE),
                        BYTE_WILL => Some(TelnetEvent::WILL(buffer.remove(0))),
                        BYTE_WONT => Some(TelnetEvent::WONT(buffer.remove(0))),
                        BYTE_DO => Some(TelnetEvent::DO(buffer.remove(0))),
                        BYTE_DONT => Some(TelnetEvent::DONT(buffer.remove(0))),
                        // Other commands?
                        _ => Some(TelnetEvent::IAC),
                    }
                } else {
                    Some(TelnetEvent::IAC)
                }
            }
            // If we get command bytes as the first byte... throw a warning?
            BYTE_NOP | BYTE_SB | BYTE_SE | BYTE_WILL | BYTE_WONT | BYTE_DO | BYTE_DONT => {
                warn!("Telnet command started with command byte {:?}", byte);
                None
            }
            _ => {
                let data = buffer.drain(..).take_while(|&b| b != 255).collect();
                Some(TelnetEvent::Data(data))
            }
        }
    } else {
        None
    }
}

pub struct TelnetWrapper {
    stream: TcpStream,
}

impl TelnetWrapper {
    pub fn new(stream: TcpStream) -> Self {
        TelnetWrapper { stream }
    }

    pub async fn read(&mut self, buf: &mut [u8]) -> tokio::io::Result<usize> {
        self.stream.read(buf).await
    }

    pub async fn write_all(&mut self, src: &[u8]) {
        self.stream.write_all(src).await.unwrap();
    }

    pub async fn write_message(&mut self, message: &str) {
        // TODO: We currently never use this
        // TODO: Is this the place to handle adding `\r\n` and all of that nonsense
        // I think we should actually have functions - on that writes without the termination (eg
        // for a prompt) and another that writes with it (for normal messages)
        self.stream.write_all(message.as_bytes()).await.unwrap();
    }
}

pub async fn read_from_buffer(
    buffer: &mut [u8],
    telnet_buffer: &mut Vec<u8>,
    stream: &mut TelnetWrapper,
) -> Option<Vec<u8>> {
    match stream.read(buffer).await {
        Ok(0) => None,
        Ok(n) => {
            telnet_buffer.extend_from_slice(&buffer[..n]);
            if let Some(event) = parse_telnet_event(telnet_buffer) {
                match event {
                    TelnetEvent::Data(bytes) => Some(bytes.to_vec()),
                    _ => {
                        log::warn!("Received unhandled TelnetEvent: {:?}", event);
                        None
                    }
                }
            } else {
                None
            }
        }
        Err(e) => {
            log::error!("Error reading from stream: {}", e);
            None
        }
    }
}
