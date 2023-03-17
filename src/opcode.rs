use std::fmt::Display;

use num_derive::FromPrimitive;
use num_traits::FromPrimitive;

#[derive(FromPrimitive, PartialEq, Clone)]
pub enum Opcode {
    Err = 0x00,
    KeepAlive = 0x01,
    Join = 0x02,
    ListGames = 0x03,
    ListGamesResp = 0x04,
    CreateGame = 0x05,
    CreateGameResp = 0x06,
    JoinGame = 0x07,
    JoinGameResp = 0x08,
    LeaveGame = 0x09,
    LeaveGameResp = 0x0A,
    SendMove = 0x0B,
    SendMoveResp = 0x0C,
    RecvMove = 0x0D,
    SendMsg = 0x0E,
    ShowGame = 0x0F,
    ShowGameResp = 0x10
}

impl Display for Opcode {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let str = match self {
            Opcode::Err                 => "Err",
            Opcode::KeepAlive           => "KeepAlive",
            Opcode::Join                => "Join",
            Opcode::ListGames           => "ListGames",
            Opcode::ListGamesResp       => "ListGamesResp",
            Opcode::CreateGame          => "CreateGame",
            Opcode::CreateGameResp      => "CreateGameResp",
            Opcode::JoinGame            => "JoinGame",
            Opcode::JoinGameResp        => "JoinGameResp",
            Opcode::LeaveGame           => "LeaveGame",
            Opcode::LeaveGameResp       => "LeaveGameResp",
            Opcode::SendMove            => "SendMove",
            Opcode::SendMoveResp        => "SendMoveResp",
            Opcode::RecvMove            => "RecvMove",
            Opcode::SendMsg             => "SendMsg",
            Opcode::ShowGame            => "ShowGame",
            Opcode::ShowGameResp        => "ShowGameResp"
        };
        write!(f, "{}", str)
    }
}

impl Opcode {
    pub fn from_u8(u: u8) -> Option<Opcode> {
        FromPrimitive::from_u8(u)
    }
}