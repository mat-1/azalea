use std::{collections::HashSet, io::Cursor, sync::Arc};

use azalea_core::{ChunkPos, ResourceLocation, Vec3};
use azalea_ecs::{
    app::{App, Plugin},
    component::Component,
    ecs::Ecs,
    entity::Entity,
    event::EventWriter,
    query::Changed,
    schedule::{IntoSystemDescriptor, SystemSet},
    system::{Commands, Query, ResMut, SystemState},
};
use azalea_protocol::{
    connect::{ReadConnection, WriteConnection},
    packets::game::{
        clientbound_player_combat_kill_packet::ClientboundPlayerCombatKillPacket,
        serverbound_accept_teleportation_packet::ServerboundAcceptTeleportationPacket,
        serverbound_custom_payload_packet::ServerboundCustomPayloadPacket,
        serverbound_keep_alive_packet::ServerboundKeepAlivePacket,
        serverbound_move_player_pos_rot_packet::ServerboundMovePlayerPosRotPacket,
        ClientboundGamePacket, ServerboundGamePacket,
    },
    read::ReadPacketError,
};
use azalea_world::{
    entity::{
        metadata::{apply_metadata, Health, PlayerMetadataBundle},
        set_rotation, Dead, EntityBundle, EntityKind, LastSentPosition, MinecraftEntityId, Physics,
        PlayerBundle, Position, WorldName,
    },
    entity::{LoadedBy, RelativeEntityUpdate},
    PartialWorld, WorldContainer,
};
use log::{debug, error, trace, warn};
use parking_lot::Mutex;
use tokio::sync::mpsc;

use crate::{
    chat::{ChatPacket, ChatReceivedEvent},
    client::ClientInformation,
    disconnect::DisconnectEvent,
    local_player::{GameProfileComponent, LocalPlayer},
    PlayerInfo,
};

pub struct PacketHandlerPlugin;

impl Plugin for PacketHandlerPlugin {
    fn build(&self, app: &mut App) {
        app.add_system_set(
            SystemSet::new().with_system(handle_packets.label("packet").before("tick")),
        )
        .add_event::<AddPlayerEvent>()
        .add_event::<RemovePlayerEvent>()
        .add_event::<UpdatePlayerEvent>()
        .add_event::<ChatReceivedEvent>()
        .add_event::<DeathEvent>()
        .add_event::<KeepAliveEvent>();
    }
}

