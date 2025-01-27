use std::{
    collections::HashSet,
    io::Cursor,
    ops::Add,
    sync::{Arc, Weak},
};

use azalea_chat::FormattedText;
use azalea_core::{
    game_type::GameMode,
    math,
    position::{ChunkPos, Vec3},
    resource_location::ResourceLocation,
};
use azalea_entity::{
    indexing::{EntityIdIndex, EntityUuidIndex},
    metadata::{apply_metadata, Health, PlayerMetadataBundle},
    Dead, EntityBundle, EntityKind, LastSentPosition, LoadedBy, LocalEntity, LookDirection,
    Physics, PlayerBundle, Position, RelativeEntityUpdate,
};
use azalea_protocol::{
    packets::{
        game::{
            c_player_combat_kill::ClientboundPlayerCombatKill,
            s_accept_teleportation::ServerboundAcceptTeleportation,
            s_configuration_acknowledged::ServerboundConfigurationAcknowledged,
            s_keep_alive::ServerboundKeepAlive, s_move_player_pos_rot::ServerboundMovePlayerPosRot,
            s_pong::ServerboundPong, ClientboundGamePacket, ServerboundGamePacket,
        },
        Packet,
    },
    read::deserialize_packet,
};
use azalea_world::{Instance, InstanceContainer, InstanceName, MinecraftEntityId, PartialInstance};
use bevy_ecs::{prelude::*, system::SystemState};
use parking_lot::RwLock;
use tracing::{debug, error, trace, warn};
use uuid::Uuid;

use crate::{
    chat::{ChatPacket, ChatReceivedEvent},
    chunks,
    disconnect::DisconnectEvent,
    inventory::{
        ClientSideCloseContainerEvent, Inventory, MenuOpenedEvent, SetContainerContentEvent,
    },
    local_player::{
        GameProfileComponent, Hunger, InstanceHolder, LocalGameMode, PlayerAbilities, TabList,
    },
    movement::{KnockbackEvent, KnockbackType},
    raw_connection::RawConnection,
    ClientInformation, PlayerInfo,
};

/// An event that's sent when we receive a packet.
/// ```
/// # use azalea_client::packet_handling::game::PacketEvent;
/// # use azalea_protocol::packets::game::ClientboundGamePacket;
/// # use bevy_ecs::event::EventReader;
///
/// fn handle_packets(mut events: EventReader<PacketEvent>) {
///     for PacketEvent {
///         entity,
///         packet,
///     } in events.read() {
///         match packet.as_ref() {
///             ClientboundGamePacket::LevelParticles(p) => {
///                 // ...
///             }
///             _ => {}
///         }
///     }
/// }
/// ```
#[derive(Event, Debug, Clone)]
pub struct PacketEvent {
    /// The client entity that received the packet.
    pub entity: Entity,
    /// The packet that was actually received.
    pub packet: Arc<ClientboundGamePacket>,
}

/// A player joined the game (or more specifically, was added to the tab
/// list of a local player).
#[derive(Event, Debug, Clone)]
pub struct AddPlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player left the game (or maybe is still in the game and was just
/// removed from the tab list of a local player).
#[derive(Event, Debug, Clone)]
pub struct RemovePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}
/// A player was updated in the tab list of a local player (gamemode, display
/// name, or latency changed).
#[derive(Event, Debug, Clone)]
pub struct UpdatePlayerEvent {
    /// The local player entity that received this event.
    pub entity: Entity,
    pub info: PlayerInfo,
}

/// Event for when an entity dies. dies. If it's a local player and there's a
/// reason in the death screen, the [`ClientboundPlayerCombatKill`] will
/// be included.
#[derive(Event, Debug, Clone)]
pub struct DeathEvent {
    pub entity: Entity,
    pub packet: Option<ClientboundPlayerCombatKill>,
}

/// A KeepAlive packet is sent from the server to verify that the client is
/// still connected.
#[derive(Event, Debug, Clone)]
pub struct KeepAliveEvent {
    pub entity: Entity,
    /// The ID of the keepalive. This is an arbitrary number, but vanilla
    /// servers use the time to generate this.
    pub id: u64,
}

#[derive(Event, Debug, Clone)]
pub struct ResourcePackEvent {
    pub entity: Entity,
    /// The random ID for this request to download the resource pack. The packet
    /// for replying to a resource pack push must contain the same ID.
    pub id: Uuid,
    pub url: String,
    pub hash: String,
    pub required: bool,
    pub prompt: Option<FormattedText>,
}

/// An instance (aka world, dimension) was loaded by a client.
///
/// Since the instance is given to you as a weak reference, it won't be able to
/// be `upgrade`d if all local players leave it.
#[derive(Event, Debug, Clone)]
pub struct InstanceLoadedEvent {
    pub entity: Entity,
    pub name: ResourceLocation,
    pub instance: Weak<RwLock<Instance>>,
}

pub fn send_packet_events(
    query: Query<(Entity, &RawConnection), With<LocalEntity>>,
    mut packet_events: ResMut<Events<PacketEvent>>,
) {
    // we manually clear and send the events at the beginning of each update
    // since otherwise it'd cause issues with events in process_packet_events
    // running twice
    packet_events.clear();
    for (player_entity, raw_connection) in &query {
        let packets_lock = raw_connection.incoming_packet_queue();
        let mut packets = packets_lock.lock();
        if !packets.is_empty() {
            for raw_packet in packets.iter() {
                let packet =
                    match deserialize_packet::<ClientboundGamePacket>(&mut Cursor::new(raw_packet))
                    {
                        Ok(packet) => packet,
                        Err(err) => {
                            error!("failed to read packet: {err:?}");
                            debug!("packet bytes: {raw_packet:?}");
                            continue;
                        }
                    };
                packet_events.send(PacketEvent {
                    entity: player_entity,
                    packet: Arc::new(packet),
                });
            }
            // clear the packets right after we read them
            packets.clear();
        }
    }
}

