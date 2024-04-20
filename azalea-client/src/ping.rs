//! Ping Minecraft servers.

use azalea_protocol::{
    connect::{Connection, ConnectionError, Proxy},
    packets::{
        handshaking::{
            client_intention_packet::ClientIntentionPacket, ClientboundHandshakePacket,
            ServerboundHandshakePacket,
        },
        status::{
            clientbound_status_response_packet::ClientboundStatusResponsePacket,
            serverbound_status_request_packet::ServerboundStatusRequestPacket,
            ClientboundStatusPacket,
        },
        ConnectionProtocol, PROTOCOL_VERSION,
    },
    resolver, ServerAddress,
};
use std::io;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum PingError {
    #[error("{0}")]
    Resolver(#[from] resolver::ResolverError),
    #[error("{0}")]
    Connection(#[from] ConnectionError),
    #[error("{0}")]
    ReadPacket(#[from] Box<azalea_protocol::read::ReadPacketError>),
    #[error("{0}")]
    WritePacket(#[from] io::Error),
    #[error("The given address could not be parsed into a ServerAddress")]
    InvalidAddress,
}

/// Ping a Minecraft server.
///
/// # Examples
///
/// ```rust,no_run
/// use azalea_client::ping;
///
/// #[tokio::main]
/// async fn main() {
///     let response = ping::ping_server("play.hypixel.net").await.unwrap();
///     println!("{}", response.description.to_ansi());
/// }
/// ```
pub async fn ping_server(
    address: impl TryInto<ServerAddress>,
) -> Result<ClientboundStatusResponsePacket, PingError> {
    let address: ServerAddress = address.try_into().map_err(|_| PingError::InvalidAddress)?;
    let resolved_address = resolver::resolve_address(&address).await?;
    let conn = Connection::new(&resolved_address).await?;
    ping_server_with_connection(address, conn).await
}

/// Ping a Minecraft server through a Socks5 proxy.
pub async fn ping_server_with_proxy(
    address: impl TryInto<ServerAddress>,
    proxy: Proxy,
) -> Result<ClientboundStatusResponsePacket, PingError> {
    let address: ServerAddress = address.try_into().map_err(|_| PingError::InvalidAddress)?;
    let resolved_address = resolver::resolve_address(&address).await?;
    let conn = Connection::new_with_proxy(&resolved_address, proxy).await?;
    ping_server_with_connection(address, conn).await
}

/// Ping a Minecraft server after we've already created a [`Connection`]. The
/// `Connection` must still be in the handshake state (which is the state it's
/// in immediately after it's created).
pub async fn ping_server_with_connection(
    address: ServerAddress,
    mut conn: Connection<ClientboundHandshakePacket, ServerboundHandshakePacket>,
) -> Result<ClientboundStatusResponsePacket, PingError> {
    // send the client intention packet and switch to the status state
    conn.write(
        ClientIntentionPacket {
            protocol_version: PROTOCOL_VERSION,
            hostname: address.host.clone(),
            port: address.port,
            intention: ConnectionProtocol::Status,
        }
        .get(),
    )
    .await?;
    let mut conn = conn.status();

    // send the empty status request packet
    conn.write(ServerboundStatusRequestPacket {}.get()).await?;

    let packet = conn.read().await?;

    loop {
        match packet {
            ClientboundStatusPacket::StatusResponse(p) => return Ok(p),
            ClientboundStatusPacket::PongResponse(_) => {
                // we should never get this packet since we didn't send a ping
            }
        }
    }
}