/// A player joined the game (or more specifically, was added to the tab
/// list of a local player).
#[derive(Debug, Clone)]
pub struct AddPlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player left the game (or maybe is still in the game and was just
/// removed from the tab list of a local player).
#[derive(Debug, Clone)]
pub struct RemovePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player was updated in the tab list of a local player (gamemode, display
/// name, or latency changed).
#[derive(Debug, Clone)]
pub struct UpdatePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}

/// Event for when an entity dies. dies. If it's a local player and there's a
/// reason in the death screen, the [`ClientboundPlayerCombatKillPacket`] will
/// be included.
#[derive(Debug, Clone)]
pub struct DeathEvent {
    pub entity: Entity,
    pub packet: Option<ClientboundPlayerCombatKillPacket>,
}

/// A KeepAlive packet is sent from the server to verify that the client is
/// still connected.
#[derive(Debug, Clone)]
pub struct KeepAliveEvent {
    pub entity: Entity,
    /// The ID of the keepalive. This is an arbitrary number, but vanilla
    /// servers use the time to generate this.
    pub id: u64,
}

/// Something that receives packets from the server.
#[derive(Component, Clone)]
pub struct PacketReceiver {
    pub packets: Arc<Mutex<Vec<ClientboundGamePacket>>>,
    pub run_schedule_sender: mpsc::UnboundedSender<()>,
}

fn handle_packets(ecs: &mut Ecs) {
    let mut events_owned = Vec::new();

    {
        let mut system_state: SystemState<
            Query<(Entity, &PacketReceiver), Changed<PacketReceiver>>,
        > = SystemState::new(ecs);
        let query = system_state.get(ecs);
        for (player_entity, packet_events) in &query {
            let mut packets = packet_events.packets.lock();
            if !packets.is_empty() {
                events_owned.push((player_entity, packets.clone()));
                // clear the packets right after we read them
                packets.clear();
            }
        }
    }

    for (player_entity, packets) in events_owned {
        for packet in &packets {
            match packet {
                ClientboundGamePacket::Login(p) => {
                    debug!("Got login packet");

                    #[allow(clippy::type_complexity)]
                    let mut system_state: SystemState<(
                        Commands,
                        Query<(
                            &mut LocalPlayer,
                            Option<&mut WorldName>,
                            &GameProfileComponent,
                        )>,
                        ResMut<WorldContainer>,
                    )> = SystemState::new(ecs);
                    let (mut commands, mut query, mut world_container) = system_state.get_mut(ecs);
                    let (mut local_player, world_name, game_profile) =
                        query.get_mut(player_entity).unwrap();

                    {
                        // TODO: have registry_holder be a struct because this sucks rn
                        // best way would be to add serde support to azalea-nbt

                        let registry_holder = p
                            .registry_holder
                            .as_compound()
                            .expect("Registry holder is not a compound")
                            .get("")
                            .expect("No \"\" tag")
                            .as_compound()
                            .expect("\"\" tag is not a compound");
                        let dimension_types = registry_holder
                            .get("minecraft:dimension_type")
                            .expect("No dimension_type tag")
                            .as_compound()
                            .expect("dimension_type is not a compound")
                            .get("value")
                            .expect("No dimension_type value")
                            .as_list()
                            .expect("dimension_type value is not a list");
                        let dimension_type = dimension_types
                            .iter()
                            .find(|t| {
                                t.as_compound()
                                    .expect("dimension_type value is not a compound")
                                    .get("name")
                                    .expect("No name tag")
                                    .as_string()
                                    .expect("name is not a string")
                                    == p.dimension_type.to_string()
                            })
                            .unwrap_or_else(|| {
                                panic!("No dimension_type with name {}", p.dimension_type)
                            })
                            .as_compound()
                            .unwrap()
                            .get("element")
                            .expect("No element tag")
                            .as_compound()
                            .expect("element is not a compound");
                        let height = (*dimension_type
                            .get("height")
                            .expect("No height tag")
                            .as_int()
                            .expect("height tag is not an int"))
                        .try_into()
                        .expect("height is not a u32");
                        let min_y = *dimension_type
                            .get("min_y")
                            .expect("No min_y tag")
                            .as_int()
                            .expect("min_y tag is not an int");

                        let new_world_name = p.dimension.clone();

                        if let Some(mut world_name) = world_name {
                            *world_name = world_name.clone();
                        } else {
                            commands
                                .entity(player_entity)
                                .insert(WorldName(new_world_name.clone()));
                        }
                        // add this world to the world_container (or don't if it's already
                        // there)
                        let weak_world =
                            world_container.insert(new_world_name.clone(), height, min_y);
                        // set the partial_world to an empty world
                        // (when we add chunks or entities those will be in the
                        // world_container)

                        *local_player.partial_world.write() = PartialWorld::new(
                            local_player.client_information.view_distance.into(),
                            // this argument makes it so other clients don't update this
                            // player entity
                            // in a shared world
                            Some(player_entity),
                        );
                        local_player.world = weak_world;

                        let player_bundle = PlayerBundle {
                            entity: EntityBundle::new(
                                game_profile.uuid,
                                Vec3::default(),
                                azalea_registry::EntityKind::Player,
                                new_world_name,
                            ),
                            metadata: PlayerMetadataBundle::default(),
                        };
                        // insert our components into the ecs :)
                        commands
                            .entity(player_entity)
                            .insert((MinecraftEntityId(p.player_id), player_bundle));
                    }

                    // send the client information that we have set
                    let client_information_packet: ClientInformation =
                        local_player.client_information.clone();
                    log::debug!(
                        "Sending client information because login: {:?}",
                        client_information_packet
                    );
                    local_player.write_packet(client_information_packet.get());

                    // brand
                    local_player.write_packet(
                        ServerboundCustomPayloadPacket {
                            identifier: ResourceLocation::new("brand").unwrap(),
                            // they don't have to know :)
                            data: "vanilla".into(),
                        }
                        .get(),
                    );

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::SetChunkCacheRadius(p) => {
                    debug!("Got set chunk cache radius packet {:?}", p);
                }
                ClientboundGamePacket::CustomPayload(p) => {
                    debug!("Got custom payload packet {:?}", p);
                }
                ClientboundGamePacket::ChangeDifficulty(p) => {
                    debug!("Got difficulty packet {:?}", p);
                }
                ClientboundGamePacket::Commands(_p) => {
                    debug!("Got declare commands packet");
                }
                ClientboundGamePacket::PlayerAbilities(p) => {
                    debug!("Got player abilities packet {:?}", p);
                }
                ClientboundGamePacket::SetCarriedItem(p) => {
                    debug!("Got set carried item packet {:?}", p);
                }
                ClientboundGamePacket::UpdateTags(_p) => {
                    debug!("Got update tags packet");
                }
                ClientboundGamePacket::Disconnect(p) => {
                    debug!("Got disconnect packet {:?}", p);
                    let mut system_state: SystemState<EventWriter<DisconnectEvent>> =
                        SystemState::new(ecs);
                    let mut disconnect_events = system_state.get_mut(ecs);
                    disconnect_events.send(DisconnectEvent {
                        entity: player_entity,
                    });
                    // bye
                    return;
                }
                ClientboundGamePacket::UpdateRecipes(_p) => {
                    debug!("Got update recipes packet");
                }
                ClientboundGamePacket::EntityEvent(_p) => {
                    // debug!("Got entity event packet {:?}", p);
                }
                ClientboundGamePacket::Recipe(_p) => {
                    debug!("Got recipe packet");
                }
                ClientboundGamePacket::PlayerPosition(p) => {
                    // TODO: reply with teleport confirm
                    debug!("Got player position packet {:?}", p);

                    let mut system_state: SystemState<
                        Query<(
                            &mut LocalPlayer,
                            &mut Physics,
                            &mut Position,
                            &mut LastSentPosition,
                        )>,
                    > = SystemState::new(ecs);
                    let mut query = system_state.get_mut(ecs);
                    let Ok((mut local_player, mut physics, mut position, mut last_sent_position)) =
                        query.get_mut(player_entity) else {
                            continue;
                        };

                    let delta_movement = physics.delta;

                    let is_x_relative = p.relative_arguments.x;
                    let is_y_relative = p.relative_arguments.y;
                    let is_z_relative = p.relative_arguments.z;

                    let (delta_x, new_pos_x) = if is_x_relative {
                        last_sent_position.x += p.x;
                        (delta_movement.x, position.x + p.x)
                    } else {
                        last_sent_position.x = p.x;
                        (0.0, p.x)
                    };
                    let (delta_y, new_pos_y) = if is_y_relative {
                        last_sent_position.y += p.y;
                        (delta_movement.y, position.y + p.y)
                    } else {
                        last_sent_position.y = p.y;
                        (0.0, p.y)
                    };
                    let (delta_z, new_pos_z) = if is_z_relative {
                        last_sent_position.z += p.z;
                        (delta_movement.z, position.z + p.z)
                    } else {
                        last_sent_position.z = p.z;
                        (0.0, p.z)
                    };

                    let mut y_rot = p.y_rot;
                    let mut x_rot = p.x_rot;
                    if p.relative_arguments.x_rot {
                        x_rot += physics.x_rot;
                    }
                    if p.relative_arguments.y_rot {
                        y_rot += physics.y_rot;
                    }

                    physics.delta = Vec3 {
                        x: delta_x,
                        y: delta_y,
                        z: delta_z,
                    };
                    // we call a function instead of setting the fields ourself since the
                    // function makes sure the rotations stay in their
                    // ranges
                    set_rotation(&mut physics, y_rot, x_rot);
                    // TODO: minecraft sets "xo", "yo", and "zo" here but idk what that means
                    // so investigate that ig
                    let new_pos = Vec3 {
                        x: new_pos_x,
                        y: new_pos_y,
                        z: new_pos_z,
                    };

                    **position = new_pos;

                    local_player
                        .write_packet(ServerboundAcceptTeleportationPacket { id: p.id }.get());
                    local_player.write_packet(
                        ServerboundMovePlayerPosRotPacket {
                            x: new_pos.x,
                            y: new_pos.y,
                            z: new_pos.z,
                            y_rot,
                            x_rot,
                            // this is always false
                            on_ground: false,
                        }
                        .get(),
                    );
                }
                ClientboundGamePacket::PlayerInfoUpdate(p) => {
                    debug!("Got player info packet {:?}", p);

                    let mut system_state: SystemState<(
                        Query<&mut LocalPlayer>,
                        EventWriter<AddPlayerEvent>,
                        EventWriter<UpdatePlayerEvent>,
                    )> = SystemState::new(ecs);
                    let (mut query, mut add_player_events, mut update_player_events) =
                        system_state.get_mut(ecs);
                    let mut local_player = query.get_mut(player_entity).unwrap();

                    for updated_info in &p.entries {
                        // add the new player maybe
                        if p.actions.add_player {
                            let info = PlayerInfo {
                                profile: updated_info.profile.clone(),
                                uuid: updated_info.profile.uuid,
                                gamemode: updated_info.game_mode,
                                latency: updated_info.latency,
                                display_name: updated_info.display_name.clone(),
                            };
                            local_player
                                .players
                                .insert(updated_info.profile.uuid, info.clone());
                            add_player_events.send(AddPlayerEvent {
                                entity: player_entity,
                                info: info.clone(),
                            });
                        } else if let Some(info) =
                            local_player.players.get_mut(&updated_info.profile.uuid)
                        {
                            // `else if` because the block for add_player above
                            // already sets all the fields
                            if p.actions.update_game_mode {
                                info.gamemode = updated_info.game_mode;
                            }
                            if p.actions.update_latency {
                                info.latency = updated_info.latency;
                            }
                            if p.actions.update_display_name {
                                info.display_name = updated_info.display_name.clone();
                            }
                            update_player_events.send(UpdatePlayerEvent {
                                entity: player_entity,
                                info: info.clone(),
                            });
                        } else {
                            warn!(
                                "Ignoring PlayerInfoUpdate for unknown player {}",
                                updated_info.profile.uuid
                            );
                        }
                    }
                }
                ClientboundGamePacket::PlayerInfoRemove(p) => {
                    let mut system_state: SystemState<(
                        Query<&mut LocalPlayer>,
                        EventWriter<RemovePlayerEvent>,
                    )> = SystemState::new(ecs);
                    let (mut query, mut remove_player_events) = system_state.get_mut(ecs);
                    let mut local_player = query.get_mut(player_entity).unwrap();

                    for uuid in &p.profile_ids {
                        if let Some(info) = local_player.players.remove(uuid) {
                            remove_player_events.send(RemovePlayerEvent {
                                entity: player_entity,
                                info,
                            });
                        }
                    }
                }
                ClientboundGamePacket::SetChunkCacheCenter(p) => {
                    debug!("Got chunk cache center packet {:?}", p);

                    let mut system_state: SystemState<Query<&mut LocalPlayer>> =
                        SystemState::new(ecs);
                    let mut query = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();
                    let mut partial_world = local_player.partial_world.write();

                    partial_world.chunks.view_center = ChunkPos::new(p.x, p.z);
                }
                ClientboundGamePacket::LevelChunkWithLight(p) => {
                    debug!("Got chunk with light packet {} {}", p.x, p.z);
                    let pos = ChunkPos::new(p.x, p.z);

                    let mut system_state: SystemState<Query<&mut LocalPlayer>> =
                        SystemState::new(ecs);
                    let mut query = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();

                    // OPTIMIZATION: if we already know about the chunk from the
                    // shared world (and not ourselves), then we don't need to
                    // parse it again. This is only used when we have a shared
                    // world, since we check that the chunk isn't currently owned
                    // by this client.
                    let shared_chunk = local_player.world.read().chunks.get(&pos);
                    let this_client_has_chunk = local_player
                        .partial_world
                        .read()
                        .chunks
                        .limited_get(&pos)
                        .is_some();

                    let mut world = local_player.world.write();
                    let mut partial_world = local_player.partial_world.write();

                    if !this_client_has_chunk {
                        if let Some(shared_chunk) = shared_chunk {
                            trace!(
                                "Skipping parsing chunk {:?} because we already know about it",
                                pos
                            );
                            partial_world.chunks.set_with_shared_reference(
                                &pos,
                                Some(shared_chunk.clone()),
                                &mut world.chunks,
                            );
                            continue;
                        }
                    }

                    if let Err(e) = partial_world.chunks.replace_with_packet_data(
                        &pos,
                        &mut Cursor::new(&p.chunk_data.data),
                        &mut world.chunks,
                    ) {
                        error!("Couldn't set chunk data: {}", e);
                    }
                }
                ClientboundGamePacket::LightUpdate(_p) => {
                    // debug!("Got light update packet {:?}", p);
                }
                ClientboundGamePacket::AddEntity(p) => {
                    debug!("Got add entity packet {:?}", p);

                    let mut system_state: SystemState<(Commands, Query<Option<&WorldName>>)> =
                        SystemState::new(ecs);
                    let (mut commands, mut query) = system_state.get_mut(ecs);
                    let world_name = query.get_mut(player_entity).unwrap();

                    if let Some(WorldName(world_name)) = world_name {
                        let bundle = p.as_entity_bundle(world_name.clone());
                        let mut entity_commands = commands.spawn((
                            MinecraftEntityId(p.id),
                            LoadedBy(HashSet::from([player_entity])),
                            bundle,
                        ));
                        // the bundle doesn't include the default entity metadata so we add that
                        // separately
                        p.apply_metadata(&mut entity_commands);
                    } else {
                        warn!("got add player packet but we haven't gotten a login packet yet");
                    }

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::SetEntityData(p) => {
                    debug!("Got set entity data packet {:?}", p);

                    let mut system_state: SystemState<(
                        Commands,
                        Query<&mut LocalPlayer>,
                        Query<&EntityKind>,
                    )> = SystemState::new(ecs);
                    let (mut commands, mut query, entity_kind_query) = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();

                    let world = local_player.world.read();
                    let entity = world.entity_by_id(&MinecraftEntityId(p.id));
                    drop(world);

                    if let Some(entity) = entity {
                        let entity_kind = entity_kind_query.get(entity).unwrap();
                        let mut entity_commands = commands.entity(entity);
                        if let Err(e) = apply_metadata(
                            &mut entity_commands,
                            **entity_kind,
                            (*p.packed_items).clone(),
                        ) {
                            warn!("{e}");
                        }
                    } else {
                        warn!("Server sent an entity data packet for an entity id ({}) that we don't know about", p.id);
                    }

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::UpdateAttributes(_p) => {
                    // debug!("Got update attributes packet {:?}", p);
                }
                ClientboundGamePacket::SetEntityMotion(_p) => {
                    // debug!("Got entity velocity packet {:?}", p);
                }
                ClientboundGamePacket::SetEntityLink(p) => {
                    debug!("Got set entity link packet {:?}", p);
                }
                ClientboundGamePacket::AddPlayer(p) => {
                    debug!("Got add player packet {:?}", p);

                    #[allow(clippy::type_complexity)]
                    let mut system_state: SystemState<(
                        Commands,
                        Query<(&mut LocalPlayer, Option<&WorldName>)>,
                    )> = SystemState::new(ecs);
                    let (mut commands, mut query) = system_state.get_mut(ecs);
                    let (local_player, world_name) = query.get_mut(player_entity).unwrap();

                    if let Some(WorldName(world_name)) = world_name {
                        let bundle = p.as_player_bundle(world_name.clone());
                        let mut spawned = commands.spawn((
                            MinecraftEntityId(p.id),
                            LoadedBy(HashSet::from([player_entity])),
                            bundle,
                        ));

                        if let Some(player_info) = local_player.players.get(&p.uuid) {
                            spawned.insert(GameProfileComponent(player_info.profile.clone()));
                        }
                    } else {
                        warn!("got add player packet but we haven't gotten a login packet yet");
                    }

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::InitializeBorder(p) => {
                    debug!("Got initialize border packet {:?}", p);
                }
                ClientboundGamePacket::SetTime(_p) => {
                    // debug!("Got set time packet {:?}", p);
                }
                ClientboundGamePacket::SetDefaultSpawnPosition(p) => {
                    debug!("Got set default spawn position packet {:?}", p);
                }
                ClientboundGamePacket::ContainerSetContent(p) => {
                    debug!("Got container set content packet {:?}", p);
                }
                ClientboundGamePacket::SetHealth(p) => {
                    debug!("Got set health packet {:?}", p);

                    let mut system_state: SystemState<(
                        Query<&mut Health>,
                        EventWriter<DeathEvent>,
                    )> = SystemState::new(ecs);
                    let (mut query, mut death_events) = system_state.get_mut(ecs);
                    let mut health = query.get_mut(player_entity).unwrap();

                    if p.health == 0. && **health != 0. {
                        death_events.send(DeathEvent {
                            entity: player_entity,
                            packet: None,
                        });
                    }

                    **health = p.health;

                    // the `Dead` component is added by the `update_dead` system
                    // in azalea-world and then the `dead_event` system fires
                    // the Death event.
                }
                ClientboundGamePacket::SetExperience(p) => {
                    debug!("Got set experience packet {:?}", p);
                }
                ClientboundGamePacket::TeleportEntity(p) => {
                    let mut system_state: SystemState<(Commands, Query<&mut LocalPlayer>)> =
                        SystemState::new(ecs);
                    let (mut commands, mut query) = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();

                    let world = local_player.world.read();
                    let entity = world.entity_by_id(&MinecraftEntityId(p.id));
                    drop(world);

                    if let Some(entity) = entity {
                        let new_position = p.position;
                        commands.add(RelativeEntityUpdate {
                            entity,
                            partial_world: local_player.partial_world.clone(),
                            update: Box::new(move |entity| {
                                let mut position = entity.get_mut::<Position>().unwrap();
                                **position = new_position;
                            }),
                        });
                    } else {
                        warn!("Got teleport entity packet for unknown entity id {}", p.id);
                    }

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::UpdateAdvancements(p) => {
                    debug!("Got update advancements packet {:?}", p);
                }
                ClientboundGamePacket::RotateHead(_p) => {
                    // debug!("Got rotate head packet {:?}", p);
                }
                ClientboundGamePacket::MoveEntityPos(p) => {
                    let mut system_state: SystemState<(Commands, Query<&LocalPlayer>)> =
                        SystemState::new(ecs);
                    let (mut commands, mut query) = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();

                    let world = local_player.world.read();
                    let entity = world.entity_by_id(&MinecraftEntityId(p.entity_id));
                    drop(world);

                    if let Some(entity) = entity {
                        let delta = p.delta.clone();
                        commands.add(RelativeEntityUpdate {
                            entity,
                            partial_world: local_player.partial_world.clone(),
                            update: Box::new(move |entity_mut| {
                                let mut position = entity_mut.get_mut::<Position>().unwrap();
                                **position = position.with_delta(&delta);
                            }),
                        });
                    } else {
                        warn!(
                            "Got move entity pos packet for unknown entity id {}",
                            p.entity_id
                        );
                    }

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::MoveEntityPosRot(p) => {
                    let mut system_state: SystemState<(Commands, Query<&mut LocalPlayer>)> =
                        SystemState::new(ecs);
                    let (mut commands, mut query) = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();

                    let world = local_player.world.read();
                    let entity = world.entity_by_id(&MinecraftEntityId(p.entity_id));
                    drop(world);

                    if let Some(entity) = entity {
                        let delta = p.delta.clone();
                        commands.add(RelativeEntityUpdate {
                            entity,
                            partial_world: local_player.partial_world.clone(),
                            update: Box::new(move |entity_mut| {
                                let mut position = entity_mut.get_mut::<Position>().unwrap();
                                **position = position.with_delta(&delta);
                            }),
                        });
                    } else {
                        warn!(
                            "Got move entity pos rot packet for unknown entity id {}",
                            p.entity_id
                        );
                    }

                    system_state.apply(ecs);
                }

                ClientboundGamePacket::MoveEntityRot(_p) => {
                    // debug!("Got move entity rot packet {:?}", p);
                }
                ClientboundGamePacket::KeepAlive(p) => {
                    debug!("Got keep alive packet {p:?} for {player_entity:?}");

                    let mut system_state: SystemState<(
                        Query<&mut LocalPlayer>,
                        EventWriter<KeepAliveEvent>,
                    )> = SystemState::new(ecs);
                    let (mut query, mut keepalive_events) = system_state.get_mut(ecs);

                    keepalive_events.send(KeepAliveEvent {
                        entity: player_entity,
                        id: p.id,
                    });

                    let mut local_player = query.get_mut(player_entity).unwrap();
                    local_player.write_packet(ServerboundKeepAlivePacket { id: p.id }.get());
                    debug!("Sent keep alive packet {p:?} for {player_entity:?}");
                }
                ClientboundGamePacket::RemoveEntities(p) => {
                    debug!("Got remove entities packet {:?}", p);
                }
                ClientboundGamePacket::PlayerChat(p) => {
                    debug!("Got player chat packet {:?}", p);

                    let mut system_state: SystemState<EventWriter<ChatReceivedEvent>> =
                        SystemState::new(ecs);
                    let mut chat_events = system_state.get_mut(ecs);

                    chat_events.send(ChatReceivedEvent {
                        entity: player_entity,
                        packet: ChatPacket::Player(Arc::new(p.clone())),
                    });
                }
                ClientboundGamePacket::SystemChat(p) => {
                    debug!("Got system chat packet {:?}", p);

                    let mut system_state: SystemState<EventWriter<ChatReceivedEvent>> =
                        SystemState::new(ecs);
                    let mut chat_events = system_state.get_mut(ecs);

                    chat_events.send(ChatReceivedEvent {
                        entity: player_entity,
                        packet: ChatPacket::System(Arc::new(p.clone())),
                    });
                }
                ClientboundGamePacket::Sound(_p) => {
                    // debug!("Got sound packet {:?}", p);
                }
                ClientboundGamePacket::LevelEvent(p) => {
                    debug!("Got level event packet {:?}", p);
                }
                ClientboundGamePacket::BlockUpdate(p) => {
                    debug!("Got block update packet {:?}", p);

                    let mut system_state: SystemState<Query<&mut LocalPlayer>> =
                        SystemState::new(ecs);
                    let mut query = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();

                    let world = local_player.world.write();

                    world.chunks.set_block_state(&p.pos, p.block_state);
                }
                ClientboundGamePacket::Animate(p) => {
                    debug!("Got animate packet {:?}", p);
                }
                ClientboundGamePacket::SectionBlocksUpdate(p) => {
                    debug!("Got section blocks update packet {:?}", p);
                    let mut system_state: SystemState<Query<&mut LocalPlayer>> =
                        SystemState::new(ecs);
                    let mut query = system_state.get_mut(ecs);
                    let local_player = query.get_mut(player_entity).unwrap();

                    let world = local_player.world.write();

                    for state in &p.states {
                        world
                            .chunks
                            .set_block_state(&(p.section_pos + state.pos.clone()), state.state);
                    }
                }
                ClientboundGamePacket::GameEvent(p) => {
                    debug!("Got game event packet {:?}", p);
                }
                ClientboundGamePacket::LevelParticles(p) => {
                    debug!("Got level particles packet {:?}", p);
                }
                ClientboundGamePacket::ServerData(p) => {
                    debug!("Got server data packet {:?}", p);
                }
                ClientboundGamePacket::SetEquipment(p) => {
                    debug!("Got set equipment packet {:?}", p);
                }
                ClientboundGamePacket::UpdateMobEffect(p) => {
                    debug!("Got update mob effect packet {:?}", p);
                }
                ClientboundGamePacket::AddExperienceOrb(_) => {}
                ClientboundGamePacket::AwardStats(_) => {}
                ClientboundGamePacket::BlockChangedAck(_) => {}
                ClientboundGamePacket::BlockDestruction(_) => {}
                ClientboundGamePacket::BlockEntityData(_) => {}
                ClientboundGamePacket::BlockEvent(p) => {
                    debug!("Got block event packet {:?}", p);
                }
                ClientboundGamePacket::BossEvent(_) => {}
                ClientboundGamePacket::CommandSuggestions(_) => {}
                ClientboundGamePacket::ContainerSetData(_) => {}
                ClientboundGamePacket::ContainerSetSlot(_) => {}
                ClientboundGamePacket::Cooldown(_) => {}
                ClientboundGamePacket::CustomChatCompletions(_) => {}
                ClientboundGamePacket::DeleteChat(_) => {}
                ClientboundGamePacket::Explode(_) => {}
                ClientboundGamePacket::ForgetLevelChunk(_) => {}
                ClientboundGamePacket::HorseScreenOpen(_) => {}
                ClientboundGamePacket::MapItemData(_) => {}
                ClientboundGamePacket::MerchantOffers(_) => {}
                ClientboundGamePacket::MoveVehicle(_) => {}
                ClientboundGamePacket::OpenBook(_) => {}
                ClientboundGamePacket::OpenScreen(_) => {}
                ClientboundGamePacket::OpenSignEditor(_) => {}
                ClientboundGamePacket::Ping(_) => {}
                ClientboundGamePacket::PlaceGhostRecipe(_) => {}
                ClientboundGamePacket::PlayerCombatEnd(_) => {}
                ClientboundGamePacket::PlayerCombatEnter(_) => {}
                ClientboundGamePacket::PlayerCombatKill(p) => {
                    debug!("Got player kill packet {:?}", p);

                    #[allow(clippy::type_complexity)]
                    let mut system_state: SystemState<(
                        Commands,
                        Query<(&MinecraftEntityId, Option<&Dead>)>,
                        EventWriter<DeathEvent>,
                    )> = SystemState::new(ecs);
                    let (mut commands, mut query, mut death_events) = system_state.get_mut(ecs);
                    let (entity_id, dead) = query.get_mut(player_entity).unwrap();

                    if **entity_id == p.player_id && dead.is_none() {
                        commands.entity(player_entity).insert(Dead);
                        death_events.send(DeathEvent {
                            entity: player_entity,
                            packet: Some(p.clone()),
                        });
                    }

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::PlayerLookAt(_) => {}
                ClientboundGamePacket::RemoveMobEffect(_) => {}
                ClientboundGamePacket::ResourcePack(_) => {}
                ClientboundGamePacket::Respawn(p) => {
                    debug!("Got respawn packet {:?}", p);

                    let mut system_state: SystemState<Commands> = SystemState::new(ecs);
                    let mut commands = system_state.get(ecs);

                    // Remove the Dead marker component from the player.
                    commands.entity(player_entity).remove::<Dead>();

                    system_state.apply(ecs);
                }
                ClientboundGamePacket::SelectAdvancementsTab(_) => {}
                ClientboundGamePacket::SetActionBarText(_) => {}
                ClientboundGamePacket::SetBorderCenter(_) => {}
                ClientboundGamePacket::SetBorderLerpSize(_) => {}
                ClientboundGamePacket::SetBorderSize(_) => {}
                ClientboundGamePacket::SetBorderWarningDelay(_) => {}
                ClientboundGamePacket::SetBorderWarningDistance(_) => {}
                ClientboundGamePacket::SetCamera(_) => {}
                ClientboundGamePacket::SetDisplayObjective(_) => {}
                ClientboundGamePacket::SetObjective(_) => {}
                ClientboundGamePacket::SetPassengers(_) => {}
                ClientboundGamePacket::SetPlayerTeam(_) => {}
                ClientboundGamePacket::SetScore(_) => {}
                ClientboundGamePacket::SetSimulationDistance(_) => {}
                ClientboundGamePacket::SetSubtitleText(_) => {}
                ClientboundGamePacket::SetTitleText(_) => {}
                ClientboundGamePacket::SetTitlesAnimation(_) => {}
                ClientboundGamePacket::SoundEntity(_) => {}
                ClientboundGamePacket::StopSound(_) => {}
                ClientboundGamePacket::TabList(_) => {}
                ClientboundGamePacket::TagQuery(_) => {}
                ClientboundGamePacket::TakeItemEntity(_) => {}
                ClientboundGamePacket::DisguisedChat(_) => {}
                ClientboundGamePacket::UpdateEnabledFeatures(_) => {}
                ClientboundGamePacket::ContainerClose(_) => {}
            }
        }
    }
}

impl PacketReceiver {
    /// Loop that reads from the connection and adds the packets to the queue +
    /// runs the schedule.
    pub async fn read_task(self, mut read_conn: ReadConnection<ClientboundGamePacket>) {
        loop {
            match read_conn.read().await {
                Ok(packet) => {
                    self.packets.lock().push(packet);
                    // tell the client to run all the systems
                    self.run_schedule_sender.send(()).unwrap();
                }
                Err(error) => {
                    if !matches!(*error, ReadPacketError::ConnectionClosed) {
                        error!("Error reading packet from Client: {error:?}");
                    }
                    break;
                }
            }
        }
        // TODO: it should send a DisconnectEvent here somehow
        // maybe use a tokio::sync::oneshot that tells it to close and have the
        // receiver in localplayer and have a system that watches that or
        // something?
    }

    /// Consume the [`ServerboundGamePacket`] queue and actually write the
    /// packets to the server. It's like this so writing packets doesn't need to
    /// be awaited.
    pub async fn write_task(
        self,
        mut write_conn: WriteConnection<ServerboundGamePacket>,
        mut write_receiver: mpsc::UnboundedReceiver<ServerboundGamePacket>,
    ) {
        while let Some(packet) = write_receiver.recv().await {
            if let Err(err) = write_conn.write(packet).await {
                error!("Disconnecting because we couldn't write a packet: {err}.");
                break;
            };
        }
        println!("Write task finished");
        // receiver is automatically closed when it's dropped
    }
}
