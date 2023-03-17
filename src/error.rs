use std::fmt::Display;

pub enum Err {
    Unk = 0x00,
    IllegalOpcode = 0x01,
    IllegalLen = 0x02,
    NameExists = 0x03,
    IllegalMove = 0x04,
    UserLimit = 0x05,
    GameLimit = 0x06,
    BadAddr = 0x07,
    AlreadyInGame = 0x08,
    GameDoesntExist = 0x09,
    MalformedPacket = 0x0A,
    InvalidAccess = 0x0B
}

impl Display for Err {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Err::Unk => "Unknown",
            Err::IllegalOpcode => "Illegal Opcode",
            Err::IllegalLen => "Illegal Length",
            Err::NameExists => "Name Exists",
            Err::IllegalMove => "Illegal Move",
            Err::UserLimit => "User Limit",
            Err::GameLimit => "Game Limit",
            Err::BadAddr => "Bad IP Address",
            Err::AlreadyInGame => "Already in a Game",
            Err::GameDoesntExist => "Game Doesn't Exist",
            Err::MalformedPacket => "Malformed Packet",
            Err::InvalidAccess => "Bad Permissions"
        };
        write!(f, "{}", str)
    }
}