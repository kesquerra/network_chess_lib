use std::{io, net::TcpStream, io::Write};

use num_traits::FromPrimitive;
use serde::{Serialize, Deserialize};
use crate::{opcode::Opcode, error::Err};


#[derive(Serialize, Deserialize, Debug)]
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

    pub fn error(error: Err) -> Packet {
        Packet {
            opcode: Opcode::Err as u8,
            length: 1,
            payload: vec![error as u8]
        }
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
            Err(e) => "".to_string()
        }
    }

    pub fn as_bytes(&self) -> Vec<u8> {
        let mut bytes = self.payload.clone();
        bytes.insert(0, self.opcode);
        bytes.insert(1, self.length as u8);
        bytes
    }

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
}