pub fn process_packet_events(ecs: &mut World) {
    let mut events_owned = Vec::<(Entity, Arc<ClientboundGamePacket>)>::new();
    {
        let mut system_state = SystemState::<EventReader<PacketEvent>>::new(ecs);
        let mut events = system_state.get_mut(ecs);
        for PacketEvent {
            entity: player_entity,
            packet,
        } in events.read()
        {
            // we do this so `ecs` isn't borrowed for the whole loop
            events_owned.push((*player_entity, packet.clone()));
        }
    }
    for (player_entity, packet) in events_owned {
        let packet_clone = packet.clone();
        let packet_ref = packet_clone.as_ref();
        match packet_ref {
            ClientboundGamePacket::Login(p) => {
                debug!("Got login packet");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<(
                        &GameProfileComponent,
                        &ClientInformation,
                        Option<&mut InstanceName>,
                        Option<&mut LoadedBy>,
                        &mut EntityIdIndex,
                        &mut InstanceHolder,
                    )>,
                    EventWriter<InstanceLoadedEvent>,
                    ResMut<InstanceContainer>,
                    ResMut<EntityUuidIndex>,
                    EventWriter<SendPacketEvent>,
                )> = SystemState::new(ecs);
                let (
                    mut commands,
                    mut query,
                    mut instance_loaded_events,
                    mut instance_container,
                    mut entity_uuid_index,
                    mut send_packet_events,
                ) = system_state.get_mut(ecs);
                let (
                    game_profile,
                    client_information,
                    instance_name,
                    loaded_by,
                    mut entity_id_index,
                    mut instance_holder,
                ) = query.get_mut(player_entity).unwrap();

                {
                    let new_instance_name = p.common.dimension.clone();

                    if let Some(mut instance_name) = instance_name {
                        *instance_name = instance_name.clone();
                    } else {
                        commands
                            .entity(player_entity)
                            .insert(InstanceName(new_instance_name.clone()));
                    }

                    let Some(dimension_type_element) =
                        instance_holder.instance.read().registries.dimension_type()
                    else {
                        error!("Server didn't send dimension type registry, can't log in");
                        continue;
                    };

                    let dimension_name = ResourceLocation::new(&p.common.dimension.to_string());

                    let Some(dimension) = dimension_type_element.map.get(&dimension_name) else {
                        error!("No dimension_type with name {dimension_name}");
                        continue;
                    };

                    // add this world to the instance_container (or don't if it's already
                    // there)
                    let weak_instance = instance_container.insert(
                        new_instance_name.clone(),
                        dimension.height,
                        dimension.min_y,
                    );
                    instance_loaded_events.send(InstanceLoadedEvent {
                        entity: player_entity,
                        name: new_instance_name.clone(),
                        instance: Arc::downgrade(&weak_instance),
                    });

                    // set the partial_world to an empty world
                    // (when we add chunks or entities those will be in the
                    // instance_container)

                    *instance_holder.partial_instance.write() = PartialInstance::new(
                        azalea_world::chunk_storage::calculate_chunk_storage_range(
                            client_information.view_distance.into(),
                        ),
                        // this argument makes it so other clients don't update this player entity
                        // in a shared instance
                        Some(player_entity),
                    );
                    {
                        let new_registries = &mut weak_instance.write().registries;
                        // add the registries from this instance to the weak instance
                        for (registry_name, registry) in
                            &instance_holder.instance.read().registries.map
                        {
                            new_registries
                                .map
                                .insert(registry_name.clone(), registry.clone());
                        }
                    }
                    instance_holder.instance = weak_instance;

                    let player_bundle = PlayerBundle {
                        entity: EntityBundle::new(
                            game_profile.uuid,
                            Vec3::default(),
                            azalea_registry::EntityKind::Player,
                            new_instance_name,
                        ),
                        metadata: PlayerMetadataBundle::default(),
                    };
                    let entity_id = MinecraftEntityId(p.player_id);
                    // insert our components into the ecs :)
                    commands.entity(player_entity).insert((
                        entity_id,
                        LocalGameMode {
                            current: p.common.game_type,
                            previous: p.common.previous_game_type.into(),
                        },
                        player_bundle,
                    ));

                    azalea_entity::indexing::add_entity_to_indexes(
                        entity_id,
                        player_entity,
                        Some(game_profile.uuid),
                        &mut entity_id_index,
                        &mut entity_uuid_index,
                        &mut instance_holder.instance.write(),
                    );

                    // update or insert loaded_by
                    if let Some(mut loaded_by) = loaded_by {
                        loaded_by.insert(player_entity);
                    } else {
                        commands
                            .entity(player_entity)
                            .insert(LoadedBy(HashSet::from_iter(vec![player_entity])));
                    }
                }

                // send the client information that we have set
                debug!(
                    "Sending client information because login: {:?}",
                    client_information
                );
                send_packet_events.send(SendPacketEvent::new(player_entity,
                    azalea_protocol::packets::game::s_client_information::ServerboundClientInformation { information: client_information.clone() },
                ));

                system_state.apply(ecs);
            }
            ClientboundGamePacket::SetChunkCacheRadius(p) => {
                debug!("Got set chunk cache radius packet {p:?}");
            }

            ClientboundGamePacket::ChunkBatchStart(_p) => {
                // the packet is empty, just a marker to tell us when the batch starts and ends
                debug!("Got chunk batch start");
                let mut system_state: SystemState<EventWriter<chunks::ChunkBatchStartEvent>> =
                    SystemState::new(ecs);
                let mut chunk_batch_start_events = system_state.get_mut(ecs);

                chunk_batch_start_events.send(chunks::ChunkBatchStartEvent {
                    entity: player_entity,
                });
            }
            ClientboundGamePacket::ChunkBatchFinished(p) => {
                debug!("Got chunk batch finished {p:?}");

                let mut system_state: SystemState<EventWriter<chunks::ChunkBatchFinishedEvent>> =
                    SystemState::new(ecs);
                let mut chunk_batch_start_events = system_state.get_mut(ecs);

                chunk_batch_start_events.send(chunks::ChunkBatchFinishedEvent {
                    entity: player_entity,
                    batch_size: p.batch_size,
                });
            }

            ClientboundGamePacket::CustomPayload(p) => {
                debug!("Got custom payload packet {p:?}");
            }
            ClientboundGamePacket::ChangeDifficulty(p) => {
                debug!("Got difficulty packet {p:?}");
            }
            ClientboundGamePacket::Commands(_p) => {
                debug!("Got declare commands packet");
            }
            ClientboundGamePacket::PlayerAbilities(p) => {
                debug!("Got player abilities packet {p:?}");
                let mut system_state: SystemState<Query<&mut PlayerAbilities>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut player_abilities = query.get_mut(player_entity).unwrap();

                *player_abilities = PlayerAbilities::from(p);
            }
            ClientboundGamePacket::SetCursorItem(p) => {
                debug!("Got set cursor item packet {p:?}");
            }
            ClientboundGamePacket::UpdateTags(_p) => {
                debug!("Got update tags packet");
            }
            ClientboundGamePacket::Disconnect(p) => {
                warn!("Got disconnect packet {p:?}");
                let mut system_state: SystemState<EventWriter<DisconnectEvent>> =
                    SystemState::new(ecs);
                let mut disconnect_events = system_state.get_mut(ecs);
                disconnect_events.send(DisconnectEvent {
                    entity: player_entity,
                    reason: Some(p.reason.clone()),
                });
            }
            ClientboundGamePacket::UpdateRecipes(_p) => {
                debug!("Got update recipes packet");
            }
            ClientboundGamePacket::EntityEvent(_p) => {
                // debug!("Got entity event packet {p:?}");
            }
            ClientboundGamePacket::PlayerPosition(p) => {
                debug!("Got player position packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Query<(
                        &mut Physics,
                        &mut LookDirection,
                        &mut Position,
                        &mut LastSentPosition,
                    )>,
                    EventWriter<SendPacketEvent>,
                )> = SystemState::new(ecs);
                let (mut query, mut send_packet_events) = system_state.get_mut(ecs);
                let Ok((mut physics, mut direction, mut position, mut last_sent_position)) =
                    query.get_mut(player_entity)
                else {
                    continue;
                };

                **last_sent_position = **position;

                fn apply_change<T: Add<Output = T>>(base: T, condition: bool, change: T) -> T {
                    if condition {
                        base + change
                    } else {
                        change
                    }
                }

                let new_x = apply_change(position.x, p.relative.x, p.change.pos.x);
                let new_y = apply_change(position.y, p.relative.y, p.change.pos.y);
                let new_z = apply_change(position.z, p.relative.z, p.change.pos.z);

                let new_y_rot = apply_change(
                    direction.y_rot,
                    p.relative.y_rot,
                    p.change.look_direction.y_rot,
                );
                let new_x_rot = apply_change(
                    direction.x_rot,
                    p.relative.x_rot,
                    p.change.look_direction.x_rot,
                );

                let mut new_delta_from_rotations = physics.velocity;
                if p.relative.rotate_delta {
                    let y_rot_delta = direction.y_rot - new_y_rot;
                    let x_rot_delta = direction.x_rot - new_x_rot;
                    new_delta_from_rotations = new_delta_from_rotations
                        .x_rot(math::to_radians(x_rot_delta as f64) as f32)
                        .y_rot(math::to_radians(y_rot_delta as f64) as f32);
                }

                let new_delta = Vec3::new(
                    apply_change(
                        new_delta_from_rotations.x,
                        p.relative.delta_x,
                        p.change.delta.x,
                    ),
                    apply_change(
                        new_delta_from_rotations.y,
                        p.relative.delta_y,
                        p.change.delta.y,
                    ),
                    apply_change(
                        new_delta_from_rotations.z,
                        p.relative.delta_z,
                        p.change.delta.z,
                    ),
                );

                // apply the updates

                physics.velocity = new_delta;

                (direction.y_rot, direction.x_rot) = (new_y_rot, new_x_rot);

                let new_pos = Vec3::new(new_x, new_y, new_z);
                if new_pos != **position {
                    **position = new_pos;
                }

                // old_pos is set to the current position when we're teleported
                physics.set_old_pos(&position);

                // send the relevant packets

                send_packet_events.send(SendPacketEvent::new(
                    player_entity,
                    ServerboundAcceptTeleportation { id: p.id },
                ));
                send_packet_events.send(SendPacketEvent::new(
                    player_entity,
                    ServerboundMovePlayerPosRot {
                        pos: new_pos,
                        look_direction: LookDirection::new(new_y_rot, new_x_rot),
                        // this is always false
                        on_ground: false,
                    },
                ));
            }
            ClientboundGamePacket::PlayerInfoUpdate(p) => {
                debug!("Got player info packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Query<&mut TabList>,
                    EventWriter<AddPlayerEvent>,
                    EventWriter<UpdatePlayerEvent>,
                    ResMut<TabList>,
                )> = SystemState::new(ecs);
                let (
                    mut query,
                    mut add_player_events,
                    mut update_player_events,
                    mut tab_list_resource,
                ) = system_state.get_mut(ecs);
                let mut tab_list = query.get_mut(player_entity).unwrap();

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
                        tab_list.insert(updated_info.profile.uuid, info.clone());
                        add_player_events.send(AddPlayerEvent {
                            entity: player_entity,
                            info: info.clone(),
                        });
                    } else if let Some(info) = tab_list.get_mut(&updated_info.profile.uuid) {
                        // `else if` because the block for add_player above
                        // already sets all the fields
                        if p.actions.update_game_mode {
                            info.gamemode = updated_info.game_mode;
                        }
                        if p.actions.update_latency {
                            info.latency = updated_info.latency;
                        }
                        if p.actions.update_display_name {
                            info.display_name.clone_from(&updated_info.display_name);
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

                *tab_list_resource = tab_list.clone();
            }
            ClientboundGamePacket::PlayerInfoRemove(p) => {
                let mut system_state: SystemState<(
                    Query<&mut TabList>,
                    EventWriter<RemovePlayerEvent>,
                    ResMut<TabList>,
                )> = SystemState::new(ecs);
                let (mut query, mut remove_player_events, mut tab_list_resource) =
                    system_state.get_mut(ecs);
                let mut tab_list = query.get_mut(player_entity).unwrap();

                for uuid in &p.profile_ids {
                    if let Some(info) = tab_list.remove(uuid) {
                        remove_player_events.send(RemovePlayerEvent {
                            entity: player_entity,
                            info,
                        });
                    }
                    tab_list_resource.remove(uuid);
                }
            }
            ClientboundGamePacket::SetChunkCacheCenter(p) => {
                debug!("Got chunk cache center packet {p:?}");

                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let instance_holder = query.get_mut(player_entity).unwrap();
                let mut partial_world = instance_holder.partial_instance.write();

                partial_world
                    .chunks
                    .update_view_center(ChunkPos::new(p.x, p.z));
            }
            ClientboundGamePacket::ChunksBiomes(_) => {}
            ClientboundGamePacket::LightUpdate(_p) => {
                // debug!("Got light update packet {p:?}");
            }
            ClientboundGamePacket::LevelChunkWithLight(p) => {
                debug!("Got chunk with light packet {} {}", p.x, p.z);

                let mut system_state: SystemState<EventWriter<chunks::ReceiveChunkEvent>> =
                    SystemState::new(ecs);
                let mut receive_chunk_events = system_state.get_mut(ecs);
                receive_chunk_events.send(chunks::ReceiveChunkEvent {
                    entity: player_entity,
                    packet: p.clone(),
                });
            }
            ClientboundGamePacket::AddEntity(p) => {
                debug!("Got add entity packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&mut EntityIdIndex, Option<&InstanceName>, Option<&TabList>)>,
                    Query<&mut LoadedBy>,
                    Query<Entity>,
                    Res<InstanceContainer>,
                    ResMut<EntityUuidIndex>,
                )> = SystemState::new(ecs);
                let (
                    mut commands,
                    mut query,
                    mut loaded_by_query,
                    entity_query,
                    instance_container,
                    mut entity_uuid_index,
                ) = system_state.get_mut(ecs);
                let (mut entity_id_index, instance_name, tab_list) =
                    query.get_mut(player_entity).unwrap();

                let entity_id = MinecraftEntityId(p.id);

                let Some(instance_name) = instance_name else {
                    warn!("got add player packet but we haven't gotten a login packet yet");
                    continue;
                };

                // check if the entity already exists, and if it does then only add to LoadedBy
                let instance = instance_container.get(instance_name).unwrap();
                if let Some(&ecs_entity) = instance.read().entity_by_id.get(&entity_id) {
                    // entity already exists
                    let Ok(mut loaded_by) = loaded_by_query.get_mut(ecs_entity) else {
                        // LoadedBy for this entity isn't in the ecs! figure out what went wrong
                        // and print an error

                        let entity_in_ecs = entity_query.get(ecs_entity).is_ok();

                        if entity_in_ecs {
                            error!("LoadedBy for entity {entity_id:?} ({ecs_entity:?}) isn't in the ecs, but the entity is in entity_by_id");
                        } else {
                            error!("Entity {entity_id:?} ({ecs_entity:?}) isn't in the ecs, but the entity is in entity_by_id");
                        }
                        continue;
                    };
                    loaded_by.insert(player_entity);

                    // per-client id index
                    entity_id_index.insert(entity_id, ecs_entity);

                    debug!("added to LoadedBy of entity {ecs_entity:?} with id {entity_id:?}");
                    continue;
                };

                // entity doesn't exist in the global index!

                let bundle = p.as_entity_bundle((**instance_name).clone());
                let mut spawned =
                    commands.spawn((entity_id, LoadedBy(HashSet::from([player_entity])), bundle));
                let ecs_entity: Entity = spawned.id();
                debug!("spawned entity {ecs_entity:?} with id {entity_id:?}");

                azalea_entity::indexing::add_entity_to_indexes(
                    entity_id,
                    ecs_entity,
                    Some(p.uuid),
                    &mut entity_id_index,
                    &mut entity_uuid_index,
                    &mut instance.write(),
                );

                // add the GameProfileComponent if the uuid is in the tab list
                if let Some(tab_list) = tab_list {
                    // (technically this makes it possible for non-player entities to have
                    // GameProfileComponents but the server would have to be doing something
                    // really weird)
                    if let Some(player_info) = tab_list.get(&p.uuid) {
                        spawned.insert(GameProfileComponent(player_info.profile.clone()));
                    }
                }

                // the bundle doesn't include the default entity metadata so we add that
                // separately
                p.apply_metadata(&mut spawned);

                system_state.apply(ecs);
            }
            ClientboundGamePacket::SetEntityData(p) => {
                debug!("Got set entity data packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&EntityIdIndex, &InstanceHolder)>,
                    Query<&EntityKind>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query, entity_kind_query) = system_state.get_mut(ecs);
                let (entity_id_index, instance_holder) = query.get_mut(player_entity).unwrap();

                let entity = entity_id_index.get(MinecraftEntityId(p.id));

                let Some(entity) = entity else {
                    warn!("Server sent an entity data packet for an entity id ({}) that we don't know about", p.id);
                    continue;
                };
                let entity_kind = *entity_kind_query.get(entity).unwrap();

                let packed_items = p.packed_items.clone().to_vec();

                // we use RelativeEntityUpdate because it makes sure changes aren't made
                // multiple times
                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity| {
                        let entity_id = entity.id();
                        entity.world_scope(|world| {
                            let mut commands_system_state = SystemState::<Commands>::new(world);
                            let mut commands = commands_system_state.get_mut(world);
                            let mut entity_commands = commands.entity(entity_id);
                            if let Err(e) =
                                apply_metadata(&mut entity_commands, *entity_kind, packed_items)
                            {
                                warn!("{e}");
                            }
                            commands_system_state.apply(world);
                        });
                    }),
                });

                system_state.apply(ecs);
            }
            ClientboundGamePacket::UpdateAttributes(_p) => {
                // debug!("Got update attributes packet {p:?}");
            }
            ClientboundGamePacket::SetEntityMotion(p) => {
                // vanilla servers use this packet for knockback, but note that the Explode
                // packet is also sometimes used by servers for knockback

                let mut system_state: SystemState<(
                    Commands,
                    Query<(&EntityIdIndex, &InstanceHolder)>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let (entity_id_index, instance_holder) = query.get_mut(player_entity).unwrap();

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.id)) else {
                    // note that this log (and some other ones like the one in RemoveEntities)
                    // sometimes happens when killing mobs. it seems to be a vanilla bug, which is
                    // why it's a debug log instead of a warning
                    debug!(
                        "Got set entity motion packet for unknown entity id {}",
                        p.id
                    );
                    continue;
                };

                // this is to make sure the same entity velocity update doesn't get sent
                // multiple times when in swarms

                let knockback = KnockbackType::Set(Vec3 {
                    x: p.xa as f64 / 8000.,
                    y: p.ya as f64 / 8000.,
                    z: p.za as f64 / 8000.,
                });

                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity_mut| {
                        entity_mut.world_scope(|world| {
                            world.send_event(KnockbackEvent { entity, knockback })
                        });
                    }),
                });

                system_state.apply(ecs);
            }
            ClientboundGamePacket::SetEntityLink(p) => {
                debug!("Got set entity link packet {p:?}");
            }
            ClientboundGamePacket::InitializeBorder(p) => {
                debug!("Got initialize border packet {p:?}");
            }
            ClientboundGamePacket::SetTime(_p) => {
                // debug!("Got set time packet {p:?}");
            }
            ClientboundGamePacket::SetDefaultSpawnPosition(p) => {
                debug!("Got set default spawn position packet {p:?}");
            }
            ClientboundGamePacket::SetHealth(p) => {
                debug!("Got set health packet {p:?}");

                let mut system_state: SystemState<Query<(&mut Health, &mut Hunger)>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let (mut health, mut hunger) = query.get_mut(player_entity).unwrap();

                **health = p.health;
                (hunger.food, hunger.saturation) = (p.food, p.saturation);

                // the `Dead` component is added by the `update_dead` system
                // in azalea-world and then the `dead_event` system fires
                // the Death event.
            }
            ClientboundGamePacket::SetExperience(p) => {
                debug!("Got set experience packet {p:?}");
            }
            ClientboundGamePacket::TeleportEntity(p) => {
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&EntityIdIndex, &InstanceHolder)>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let (entity_id_index, instance_holder) = query.get_mut(player_entity).unwrap();

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.id)) else {
                    warn!("Got teleport entity packet for unknown entity id {}", p.id);
                    continue;
                };

                let new_pos = p.change.pos;
                let new_look_direction = LookDirection {
                    x_rot: (p.change.look_direction.x_rot as i32 * 360) as f32 / 256.,
                    y_rot: (p.change.look_direction.y_rot as i32 * 360) as f32 / 256.,
                };
                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity| {
                        let mut position = entity.get_mut::<Position>().unwrap();
                        if new_pos != **position {
                            **position = new_pos;
                        }
                        let position = *position;
                        let mut look_direction = entity.get_mut::<LookDirection>().unwrap();
                        if new_look_direction != *look_direction {
                            *look_direction = new_look_direction;
                        }
                        // old_pos is set to the current position when we're teleported
                        let mut physics = entity.get_mut::<Physics>().unwrap();
                        physics.set_old_pos(&position);
                    }),
                });

                system_state.apply(ecs);
            }
            ClientboundGamePacket::UpdateAdvancements(p) => {
                debug!("Got update advancements packet {p:?}");
            }
            ClientboundGamePacket::RotateHead(_p) => {
                // debug!("Got rotate head packet {p:?}");
            }
            ClientboundGamePacket::MoveEntityPos(p) => {
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&EntityIdIndex, &InstanceHolder)>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let (entity_id_index, instance_holder) = query.get_mut(player_entity).unwrap();

                debug!("Got move entity pos packet {p:?}");

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.entity_id)) else {
                    debug!(
                        "Got move entity pos packet for unknown entity id {}",
                        p.entity_id
                    );
                    continue;
                };

                let new_delta = p.delta.clone();
                let new_on_ground = p.on_ground;
                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity_mut| {
                        let mut physics = entity_mut.get_mut::<Physics>().unwrap();
                        let new_pos = physics.vec_delta_codec.decode(
                            new_delta.xa as i64,
                            new_delta.ya as i64,
                            new_delta.za as i64,
                        );
                        physics.vec_delta_codec.set_base(new_pos);
                        physics.set_on_ground(new_on_ground);

                        let mut position = entity_mut.get_mut::<Position>().unwrap();
                        if new_pos != **position {
                            **position = new_pos;
                        }
                    }),
                });

                system_state.apply(ecs);
            }
            ClientboundGamePacket::MoveEntityPosRot(p) => {
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&EntityIdIndex, &InstanceHolder)>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let (entity_id_index, instance_holder) = query.get_mut(player_entity).unwrap();

                debug!("Got move entity pos rot packet {p:?}");

                let entity = entity_id_index.get(MinecraftEntityId(p.entity_id));

                if let Some(entity) = entity {
                    let new_delta = p.delta.clone();
                    let new_look_direction = LookDirection {
                        x_rot: (p.x_rot as i32 * 360) as f32 / 256.,
                        y_rot: (p.y_rot as i32 * 360) as f32 / 256.,
                    };

                    let new_on_ground = p.on_ground;

                    commands.entity(entity).queue(RelativeEntityUpdate {
                        partial_world: instance_holder.partial_instance.clone(),
                        update: Box::new(move |entity_mut| {
                            let mut physics = entity_mut.get_mut::<Physics>().unwrap();
                            let new_pos = physics.vec_delta_codec.decode(
                                new_delta.xa as i64,
                                new_delta.ya as i64,
                                new_delta.za as i64,
                            );
                            physics.vec_delta_codec.set_base(new_pos);
                            physics.set_on_ground(new_on_ground);

                            let mut position = entity_mut.get_mut::<Position>().unwrap();
                            if new_pos != **position {
                                **position = new_pos;
                            }

                            let mut look_direction = entity_mut.get_mut::<LookDirection>().unwrap();
                            if new_look_direction != *look_direction {
                                *look_direction = new_look_direction;
                            }
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

            ClientboundGamePacket::MoveEntityRot(p) => {
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&EntityIdIndex, &InstanceHolder)>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let (entity_id_index, instance_holder) = query.get_mut(player_entity).unwrap();

                let entity = entity_id_index.get(MinecraftEntityId(p.entity_id));

                if let Some(entity) = entity {
                    let new_look_direction = LookDirection {
                        x_rot: (p.x_rot as i32 * 360) as f32 / 256.,
                        y_rot: (p.y_rot as i32 * 360) as f32 / 256.,
                    };
                    let new_on_ground = p.on_ground;

                    commands.entity(entity).queue(RelativeEntityUpdate {
                        partial_world: instance_holder.partial_instance.clone(),
                        update: Box::new(move |entity_mut| {
                            let mut physics = entity_mut.get_mut::<Physics>().unwrap();
                            physics.set_on_ground(new_on_ground);

                            let mut look_direction = entity_mut.get_mut::<LookDirection>().unwrap();
                            if new_look_direction != *look_direction {
                                *look_direction = new_look_direction;
                            }
                        }),
                    });
                } else {
                    warn!(
                        "Got move entity rot packet for unknown entity id {}",
                        p.entity_id
                    );
                }

                system_state.apply(ecs);
            }
            ClientboundGamePacket::KeepAlive(p) => {
                debug!("Got keep alive packet {p:?} for {player_entity:?}");

                let mut system_state: SystemState<(
                    EventWriter<KeepAliveEvent>,
                    EventWriter<SendPacketEvent>,
                )> = SystemState::new(ecs);
                let (mut keepalive_events, mut send_packet_events) = system_state.get_mut(ecs);

                keepalive_events.send(KeepAliveEvent {
                    entity: player_entity,
                    id: p.id,
                });
                send_packet_events.send(SendPacketEvent::new(
                    player_entity,
                    ServerboundKeepAlive { id: p.id },
                ));
            }
            ClientboundGamePacket::RemoveEntities(p) => {
                debug!("Got remove entities packet {p:?}");

                let mut system_state: SystemState<(
                    Query<&mut EntityIdIndex>,
                    Query<&mut LoadedBy>,
                )> = SystemState::new(ecs);

                let (mut query, mut entity_query) = system_state.get_mut(ecs);
                let Ok(mut entity_id_index) = query.get_mut(player_entity) else {
                    warn!("our local player doesn't have EntityIdIndex");
                    continue;
                };

                for &id in &p.entity_ids {
                    let Some(entity) = entity_id_index.remove(MinecraftEntityId(id)) else {
                        debug!("Tried to remove entity with id {id} but it wasn't in the EntityIdIndex");
                        continue;
                    };
                    let Ok(mut loaded_by) = entity_query.get_mut(entity) else {
                        warn!(
                            "tried to despawn entity {id} but it doesn't have a LoadedBy component",
                        );
                        continue;
                    };

                    // the [`remove_despawned_entities_from_indexes`] system will despawn the entity
                    // if it's not loaded by anything anymore

                    // also we can't just ecs.despawn because if we're in a swarm then the entity
                    // might still be loaded by another client

                    loaded_by.remove(&player_entity);
                }
            }
            ClientboundGamePacket::PlayerChat(p) => {
                debug!("Got player chat packet {p:?}");

                let mut system_state: SystemState<EventWriter<ChatReceivedEvent>> =
                    SystemState::new(ecs);
                let mut chat_events = system_state.get_mut(ecs);

                chat_events.send(ChatReceivedEvent {
                    entity: player_entity,
                    packet: ChatPacket::Player(Arc::new(p.clone())),
                });
            }
            ClientboundGamePacket::SystemChat(p) => {
                debug!("Got system chat packet {p:?}");

                let mut system_state: SystemState<EventWriter<ChatReceivedEvent>> =
                    SystemState::new(ecs);
                let mut chat_events = system_state.get_mut(ecs);

                chat_events.send(ChatReceivedEvent {
                    entity: player_entity,
                    packet: ChatPacket::System(Arc::new(p.clone())),
                });
            }
            ClientboundGamePacket::DisguisedChat(p) => {
                debug!("Got disguised chat packet {p:?}");

                let mut system_state: SystemState<EventWriter<ChatReceivedEvent>> =
                    SystemState::new(ecs);
                let mut chat_events = system_state.get_mut(ecs);

                chat_events.send(ChatReceivedEvent {
                    entity: player_entity,
                    packet: ChatPacket::Disguised(Arc::new(p.clone())),
                });
            }
            ClientboundGamePacket::Sound(_p) => {
                // debug!("Got sound packet {p:?}");
            }
            ClientboundGamePacket::LevelEvent(p) => {
                debug!("Got level event packet {p:?}");
            }
            ClientboundGamePacket::BlockUpdate(p) => {
                debug!("Got block update packet {p:?}");

                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.write();

                world.chunks.set_block_state(&p.pos, p.block_state);
            }
            ClientboundGamePacket::Animate(p) => {
                debug!("Got animate packet {p:?}");
            }
            ClientboundGamePacket::SectionBlocksUpdate(p) => {
                debug!("Got section blocks update packet {p:?}");
                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let world = local_player.instance.write();

                for state in &p.states {
                    world
                        .chunks
                        .set_block_state(&(p.section_pos + state.pos), state.state);
                }
            }
            ClientboundGamePacket::GameEvent(p) => {
                use azalea_protocol::packets::game::c_game_event::EventType;

                debug!("Got game event packet {p:?}");

                #[allow(clippy::single_match)]
                match p.event {
                    EventType::ChangeGameMode => {
                        let mut system_state: SystemState<Query<&mut LocalGameMode>> =
                            SystemState::new(ecs);
                        let mut query = system_state.get_mut(ecs);
                        let mut local_game_mode = query.get_mut(player_entity).unwrap();
                        if let Some(new_game_mode) = GameMode::from_id(p.param as u8) {
                            local_game_mode.current = new_game_mode;
                        }
                    }
                    _ => {}
                }
            }
            ClientboundGamePacket::LevelParticles(p) => {
                debug!("Got level particles packet {p:?}");
            }
            ClientboundGamePacket::ServerData(p) => {
                debug!("Got server data packet {p:?}");
            }
            ClientboundGamePacket::SetEquipment(p) => {
                debug!("Got set equipment packet {p:?}");
            }
            ClientboundGamePacket::UpdateMobEffect(p) => {
                debug!("Got update mob effect packet {p:?}");
            }
            ClientboundGamePacket::AwardStats(_) => {}
            ClientboundGamePacket::BlockChangedAck(_) => {}
            ClientboundGamePacket::BlockDestruction(_) => {}
            ClientboundGamePacket::BlockEntityData(_) => {}
            ClientboundGamePacket::BlockEvent(p) => {
                debug!("Got block event packet {p:?}");
            }
            ClientboundGamePacket::BossEvent(_) => {}
            ClientboundGamePacket::CommandSuggestions(_) => {}
            ClientboundGamePacket::ContainerSetContent(p) => {
                debug!("Got container set content packet {p:?}");

                let mut system_state: SystemState<(
                    Query<&mut Inventory>,
                    EventWriter<SetContainerContentEvent>,
                )> = SystemState::new(ecs);
                let (mut query, mut events) = system_state.get_mut(ecs);
                let mut inventory = query.get_mut(player_entity).unwrap();

                // container id 0 is always the player's inventory
                if p.container_id == 0 {
                    // this is just so it has the same type as the `else` block
                    for (i, slot) in p.items.iter().enumerate() {
                        if let Some(slot_mut) = inventory.inventory_menu.slot_mut(i) {
                            *slot_mut = slot.clone();
                        }
                    }
                } else {
                    events.send(SetContainerContentEvent {
                        entity: player_entity,
                        slots: p.items.clone(),
                        container_id: p.container_id,
                    });
                }
            }
            ClientboundGamePacket::ContainerSetData(p) => {
                debug!("Got container set data packet {p:?}");
                // let mut system_state: SystemState<Query<&mut
                // InventoryComponent>> =
                //     SystemState::new(ecs);
                // let mut query = system_state.get_mut(ecs);
                // let mut inventory =
                // query.get_mut(player_entity).unwrap();

                // TODO: handle ContainerSetData packet
                // this is used for various things like the furnace progress
                // bar
                // see https://wiki.vg/Protocol#Set_Container_Property
            }
            ClientboundGamePacket::ContainerSetSlot(p) => {
                debug!("Got container set slot packet {p:?}");

                let mut system_state: SystemState<Query<&mut Inventory>> = SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let mut inventory = query.get_mut(player_entity).unwrap();

                if p.container_id == -1 {
                    // -1 means carried item
                    inventory.carried = p.item_stack.clone();
                } else if p.container_id == -2 {
                    if let Some(slot) = inventory.inventory_menu.slot_mut(p.slot.into()) {
                        *slot = p.item_stack.clone();
                    }
                } else {
                    let is_creative_mode_and_inventory_closed = false;
                    // technically minecraft has slightly different behavior here if you're in
                    // creative mode and have your inventory open
                    if p.container_id == 0
                        && azalea_inventory::Player::is_hotbar_slot(p.slot.into())
                    {
                        // minecraft also sets a "pop time" here which is used for an animation
                        // but that's not really necessary
                        if let Some(slot) = inventory.inventory_menu.slot_mut(p.slot.into()) {
                            *slot = p.item_stack.clone();
                        }
                    } else if p.container_id == inventory.id
                        && (p.container_id != 0 || !is_creative_mode_and_inventory_closed)
                    {
                        // var2.containerMenu.setItem(var4, var1.getStateId(), var3);
                        if let Some(slot) = inventory.menu_mut().slot_mut(p.slot.into()) {
                            *slot = p.item_stack.clone();
                            inventory.state_id = p.state_id;
                        }
                    }
                }
            }
            ClientboundGamePacket::ContainerClose(_p) => {
                // there's p.container_id but minecraft doesn't actually check it
                let mut system_state: SystemState<EventWriter<ClientSideCloseContainerEvent>> =
                    SystemState::new(ecs);
                let mut client_side_close_container_events = system_state.get_mut(ecs);
                client_side_close_container_events.send(ClientSideCloseContainerEvent {
                    entity: player_entity,
                });
            }
            ClientboundGamePacket::Cooldown(_) => {}
            ClientboundGamePacket::CustomChatCompletions(_) => {}
            ClientboundGamePacket::DeleteChat(_) => {}
            ClientboundGamePacket::Explode(p) => {
                trace!("Got explode packet {p:?}");
                if let Some(knockback) = p.knockback {
                    let mut system_state: SystemState<EventWriter<KnockbackEvent>> =
                        SystemState::new(ecs);
                    let mut knockback_events = system_state.get_mut(ecs);

                    knockback_events.send(KnockbackEvent {
                        entity: player_entity,
                        knockback: KnockbackType::Set(knockback),
                    });

                    system_state.apply(ecs);
                }
            }
            ClientboundGamePacket::ForgetLevelChunk(p) => {
                debug!("Got forget level chunk packet {p:?}");

                let mut system_state: SystemState<Query<&mut InstanceHolder>> =
                    SystemState::new(ecs);
                let mut query = system_state.get_mut(ecs);
                let local_player = query.get_mut(player_entity).unwrap();

                let mut partial_instance = local_player.partial_instance.write();

                partial_instance.chunks.limited_set(&p.pos, None);
            }
            ClientboundGamePacket::HorseScreenOpen(_) => {}
            ClientboundGamePacket::MapItemData(_) => {}
            ClientboundGamePacket::MerchantOffers(_) => {}
            ClientboundGamePacket::MoveVehicle(_) => {}
            ClientboundGamePacket::OpenBook(_) => {}
            ClientboundGamePacket::OpenScreen(p) => {
                debug!("Got open screen packet {p:?}");
                let mut system_state: SystemState<EventWriter<MenuOpenedEvent>> =
                    SystemState::new(ecs);
                let mut menu_opened_events = system_state.get_mut(ecs);
                menu_opened_events.send(MenuOpenedEvent {
                    entity: player_entity,
                    window_id: p.container_id,
                    menu_type: p.menu_type,
                    title: p.title.to_owned(),
                });
            }
            ClientboundGamePacket::OpenSignEditor(_) => {}
            ClientboundGamePacket::Ping(p) => {
                debug!("Got ping packet {p:?}");

                let mut system_state: SystemState<EventWriter<SendPacketEvent>> =
                    SystemState::new(ecs);
                let mut send_packet_events = system_state.get_mut(ecs);

                send_packet_events.send(SendPacketEvent::new(
                    player_entity,
                    ServerboundPong { id: p.id },
                ));
            }
            ClientboundGamePacket::PlaceGhostRecipe(_) => {}
            ClientboundGamePacket::PlayerCombatEnd(_) => {}
            ClientboundGamePacket::PlayerCombatEnter(_) => {}
            ClientboundGamePacket::PlayerCombatKill(p) => {
                debug!("Got player kill packet {p:?}");

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
            ClientboundGamePacket::ResourcePackPush(p) => {
                debug!("Got resource pack packet {p:?}");

                let mut system_state: SystemState<EventWriter<ResourcePackEvent>> =
                    SystemState::new(ecs);
                let mut resource_pack_events = system_state.get_mut(ecs);

                resource_pack_events.send(ResourcePackEvent {
                    entity: player_entity,
                    id: p.id,
                    url: p.url.to_owned(),
                    hash: p.hash.to_owned(),
                    required: p.required,
                    prompt: p.prompt.to_owned(),
                });

                system_state.apply(ecs);
            }
            ClientboundGamePacket::ResourcePackPop(_) => {}
            ClientboundGamePacket::Respawn(p) => {
                debug!("Got respawn packet {p:?}");

                #[allow(clippy::type_complexity)]
                let mut system_state: SystemState<(
                    Commands,
                    Query<(
                        &mut InstanceHolder,
                        &GameProfileComponent,
                        &ClientInformation,
                    )>,
                    EventWriter<InstanceLoadedEvent>,
                    ResMut<InstanceContainer>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query, mut instance_loaded_events, mut instance_container) =
                    system_state.get_mut(ecs);
                let (mut instance_holder, game_profile, client_information) =
                    query.get_mut(player_entity).unwrap();

                {
                    let new_instance_name = p.common.dimension.clone();

                    let Some(dimension_type_element) =
                        instance_holder.instance.read().registries.dimension_type()
                    else {
                        error!("Server didn't send dimension type registry, can't log in.");
                        continue;
                    };

                    let dimension_name = ResourceLocation::new(&p.common.dimension.to_string());

                    let Some(dimension) = dimension_type_element.map.get(&dimension_name) else {
                        error!("No dimension_type with name {dimension_name}");
                        continue;
                    };

                    // add this world to the instance_container (or don't if it's already
                    // there)
                    let weak_instance = instance_container.insert(
                        new_instance_name.clone(),
                        dimension.height,
                        dimension.min_y,
                    );
                    instance_loaded_events.send(InstanceLoadedEvent {
                        entity: player_entity,
                        name: new_instance_name.clone(),
                        instance: Arc::downgrade(&weak_instance),
                    });

                    // set the partial_world to an empty world
                    // (when we add chunks or entities those will be in the
                    // instance_container)

                    *instance_holder.partial_instance.write() = PartialInstance::new(
                        azalea_world::chunk_storage::calculate_chunk_storage_range(
                            client_information.view_distance.into(),
                        ),
                        Some(player_entity),
                    );
                    instance_holder.instance = weak_instance;

                    // this resets a bunch of our components like physics and stuff
                    let player_bundle = PlayerBundle {
                        entity: EntityBundle::new(
                            game_profile.uuid,
                            Vec3::default(),
                            azalea_registry::EntityKind::Player,
                            new_instance_name,
                        ),
                        metadata: PlayerMetadataBundle::default(),
                    };
                    // update the local gamemode and metadata things
                    commands.entity(player_entity).insert((
                        LocalGameMode {
                            current: p.common.game_type,
                            previous: p.common.previous_game_type.into(),
                        },
                        player_bundle,
                    ));
                }

                // Remove the Dead marker component from the player.
                commands.entity(player_entity).remove::<Dead>();

                system_state.apply(ecs);
            }

            ClientboundGamePacket::StartConfiguration(_p) => {
                let mut system_state: SystemState<(Commands, EventWriter<SendPacketEvent>)> =
                    SystemState::new(ecs);
                let (mut commands, mut packet_events) = system_state.get_mut(ecs);

                packet_events.send(SendPacketEvent::new(
                    player_entity,
                    ServerboundConfigurationAcknowledged {},
                ));

                commands
                    .entity(player_entity)
                    .insert(crate::client::InConfigurationState)
                    .remove::<crate::JoinedClientBundle>();

                system_state.apply(ecs);
            }

            ClientboundGamePacket::EntityPositionSync(p) => {
                let mut system_state: SystemState<(
                    Commands,
                    Query<(&EntityIdIndex, &InstanceHolder)>,
                )> = SystemState::new(ecs);
                let (mut commands, mut query) = system_state.get_mut(ecs);
                let (entity_id_index, instance_holder) = query.get_mut(player_entity).unwrap();

                let Some(entity) = entity_id_index.get(MinecraftEntityId(p.id)) else {
                    debug!("Got teleport entity packet for unknown entity id {}", p.id);
                    continue;
                };

                let new_position = p.values.pos;
                let new_on_ground = p.on_ground;
                let new_look_direction = p.values.look_direction;

                commands.entity(entity).queue(RelativeEntityUpdate {
                    partial_world: instance_holder.partial_instance.clone(),
                    update: Box::new(move |entity_mut| {
                        let is_local_entity = entity_mut.get::<LocalEntity>().is_some();
                        let mut physics = entity_mut.get_mut::<Physics>().unwrap();

                        physics.vec_delta_codec.set_base(new_position);

                        if is_local_entity {
                            debug!("Ignoring entity position sync packet for local player");
                            return;
                        }

                        physics.set_on_ground(new_on_ground);

                        let mut last_sent_position =
                            entity_mut.get_mut::<LastSentPosition>().unwrap();
                        **last_sent_position = new_position;
                        let mut position = entity_mut.get_mut::<Position>().unwrap();
                        **position = new_position;

                        let mut look_direction = entity_mut.get_mut::<LookDirection>().unwrap();
                        *look_direction = new_look_direction;
                    }),
                });

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
            ClientboundGamePacket::ClearTitles(_) => {}
            ClientboundGamePacket::SoundEntity(_) => {}
            ClientboundGamePacket::StopSound(_) => {}
            ClientboundGamePacket::TabList(_) => {}
            ClientboundGamePacket::TagQuery(_) => {}
            ClientboundGamePacket::TakeItemEntity(_) => {}
            ClientboundGamePacket::BundleDelimiter(_) => {}
            ClientboundGamePacket::DamageEvent(_) => {}
            ClientboundGamePacket::HurtAnimation(_) => {}

            ClientboundGamePacket::TickingState(_) => {}
            ClientboundGamePacket::TickingStep(_) => {}

            ClientboundGamePacket::ResetScore(_) => {}
            ClientboundGamePacket::CookieRequest(_) => {}
            ClientboundGamePacket::DebugSample(_) => {}
            ClientboundGamePacket::PongResponse(_) => {}
            ClientboundGamePacket::StoreCookie(_) => {}
            ClientboundGamePacket::Transfer(_) => {}
            ClientboundGamePacket::MoveMinecartAlongTrack(_) => {}
            ClientboundGamePacket::SetHeldSlot(_) => {}
            ClientboundGamePacket::SetPlayerInventory(_) => {}
            ClientboundGamePacket::ProjectilePower(_) => {}
            ClientboundGamePacket::CustomReportDetails(_) => {}
            ClientboundGamePacket::ServerLinks(_) => {}
            ClientboundGamePacket::PlayerRotation(_) => {}
            ClientboundGamePacket::RecipeBookAdd(_) => {}
            ClientboundGamePacket::RecipeBookRemove(_) => {}
            ClientboundGamePacket::RecipeBookSettings(_) => {}
            ClientboundGamePacket::TestInstanceBlockStatus(_) => {}
        }
    }
}

/// An event for sending a packet to the server while we're in the `game` state.
#[derive(Event)]
pub struct SendPacketEvent {
    pub sent_by: Entity,
    pub packet: ServerboundGamePacket,
}
impl SendPacketEvent {
    pub fn new(sent_by: Entity, packet: impl Packet<ServerboundGamePacket>) -> Self {
        let packet = packet.into_variant();
        Self { sent_by, packet }
    }
}

pub fn handle_send_packet_event(
    mut send_packet_events: EventReader<SendPacketEvent>,
    mut query: Query<&mut RawConnection>,
) {
    for event in send_packet_events.read() {
        if let Ok(raw_connection) = query.get_mut(event.sent_by) {
            // debug!("Sending packet: {:?}", event.packet);
            if let Err(e) = raw_connection.write_packet(event.packet.clone()) {
                error!("Failed to send packet: {e}");
            }
        }
    }
}
