use log::warn;

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
