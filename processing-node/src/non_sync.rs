use serde::{Deserialize, Serialize};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    net::TcpStream,
};

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

    pub async fn read<R>(&self, socket: &mut tokio::net::TcpStream) -> Result<R, String>
    where
        for<'a> R: Deserialize<'a>,
    {
        let mut hd = vec![0; self.header_length];
        let n = socket
            .read_exact(&mut hd)
            .await
            .map_err(|x| format!("reading packet length: {x:?}"))?;
        assert_eq!(n, self.header_length);

        let hd =
            serde_xdr::from_bytes::<_, Packet>(&hd).map_err(|x| format!("deserialize: {x:?}"))?;
        let mut body = vec![0; hd.len];
        socket
            .read_exact(&mut body)
            .await
            .map_err(|x| format!("reading packet data: {x:?}"))?;
        let q = serde_xdr::from_bytes(&body);
        q.map_err(|x| x.to_string())
    }

    pub async fn write<R>(&self, socket: &mut TcpStream, value: &R) -> Result<(), String>
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
        socket.write_all(&hd).await.map_err(|x| x.to_string())?;
        socket.write_all(&body).await.map_err(|x| x.to_string())?;
        Ok(())
    }
}
