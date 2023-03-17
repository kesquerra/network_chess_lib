use num_traits::FromPrimitive;
use tokio::{net::{TcpStream, tcp::{OwnedReadHalf, OwnedWriteHalf}}, io::{AsyncWriteExt, AsyncReadExt}};
use crate::{opcode::Opcode, error::Err};


#[derive(Debug, Clone)]
pub struct Packet {
    opcode: u8,
    length: usize,
    payload: Vec<u8>
}

impl Packet {
    pub fn new(opcode: Opcode, message: String) -> Packet {
        Packet {
            opcode: opcode as u8,
            length: message.bytes().count(),
            payload: message.as_bytes().to_owned()
        }
    }

    pub fn new_prim(opcode: Opcode, payload: Vec<u8>) -> Packet {
        Packet {
            opcode: opcode as u8,
            length: payload.len(),
            payload
        }
    }

    // keepalive packet
    pub fn ka() -> Packet {
        Packet { opcode: Opcode::KeepAlive as u8, length: 0, payload: Vec::new()}
    }

    // error packet
    pub fn error(error: Err) -> Packet {
        Packet::new(Opcode::Err, format!("{}", error))
    }


    pub fn opcode(&self) -> u8 {
        self.opcode
    }

    pub fn op(&self) -> Opcode {
        match FromPrimitive::from_u8(self.opcode) {
            Some(v) => v,
            None => Opcode::Err
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn payload(&self) -> Vec<u8> {
        self.payload.clone()
    }

    pub fn payload_str(&self) -> String {
        match String::from_utf8(self.payload.clone()) {
            Ok(s) => s,
            Err(_) => "".to_string()
        }
    }

    // convert the packet into a vector of bytes
    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.payload.clone();
        bytes.insert(0, self.opcode);
        bytes.insert(1, self.length as u8);
        bytes
    }

    // parse a vector of bytes into a packet (leaves remaining bytes)
    pub fn parse(bs: &mut Vec<u8>) -> Option<Packet> {
        let len = bs.len();
        if len < 2 {
            None
        } else {
            let size: usize = bs[1] as usize;
            if size+2 <= len {
                let op = bs.remove(0);
                let size: usize = bs.remove(0) as usize;
                let mut payload = Vec::new();
                for _ in 0..size {
                    payload.push(bs.remove(0));
                }
                Some(Packet{opcode: op, length: size, payload})
            } else {
                None
            }
        }
    }

    // parse a vector of bytes into a packet (consumes all bytes)
    pub fn from_bytes(mut bs: Vec<u8>) -> Result<Self, String> {
        let len = bs.len();
        if len < 2 {
            Err("Packet is too small.".to_string())
        } else {
            let op = bs.remove(0);
            let size: usize = bs.remove(0) as usize;
            if bs.len() != size {
                Err("Packet length is incorrect".to_string())
            } else {
                Ok(Packet {
                    opcode: op,
                    length: size,
                    payload: bs
                })
            }
        }
    }

    // send a packet on a TCP stream
    pub async fn send(self, tcp: &mut TcpStream) -> Result<(), String> {
        match tcp.write(&self.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }

    // send a packet on a TCP write stream
    pub async fn send_write(self, tcp: &mut OwnedWriteHalf) -> Result<(), String> {
        match tcp.write(&self.as_bytes()).await {
            Ok(_) => Ok(()),
            Err(e) => Err(e.to_string())
        }
    }

    // read a packet from a TCP stream
    pub async fn from_tcp(tcp: &mut TcpStream) -> Result<Packet, String> {
        let mut buf: [u8; 2] = [0, 0];
        match tcp.peek(&mut buf).await {
            Ok(_) => {
                let size: u64 = buf[1] as u64;
                let mut pbytes: Vec<u8> = Vec::new();
                match tcp.take(size+2).read_to_end(&mut pbytes).await {
                    Ok(_) => Packet::from_bytes(pbytes),
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }

    // read a packet from a TCP read stream
    pub async fn from_tcp_read(tcp: &mut OwnedReadHalf) -> Result<Packet, String> {
        let mut buf: [u8; 2] = [0, 0];
        match tcp.peek(&mut buf).await {
            Ok(_) => {
                let size: u64 = buf[1] as u64;
                let mut pbytes: Vec<u8> = Vec::new();
                match tcp.take(size+2).read_to_end(&mut pbytes).await {
                    Ok(_) => Packet::from_bytes(pbytes),
                    Err(e) => Err(e.to_string())
                }
            },
            Err(e) => Err(e.to_string())
        }
    }
}