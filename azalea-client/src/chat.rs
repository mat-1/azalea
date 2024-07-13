//! Implementations of chat-related features.

use azalea_chat::FormattedText;
use azalea_protocol::packets::game::{
    clientbound_disguised_chat_packet::ClientboundDisguisedChatPacket,
    clientbound_player_chat_packet::ClientboundPlayerChatPacket,
    clientbound_system_chat_packet::ClientboundSystemChatPacket,
    serverbound_chat_command_packet::ServerboundChatCommandPacket,
    serverbound_chat_packet::{LastSeenMessagesUpdate, ServerboundChatPacket},
};
use bevy_app::{App, Plugin, Update};
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    prelude::Event,
    schedule::IntoSystemConfigs,
};
use std::{
    sync::Arc,
    time::{SystemTime, UNIX_EPOCH},
};
use uuid::Uuid;

use crate::packet_handling::game::{handle_send_packet_event, SendPacketEvent};

/// A chat packet, either a system message or a chat message.
#[derive(Debug, Clone, PartialEq)]
pub enum ChatPacket {
    System(Arc<ClientboundSystemChatPacket>),
    Player(Arc<ClientboundPlayerChatPacket>),
    Disguised(Arc<ClientboundDisguisedChatPacket>),
}

macro_rules! regex {
    ($re:literal $(,)?) => {{
        static RE: once_cell::sync::OnceCell<regex::Regex> = once_cell::sync::OnceCell::new();
        RE.get_or_init(|| regex::Regex::new($re).unwrap())
    }};
}

impl ChatPacket {
    /// Get the message shown in chat for this packet.
    pub fn message(&self) -> FormattedText {
        match self {
            ChatPacket::System(p) => p.content.clone(),
            ChatPacket::Player(p) => p.message(),
            ChatPacket::Disguised(p) => p.message(),
        }
    }

    /// Determine the username of the sender and content of the message. This
    /// does not preserve formatting codes. If it's not a player-sent chat
    /// message or the sender couldn't be determined, the username part will be
    /// None.
    pub fn split_sender_and_content(&self) -> (Option<String>, String) {
        match self {
            ChatPacket::System(p) => {
                let message = p.content.to_string();
                // Overlay messages aren't in chat
                if p.overlay {
                    return (None, message);
                }
                // It's a system message, so we'll have to match the content
                // with regex
                if let Some(m) = regex!("^<([a-zA-Z_0-9]{1,16})> (.+)$").captures(&message) {
                    return (Some(m[1].to_string()), m[2].to_string());
                }

                (None, message)
            }
            ChatPacket::Player(p) => (
                // If it's a player chat packet, then the sender and content
                // are already split for us.
                Some(p.chat_type.name.to_string()),
                p.body.content.clone(),
            ),
            ChatPacket::Disguised(p) => (
                // disguised chat packets are basically the same as player chat packets but without
                // the chat signing things
                Some(p.chat_type.name.to_string()),
                p.message.to_string(),
            ),
        }
    }

    /// Get the username of the sender of the message. If it's not a
    /// player-sent chat message or the sender couldn't be determined, this
    /// will be None.
    pub fn username(&self) -> Option<String> {
        self.split_sender_and_content().0
    }

    /// Get the UUID of the sender of the message. If it's not a
    /// player-sent chat message, this will be None (this is sometimes the case
    /// when a server uses a plugin to modify chat messages).
    pub fn uuid(&self) -> Option<Uuid> {
        match self {
            ChatPacket::System(_) => None,
            ChatPacket::Player(m) => Some(m.sender),
            ChatPacket::Disguised(_) => None,
        }
    }

    /// Get the content part of the message as a string. This does not preserve
    /// formatting codes. If it's not a player-sent chat message or the sender
    /// couldn't be determined, this will contain the entire message.
    pub fn content(&self) -> String {
        self.split_sender_and_content().1
    }

