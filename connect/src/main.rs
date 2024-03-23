use std::{
    io::{BufReader, Write},
    net::{SocketAddr, TcpStream},
    time::{SystemTime, UNIX_EPOCH},
};

use anyhow::Result;
use bitcoin::{
    consensus::{encode, Decodable},
    key::rand::random,
    p2p::{
        address::Address,
        message::{NetworkMessage, RawNetworkMessage},
        message_network::VersionMessage,
        ServiceFlags,
    },
    Network,
};
use log::{debug, info, warn};

fn main() -> Result<()> {
    env_logger::try_init()?;

    let addr: SocketAddr = "192.146.137.44:8333".parse()?;

    info!("connecting to {}", addr);
    let stream = connect(addr)?;
    info!("connected to {}", addr);

    let addrs = peers(stream)?;

    dbg!(addrs);

    Ok(())
}

/// Connect to the given address and exchange version information
fn connect(addr: SocketAddr) -> Result<TcpStream> {
    let mut stream = TcpStream::connect(addr)?;

    let version = RawNetworkMessage::new(Network::Bitcoin.magic(), version_msg(addr)?);

    stream.write_all(encode::serialize(&version).as_slice())?;

    let mut reader = BufReader::new(stream.try_clone()?);

    loop {
        let reply = RawNetworkMessage::consensus_decode(&mut reader)?;

        match reply.payload() {
            NetworkMessage::Version(msg) => {
                let verack =
                    RawNetworkMessage::new(Network::Bitcoin.magic(), NetworkMessage::Verack);

                debug!("received {:?}", msg);

                stream.write_all(encode::serialize(&verack).as_slice())?;
            }
            NetworkMessage::Verack => {
                break;
            }
            _ => {
                warn!("received unknown message: {:?}", reply.payload());
                break;
            }
        }
    }

    Ok(stream)
}

// Get known active peers from a node
fn peers(mut stream: TcpStream) -> Result<Vec<(u32, Address)>> {
    let addr = RawNetworkMessage::new(Network::Bitcoin.magic(), NetworkMessage::GetAddr);

    stream.write_all(encode::serialize(&addr).as_slice())?;

    let mut reader = BufReader::new(stream.try_clone()?);

    let reply = RawNetworkMessage::consensus_decode(&mut reader)?;

    match reply.payload() {
        NetworkMessage::Addr(msg) => Ok(msg.clone()),
        _ => Err(anyhow::anyhow!(
            "received unknown message: {:?}",
            reply.payload()
        )),
    }
}

// Build a network version message for the given address
fn version_msg(addr: SocketAddr) -> Result<NetworkMessage> {
    Ok(NetworkMessage::Version(VersionMessage {
        version: 70015,
        services: ServiceFlags::NETWORK,
        timestamp: SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs() as i64,
        receiver: Address::new(&addr, ServiceFlags::NONE),
        sender: Address::new(&addr, ServiceFlags::NONE),
        nonce: random::<u64>(),
        user_agent: "/bitcoin-rust v0.1.0/".to_owned(),
        start_height: 0,
        relay: false,
    }))
}
