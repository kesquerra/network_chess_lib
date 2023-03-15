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
    MalformedPacket = 0x0A
}