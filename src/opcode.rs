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
    SendMove = 0x0A,
    RecvMove = 0x0B,
    SendMsg = 0x0C
}