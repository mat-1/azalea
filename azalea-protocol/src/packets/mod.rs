pub mod common;
pub mod config;
pub mod game;
pub mod handshake;
pub mod login;
pub mod status;

use std::io::{Cursor, Write};

use azalea_buf::{AzaleaReadVar, AzaleaWrite, AzaleaWriteVar, BufReadError};

use crate::read::ReadPacketError;

pub const PROTOCOL_VERSION: i32 = 1073742054;
pub const VERSION_NAME: &str = "25w03a";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ConnectionProtocol {
    Handshake = -1,
    Game = 0,
    Status = 1,
    Login = 2,
    Configuration = 3,
}

impl ConnectionProtocol {
    #[must_use]
    pub fn from_i32(i: i32) -> Option<Self> {
        match i {
            -1 => Some(ConnectionProtocol::Handshake),
            0 => Some(ConnectionProtocol::Game),
            1 => Some(ConnectionProtocol::Status),
            2 => Some(ConnectionProtocol::Login),
            3 => Some(ConnectionProtocol::Configuration),
            _ => None,
        }
    }
}

/// An enum of packets for a certain protocol.
pub trait ProtocolPacket
where
    Self: Sized,
{
    fn id(&self) -> u32;

    /// Returns Mojang's resource name for the packet.
    ///
    /// This doesn't include the "minecraft:" prefix, it just returns a string
    /// like `pong`.
    fn name(&self) -> &'static str;

    /// Read a packet by its id, `ConnectionProtocol`, and flow
    fn read(id: u32, buf: &mut Cursor<&[u8]>) -> Result<Self, Box<ReadPacketError>>;

    fn write(&self, buf: &mut impl Write) -> Result<(), std::io::Error>;
}

pub trait Packet<Protocol> {
    fn into_variant(self) -> Protocol;
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum ClientIntention {
    Status = 1,
    Login = 2,
    Transfer = 3,
}

impl TryFrom<i32> for ClientIntention {
    type Error = ();

    fn try_from(value: i32) -> Result<Self, Self::Error> {
        match value {
            1 => Ok(ClientIntention::Status),
            2 => Ok(ClientIntention::Login),
            3 => Ok(ClientIntention::Transfer),
            _ => Err(()),
        }
    }
}

impl From<ClientIntention> for ConnectionProtocol {
    fn from(intention: ClientIntention) -> Self {
        match intention {
            ClientIntention::Status => ConnectionProtocol::Status,
            ClientIntention::Login | ClientIntention::Transfer => ConnectionProtocol::Login,
        }
    }
}

impl azalea_buf::AzaleaRead for ClientIntention {
    fn azalea_read(buf: &mut Cursor<&[u8]>) -> Result<Self, BufReadError> {
        let id = i32::azalea_read_var(buf)?;
        id.try_into()
            .map_err(|_| BufReadError::UnexpectedEnumVariant { id })
    }
}

impl AzaleaWrite for ClientIntention {
    fn azalea_write(&self, buf: &mut impl Write) -> Result<(), std::io::Error> {
        (*self as i32).azalea_write_var(buf)
    }
}