    /// Create a new ChatPacket from a string. This is meant to be used as a
    /// convenience function for testing.
    pub fn new(message: &str) -> Self {
        ChatPacket::System(Arc::new(ClientboundSystemChatPacket {
            content: FormattedText::from(message),
            overlay: false,
        }))
    }

    /// Whether this message was sent with /msg (or aliases). It works by
    /// checking the translation key, so it won't work on servers that use their
    /// own whisper system.
    pub fn is_whisper(&self) -> bool {
        match self.message() {
            FormattedText::Text(_) => false,
            FormattedText::Translatable(t) => t.key == "commands.message.display.incoming",
        }
    }
}

pub struct ChatPlugin;
impl Plugin for ChatPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<SendChatEvent>()
            .add_event::<SendChatKindEvent>()
            .add_event::<ChatReceivedEvent>()
            .add_systems(
                Update,
                (
                    handle_send_chat_event,
                    handle_send_chat_kind_event.after(handle_send_packet_event),
                )
                    .chain(),
            );
    }
}

/// A client received a chat message packet.
#[derive(Event, Debug, Clone)]
pub struct ChatReceivedEvent {
    pub entity: Entity,
    pub packet: ChatPacket,
}

/// Send a chat message (or command, if it starts with a slash) to the server.
#[derive(Event)]
pub struct SendChatEvent {
    pub entity: Entity,
    pub content: String,
}

fn handle_send_chat_event(
    mut events: EventReader<SendChatEvent>,
    mut send_chat_kind_events: EventWriter<SendChatKindEvent>,
) {
    for event in events.read() {
        if event.content.starts_with('/') {
            send_chat_kind_events.send(SendChatKindEvent {
                entity: event.entity,
                content: event.content[1..].to_string(),
                kind: ChatPacketKind::Command,
            });
        } else {
            send_chat_kind_events.send(SendChatKindEvent {
                entity: event.entity,
                content: event.content.clone(),
                kind: ChatPacketKind::Message,
            });
        }
    }
}

/// Send a chat packet to the server of a specific kind (chat message or
/// command). Usually you just want [`SendChatEvent`] instead.
///
/// Usually setting the kind to `Message` will make it send a chat message even
/// if it starts with a slash, but some server implementations will always do a
/// command if it starts with a slash.
///
/// If you're wondering why this isn't two separate events, it's so ordering is
/// preserved if multiple chat messages and commands are sent at the same time.
#[derive(Event)]
pub struct SendChatKindEvent {
    pub entity: Entity,
    pub content: String,
    pub kind: ChatPacketKind,
}

/// A kind of chat packet, either a chat message or a command.
pub enum ChatPacketKind {
    Message,
    Command,
}

fn handle_send_chat_kind_event(
    mut events: EventReader<SendChatKindEvent>,
    mut send_packet_events: EventWriter<SendPacketEvent>,
) {
    for event in events.read() {
        let content = event
            .content
            .chars()
            .filter(|c| !matches!(c, '\x00'..='\x1F' | '\x7F' | '§'))
            .take(256)
            .collect::<String>();
        let packet = match event.kind {
            ChatPacketKind::Message => ServerboundChatPacket {
                message: content,
                timestamp: SystemTime::now()
                    .duration_since(UNIX_EPOCH)
                    .expect("Time shouldn't be before epoch")
                    .as_millis()
                    .try_into()
                    .expect("Instant should fit into a u64"),
                salt: azalea_crypto::make_salt(),
                signature: None,
                last_seen_messages: LastSeenMessagesUpdate::default(),
            }
            .get(),
            ChatPacketKind::Command => {
                // TODO: chat signing
                ServerboundChatCommandPacket { command: content }.get()
            }
        };

        send_packet_events.send(SendPacketEvent {
            entity: event.entity,
            packet,
        });
    }
}

// TODO
// MessageSigner, ChatMessageContent, LastSeenMessages
// fn sign_message() -> MessageSignature {
//     MessageSignature::default()
// }
