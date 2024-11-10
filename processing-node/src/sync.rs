use std::{io::{Read, Write}, net::TcpStream};

use serde::{Deserialize, Serialize};

pub struct PacketStream {
    header_length: usize,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct Packet {
    xid: usize,
    len: usize,
}

impl PacketStream {
    pub fn new() -> Self {
        let tmp = serde_xdr::to_bytes(&Packet { len: 0, xid: 0 }).unwrap();
        Self {
            header_length: tmp.len(),
        }
    }

    pub fn read<R>(&self, socket: &mut TcpStream) -> Result<R, String>
    where
        for<'a> R: Deserialize<'a>,
    {
        let mut hd = vec![0; self.header_length];
        let n = socket
            .read_exact(&mut hd)
            .map_err(|x| x.to_string())?;

        let hd = serde_xdr::from_bytes::<_, Packet>(&hd).map_err(|x| x.to_string())?;
        let mut body = vec![0; hd.len];
        socket
            .read_exact(&mut body)
            .map_err(|x| x.to_string())?;
        let q = serde_xdr::from_bytes(&body);
        q.map_err(|x| x.to_string())
    }

    pub fn write<R>(&self, socket: &mut TcpStream, value: &R) -> Result<(), String>
    where
        R: Serialize,
    {
        let body = serde_xdr::to_bytes(value).map_err(|x| x.to_string())?;
        let hd = Packet {
            xid: 0,
            len: body.len(),
        };
        let hd = serde_xdr::to_bytes(&hd).map_err(|x| x.to_string())?;
        assert_eq!(hd.len(), self.header_length);
        socket.write_all(&hd).map_err(|x| x.to_string())?;
        socket.write_all(&body).map_err(|x| x.to_string())?;
        Ok(())
    }
}
