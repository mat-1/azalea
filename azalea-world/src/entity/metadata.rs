#![allow(clippy::single_match)]

// This file is generated from codegen/lib/code/entity.py.
// Don't change it manually!

use super::{
    EntityDataItem, EntityDataValue, OptionalUnsignedInt, Pose, Quaternion, Rotations,
    SnifferState, VillagerData,
};
use azalea_chat::FormattedText;
use azalea_core::{BlockPos, Direction, Particle, Slot, Vec3};
use bevy_ecs::{bundle::Bundle, component::Component};
use derive_more::{Deref, DerefMut};
use thiserror::Error;
use uuid::Uuid;

#[derive(Error, Debug)]
pub enum UpdateMetadataError {
    #[error("Wrong type ({0:?})")]
    WrongType(EntityDataValue),
}
impl From<EntityDataValue> for UpdateMetadataError {
    fn from(value: EntityDataValue) -> Self {
        Self::WrongType(value)
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct OnFire(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ShiftKeyDown(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Sprinting(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Swimming(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CurrentlyGlowing(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Invisible(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct FallFlying(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AirSupply(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct CustomName(pub Option<FormattedText>);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct CustomNameVisible(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Silent(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct NoGravity(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TicksFrozen(pub i32);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct AutoSpinAttack(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct AbstractLivingUsingItem(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Health(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AbstractLivingEffectColor(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct EffectAmbience(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ArrowCount(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct StingerCount(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct SleepingPos(pub Option<BlockPos>);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct NoAi(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct LeftHanded(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Aggressive(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Dancing(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct CanDuplicate(pub bool);
#[derive(Component)]
pub struct Allay;
impl Allay {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(Dancing(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(CanDuplicate(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AllayMetadataBundle {
    _marker: Allay,
    parent: AbstractCreatureMetadataBundle,
    dancing: Dancing,
    can_duplicate: CanDuplicate,
}
impl Default for AllayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Allay,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            dancing: Dancing(false),
            can_duplicate: CanDuplicate(true),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Radius(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AreaEffectCloudColor(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Waiting(pub bool);
#[derive(Component)]
pub struct AreaEffectCloud;
impl AreaEffectCloud {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Radius(d.value.into_float()?));
            }
            9 => {
                entity.insert(AreaEffectCloudColor(d.value.into_int()?));
            }
            10 => {
                entity.insert(Waiting(d.value.into_boolean()?));
            }
            11 => {
                entity.insert(d.value.into_particle()?);
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AreaEffectCloudMetadataBundle {
    _marker: AreaEffectCloud,
    parent: AbstractEntityMetadataBundle,
    radius: Radius,
    area_effect_cloud_color: AreaEffectCloudColor,
    waiting: Waiting,
    particle: Particle,
}
impl Default for AreaEffectCloudMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AreaEffectCloud,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            radius: Radius(3.0),
            area_effect_cloud_color: AreaEffectCloudColor(0),
            waiting: Waiting(false),
            particle: Particle::default(),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Small(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ShowArms(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct NoBasePlate(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ArmorStandMarker(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct HeadPose(pub Rotations);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BodyPose(pub Rotations);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LeftArmPose(pub Rotations);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct RightArmPose(pub Rotations);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LeftLegPose(pub Rotations);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct RightLegPose(pub Rotations);
#[derive(Component)]
pub struct ArmorStand;
impl ArmorStand {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::apply_metadata(entity, d)?,
            15 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Small(bitfield & 0x1 != 0));
                entity.insert(ShowArms(bitfield & 0x4 != 0));
                entity.insert(NoBasePlate(bitfield & 0x8 != 0));
                entity.insert(ArmorStandMarker(bitfield & 0x10 != 0));
            }
            16 => {
                entity.insert(HeadPose(d.value.into_rotations()?));
            }
            17 => {
                entity.insert(BodyPose(d.value.into_rotations()?));
            }
            18 => {
                entity.insert(LeftArmPose(d.value.into_rotations()?));
            }
            19 => {
                entity.insert(RightArmPose(d.value.into_rotations()?));
            }
            20 => {
                entity.insert(LeftLegPose(d.value.into_rotations()?));
            }
            21 => {
                entity.insert(RightLegPose(d.value.into_rotations()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ArmorStandMetadataBundle {
    _marker: ArmorStand,
    parent: AbstractLivingMetadataBundle,
    small: Small,
    show_arms: ShowArms,
    no_base_plate: NoBasePlate,
    armor_stand_marker: ArmorStandMarker,
    head_pose: HeadPose,
    body_pose: BodyPose,
    left_arm_pose: LeftArmPose,
    right_arm_pose: RightArmPose,
    left_leg_pose: LeftLegPose,
    right_leg_pose: RightLegPose,
}
impl Default for ArmorStandMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ArmorStand,
            parent: AbstractLivingMetadataBundle {
                _marker: AbstractLiving,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                auto_spin_attack: AutoSpinAttack(false),
                abstract_living_using_item: AbstractLivingUsingItem(false),
                health: Health(1.0),
                abstract_living_effect_color: AbstractLivingEffectColor(0),
                effect_ambience: EffectAmbience(false),
                arrow_count: ArrowCount(0),
                stinger_count: StingerCount(0),
                sleeping_pos: SleepingPos(None),
            },
            small: Small(false),
            show_arms: ShowArms(false),
            no_base_plate: NoBasePlate(false),
            armor_stand_marker: ArmorStandMarker(false),
            head_pose: HeadPose(Default::default()),
            body_pose: BodyPose(Default::default()),
            left_arm_pose: LeftArmPose(Default::default()),
            right_arm_pose: RightArmPose(Default::default()),
            left_leg_pose: LeftLegPose(Default::default()),
            right_leg_pose: RightLegPose(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ArrowCritArrow(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ArrowShotFromCrossbow(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ArrowNoPhysics(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ArrowPierceLevel(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ArrowEffectColor(pub i32);
#[derive(Component)]
pub struct Arrow;
impl Arrow {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(ArrowCritArrow(bitfield & 0x1 != 0));
                entity.insert(ArrowShotFromCrossbow(bitfield & 0x4 != 0));
                entity.insert(ArrowNoPhysics(bitfield & 0x2 != 0));
            }
            9 => {
                entity.insert(ArrowPierceLevel(d.value.into_byte()?));
            }
            10 => {
                entity.insert(ArrowEffectColor(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ArrowMetadataBundle {
    _marker: Arrow,
    parent: AbstractEntityMetadataBundle,
    arrow_crit_arrow: ArrowCritArrow,
    arrow_shot_from_crossbow: ArrowShotFromCrossbow,
    arrow_no_physics: ArrowNoPhysics,
    arrow_pierce_level: ArrowPierceLevel,
    arrow_effect_color: ArrowEffectColor,
}
impl Default for ArrowMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Arrow,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            arrow_crit_arrow: ArrowCritArrow(false),
            arrow_shot_from_crossbow: ArrowShotFromCrossbow(false),
            arrow_no_physics: ArrowNoPhysics(false),
            arrow_pierce_level: ArrowPierceLevel(0),
            arrow_effect_color: ArrowEffectColor(-1),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AbstractAgeableBaby(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AxolotlVariant(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PlayingDead(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AxolotlFromBucket(pub bool);
#[derive(Component)]
pub struct Axolotl;
impl Axolotl {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(AxolotlVariant(d.value.into_int()?));
            }
            18 => {
                entity.insert(PlayingDead(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(AxolotlFromBucket(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AxolotlMetadataBundle {
    _marker: Axolotl,
    parent: AbstractAnimalMetadataBundle,
    axolotl_variant: AxolotlVariant,
    playing_dead: PlayingDead,
    axolotl_from_bucket: AxolotlFromBucket,
}
impl Default for AxolotlMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Axolotl,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            axolotl_variant: AxolotlVariant(0),
            playing_dead: PlayingDead(false),
            axolotl_from_bucket: AxolotlFromBucket(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Resting(pub bool);
#[derive(Component)]
pub struct Bat;
impl Bat {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Resting(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct BatMetadataBundle {
    _marker: Bat,
    parent: AbstractInsentientMetadataBundle,
    resting: Resting,
}
impl Default for BatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Bat,
            parent: AbstractInsentientMetadataBundle {
                _marker: AbstractInsentient,
                parent: AbstractLivingMetadataBundle {
                    _marker: AbstractLiving,
                    parent: AbstractEntityMetadataBundle {
                        _marker: AbstractEntity,
                        on_fire: OnFire(false),
                        shift_key_down: ShiftKeyDown(false),
                        sprinting: Sprinting(false),
                        swimming: Swimming(false),
                        currently_glowing: CurrentlyGlowing(false),
                        invisible: Invisible(false),
                        fall_flying: FallFlying(false),
                        air_supply: AirSupply(Default::default()),
                        custom_name: CustomName(None),
                        custom_name_visible: CustomNameVisible(false),
                        silent: Silent(false),
                        no_gravity: NoGravity(false),
                        pose: Pose::default(),
                        ticks_frozen: TicksFrozen(0),
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            resting: Resting(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HasNectar(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HasStung(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct BeeRolling(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BeeRemainingAngerTime(pub i32);
#[derive(Component)]
pub struct Bee;
impl Bee {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HasNectar(bitfield & 0x8 != 0));
                entity.insert(HasStung(bitfield & 0x4 != 0));
                entity.insert(BeeRolling(bitfield & 0x2 != 0));
            }
            18 => {
                entity.insert(BeeRemainingAngerTime(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct BeeMetadataBundle {
    _marker: Bee,
    parent: AbstractAnimalMetadataBundle,
    has_nectar: HasNectar,
    has_stung: HasStung,
    bee_rolling: BeeRolling,
    bee_remaining_anger_time: BeeRemainingAngerTime,
}
impl Default for BeeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Bee,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            has_nectar: HasNectar(false),
            has_stung: HasStung(false),
            bee_rolling: BeeRolling(false),
            bee_remaining_anger_time: BeeRemainingAngerTime(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Charged(pub bool);
#[derive(Component)]
pub struct Blaze;
impl Blaze {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Charged(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct BlazeMetadataBundle {
    _marker: Blaze,
    parent: AbstractMonsterMetadataBundle,
    charged: Charged,
}
impl Default for BlazeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Blaze,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            charged: Charged(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayInterpolationStartDeltaTicks(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayInterpolationDuration(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayTranslation(pub Vec3);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayScale(pub Vec3);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayLeftRotation(pub Quaternion);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayRightRotation(pub Quaternion);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayBillboardRenderConstraints(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayBrightnessOverride(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayViewRange(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayShadowRadius(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayShadowStrength(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayWidth(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayHeight(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockDisplayGlowColorOverride(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BlockState(pub azalea_block::BlockState);
#[derive(Component)]
pub struct BlockDisplay;
impl BlockDisplay {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(BlockDisplayInterpolationStartDeltaTicks(
                    d.value.into_int()?,
                ));
            }
            9 => {
                entity.insert(BlockDisplayInterpolationDuration(d.value.into_int()?));
            }
            10 => {
                entity.insert(BlockDisplayTranslation(d.value.into_vector3()?));
            }
            11 => {
                entity.insert(BlockDisplayScale(d.value.into_vector3()?));
            }
            12 => {
                entity.insert(BlockDisplayLeftRotation(d.value.into_quaternion()?));
            }
            13 => {
                entity.insert(BlockDisplayRightRotation(d.value.into_quaternion()?));
            }
            14 => {
                entity.insert(BlockDisplayBillboardRenderConstraints(d.value.into_byte()?));
            }
            15 => {
                entity.insert(BlockDisplayBrightnessOverride(d.value.into_int()?));
            }
            16 => {
                entity.insert(BlockDisplayViewRange(d.value.into_float()?));
            }
            17 => {
                entity.insert(BlockDisplayShadowRadius(d.value.into_float()?));
            }
            18 => {
                entity.insert(BlockDisplayShadowStrength(d.value.into_float()?));
            }
            19 => {
                entity.insert(BlockDisplayWidth(d.value.into_float()?));
            }
            20 => {
                entity.insert(BlockDisplayHeight(d.value.into_float()?));
            }
            21 => {
                entity.insert(BlockDisplayGlowColorOverride(d.value.into_int()?));
            }
            22 => {
                entity.insert(BlockState(d.value.into_block_state()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct BlockDisplayMetadataBundle {
    _marker: BlockDisplay,
    parent: AbstractEntityMetadataBundle,
    block_display_interpolation_start_delta_ticks: BlockDisplayInterpolationStartDeltaTicks,
    block_display_interpolation_duration: BlockDisplayInterpolationDuration,
    block_display_translation: BlockDisplayTranslation,
    block_display_scale: BlockDisplayScale,
    block_display_left_rotation: BlockDisplayLeftRotation,
    block_display_right_rotation: BlockDisplayRightRotation,
    block_display_billboard_render_constraints: BlockDisplayBillboardRenderConstraints,
    block_display_brightness_override: BlockDisplayBrightnessOverride,
    block_display_view_range: BlockDisplayViewRange,
    block_display_shadow_radius: BlockDisplayShadowRadius,
    block_display_shadow_strength: BlockDisplayShadowStrength,
    block_display_width: BlockDisplayWidth,
    block_display_height: BlockDisplayHeight,
    block_display_glow_color_override: BlockDisplayGlowColorOverride,
    block_state: BlockState,
}
impl Default for BlockDisplayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: BlockDisplay,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            block_display_interpolation_start_delta_ticks: BlockDisplayInterpolationStartDeltaTicks(
                0,
            ),
            block_display_interpolation_duration: BlockDisplayInterpolationDuration(0),
            block_display_translation: BlockDisplayTranslation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            block_display_scale: BlockDisplayScale(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            block_display_left_rotation: BlockDisplayLeftRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            block_display_right_rotation: BlockDisplayRightRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            block_display_billboard_render_constraints: BlockDisplayBillboardRenderConstraints(
                Default::default(),
            ),
            block_display_brightness_override: BlockDisplayBrightnessOverride(-1),
            block_display_view_range: BlockDisplayViewRange(1.0),
            block_display_shadow_radius: BlockDisplayShadowRadius(0.0),
            block_display_shadow_strength: BlockDisplayShadowStrength(1.0),
            block_display_width: BlockDisplayWidth(0.0),
            block_display_height: BlockDisplayHeight(0.0),
            block_display_glow_color_override: BlockDisplayGlowColorOverride(-1),
            block_state: BlockState(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct BoatHurt(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BoatHurtdir(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BoatDamage(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BoatKind(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PaddleLeft(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PaddleRight(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BubbleTime(pub i32);
#[derive(Component)]
pub struct Boat;
impl Boat {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(BoatHurt(d.value.into_int()?));
            }
            9 => {
                entity.insert(BoatHurtdir(d.value.into_int()?));
            }
            10 => {
                entity.insert(BoatDamage(d.value.into_float()?));
            }
            11 => {
                entity.insert(BoatKind(d.value.into_int()?));
            }
            12 => {
                entity.insert(PaddleLeft(d.value.into_boolean()?));
            }
            13 => {
                entity.insert(PaddleRight(d.value.into_boolean()?));
            }
            14 => {
                entity.insert(BubbleTime(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct BoatMetadataBundle {
    _marker: Boat,
    parent: AbstractEntityMetadataBundle,
    boat_hurt: BoatHurt,
    boat_hurtdir: BoatHurtdir,
    boat_damage: BoatDamage,
    boat_kind: BoatKind,
    paddle_left: PaddleLeft,
    paddle_right: PaddleRight,
    bubble_time: BubbleTime,
}
impl Default for BoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Boat,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            boat_hurt: BoatHurt(0),
            boat_hurtdir: BoatHurtdir(1),
            boat_damage: BoatDamage(0.0),
            boat_kind: BoatKind(Default::default()),
            paddle_left: PaddleLeft(false),
            paddle_right: PaddleRight(false),
            bubble_time: BubbleTime(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CamelTamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CamelEating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CamelStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CamelBred(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct CamelSaddled(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Dash(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LastPoseChangeTick(pub i64);
#[derive(Component)]
pub struct Camel;
impl Camel {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(CamelTamed(bitfield & 0x2 != 0));
                entity.insert(CamelEating(bitfield & 0x10 != 0));
                entity.insert(CamelStanding(bitfield & 0x20 != 0));
                entity.insert(CamelBred(bitfield & 0x8 != 0));
                entity.insert(CamelSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(Dash(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(LastPoseChangeTick(d.value.into_long()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct CamelMetadataBundle {
    _marker: Camel,
    parent: AbstractAnimalMetadataBundle,
    camel_tamed: CamelTamed,
    camel_eating: CamelEating,
    camel_standing: CamelStanding,
    camel_bred: CamelBred,
    camel_saddled: CamelSaddled,
    dash: Dash,
    last_pose_change_tick: LastPoseChangeTick,
}
impl Default for CamelMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Camel,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            camel_tamed: CamelTamed(false),
            camel_eating: CamelEating(false),
            camel_standing: CamelStanding(false),
            camel_bred: CamelBred(false),
            camel_saddled: CamelSaddled(false),
            dash: Dash(false),
            last_pose_change_tick: LastPoseChangeTick(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Tame(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct InSittingPose(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Owneruuid(pub Option<Uuid>);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct CatVariant(pub azalea_registry::CatVariant);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct IsLying(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct RelaxStateOne(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct CatCollarColor(pub i32);
#[derive(Component)]
pub struct Cat;
impl Cat {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
            19 => {
                entity.insert(CatVariant(d.value.into_cat_variant()?));
            }
            20 => {
                entity.insert(IsLying(d.value.into_boolean()?));
            }
            21 => {
                entity.insert(RelaxStateOne(d.value.into_boolean()?));
            }
            22 => {
                entity.insert(CatCollarColor(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct CatMetadataBundle {
    _marker: Cat,
    parent: AbstractTameableMetadataBundle,
    cat_variant: CatVariant,
    is_lying: IsLying,
    relax_state_one: RelaxStateOne,
    cat_collar_color: CatCollarColor,
}
impl Default for CatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Cat,
            parent: AbstractTameableMetadataBundle {
                _marker: AbstractTameable,
                parent: AbstractAnimalMetadataBundle {
                    _marker: AbstractAnimal,
                    parent: AbstractAgeableMetadataBundle {
                        _marker: AbstractAgeable,
                        parent: AbstractCreatureMetadataBundle {
                            _marker: AbstractCreature,
                            parent: AbstractInsentientMetadataBundle {
                                _marker: AbstractInsentient,
                                parent: AbstractLivingMetadataBundle {
                                    _marker: AbstractLiving,
                                    parent: AbstractEntityMetadataBundle {
                                        _marker: AbstractEntity,
                                        on_fire: OnFire(false),
                                        shift_key_down: ShiftKeyDown(false),
                                        sprinting: Sprinting(false),
                                        swimming: Swimming(false),
                                        currently_glowing: CurrentlyGlowing(false),
                                        invisible: Invisible(false),
                                        fall_flying: FallFlying(false),
                                        air_supply: AirSupply(Default::default()),
                                        custom_name: CustomName(None),
                                        custom_name_visible: CustomNameVisible(false),
                                        silent: Silent(false),
                                        no_gravity: NoGravity(false),
                                        pose: Pose::default(),
                                        ticks_frozen: TicksFrozen(0),
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
                tame: Tame(false),
                in_sitting_pose: InSittingPose(false),
                owneruuid: Owneruuid(None),
            },
            cat_variant: CatVariant(azalea_registry::CatVariant::Tabby),
            is_lying: IsLying(false),
            relax_state_one: RelaxStateOne(false),
            cat_collar_color: CatCollarColor(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Climbing(pub bool);
#[derive(Component)]
pub struct CaveSpider;
impl CaveSpider {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Spider::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct CaveSpiderMetadataBundle {
    _marker: CaveSpider,
    parent: SpiderMetadataBundle,
}
impl Default for CaveSpiderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CaveSpider,
            parent: SpiderMetadataBundle {
                _marker: Spider,
                parent: AbstractMonsterMetadataBundle {
                    _marker: AbstractMonster,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                climbing: Climbing(false),
            },
        }
    }
}

#[derive(Component)]
pub struct ChestBoat;
impl ChestBoat {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => Boat::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ChestBoatMetadataBundle {
    _marker: ChestBoat,
    parent: BoatMetadataBundle,
}
impl Default for ChestBoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ChestBoat,
            parent: BoatMetadataBundle {
                _marker: Boat,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                boat_hurt: BoatHurt(0),
                boat_hurtdir: BoatHurtdir(1),
                boat_damage: BoatDamage(0.0),
                boat_kind: BoatKind(Default::default()),
                paddle_left: PaddleLeft(false),
                paddle_right: PaddleRight(false),
                bubble_time: BubbleTime(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AbstractMinecartHurt(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AbstractMinecartHurtdir(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AbstractMinecartDamage(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct DisplayBlock(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct DisplayOffset(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct CustomDisplay(pub bool);
#[derive(Component)]
pub struct ChestMinecart;
impl ChestMinecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ChestMinecartMetadataBundle {
    _marker: ChestMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for ChestMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ChestMinecart,
            parent: AbstractMinecartMetadataBundle {
                _marker: AbstractMinecart,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component)]
pub struct Chicken;
impl Chicken {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ChickenMetadataBundle {
    _marker: Chicken,
    parent: AbstractAnimalMetadataBundle,
}
impl Default for ChickenMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Chicken,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct CodFromBucket(pub bool);
#[derive(Component)]
pub struct Cod;
impl Cod {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(CodFromBucket(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct CodMetadataBundle {
    _marker: Cod,
    parent: AbstractCreatureMetadataBundle,
    cod_from_bucket: CodFromBucket,
}
impl Default for CodMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Cod,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            cod_from_bucket: CodFromBucket(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct CommandName(pub String);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LastOutput(pub FormattedText);
#[derive(Component)]
pub struct CommandBlockMinecart;
impl CommandBlockMinecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::apply_metadata(entity, d)?,
            14 => {
                entity.insert(CommandName(d.value.into_string()?));
            }
            15 => {
                entity.insert(LastOutput(d.value.into_formatted_text()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct CommandBlockMinecartMetadataBundle {
    _marker: CommandBlockMinecart,
    parent: AbstractMinecartMetadataBundle,
    command_name: CommandName,
    last_output: LastOutput,
}
impl Default for CommandBlockMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: CommandBlockMinecart,
            parent: AbstractMinecartMetadataBundle {
                _marker: AbstractMinecart,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
            command_name: CommandName("".to_string()),
            last_output: LastOutput(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct Cow;
impl Cow {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct CowMetadataBundle {
    _marker: Cow,
    parent: AbstractAnimalMetadataBundle,
}
impl Default for CowMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Cow,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct SwellDir(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct IsPowered(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct IsIgnited(pub bool);
#[derive(Component)]
pub struct Creeper;
impl Creeper {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(SwellDir(d.value.into_int()?));
            }
            17 => {
                entity.insert(IsPowered(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(IsIgnited(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct CreeperMetadataBundle {
    _marker: Creeper,
    parent: AbstractMonsterMetadataBundle,
    swell_dir: SwellDir,
    is_powered: IsPowered,
    is_ignited: IsIgnited,
}
impl Default for CreeperMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Creeper,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            swell_dir: SwellDir(-1),
            is_powered: IsPowered(false),
            is_ignited: IsIgnited(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct TreasurePos(pub BlockPos);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct GotFish(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct MoistnessLevel(pub i32);
#[derive(Component)]
pub struct Dolphin;
impl Dolphin {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(TreasurePos(d.value.into_block_pos()?));
            }
            17 => {
                entity.insert(GotFish(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(MoistnessLevel(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct DolphinMetadataBundle {
    _marker: Dolphin,
    parent: AbstractCreatureMetadataBundle,
    treasure_pos: TreasurePos,
    got_fish: GotFish,
    moistness_level: MoistnessLevel,
}
impl Default for DolphinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Dolphin,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            treasure_pos: TreasurePos(BlockPos::new(0, 0, 0)),
            got_fish: GotFish(false),
            moistness_level: MoistnessLevel(2400),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct DonkeyTamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct DonkeyEating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct DonkeyStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct DonkeyBred(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct DonkeySaddled(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct DonkeyChest(pub bool);
#[derive(Component)]
pub struct Donkey;
impl Donkey {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(DonkeyTamed(bitfield & 0x2 != 0));
                entity.insert(DonkeyEating(bitfield & 0x10 != 0));
                entity.insert(DonkeyStanding(bitfield & 0x20 != 0));
                entity.insert(DonkeyBred(bitfield & 0x8 != 0));
                entity.insert(DonkeySaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(DonkeyChest(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct DonkeyMetadataBundle {
    _marker: Donkey,
    parent: AbstractAnimalMetadataBundle,
    donkey_tamed: DonkeyTamed,
    donkey_eating: DonkeyEating,
    donkey_standing: DonkeyStanding,
    donkey_bred: DonkeyBred,
    donkey_saddled: DonkeySaddled,
    donkey_chest: DonkeyChest,
}
impl Default for DonkeyMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Donkey,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            donkey_tamed: DonkeyTamed(false),
            donkey_eating: DonkeyEating(false),
            donkey_standing: DonkeyStanding(false),
            donkey_bred: DonkeyBred(false),
            donkey_saddled: DonkeySaddled(false),
            donkey_chest: DonkeyChest(false),
        }
    }
}

#[derive(Component)]
pub struct DragonFireball;
impl DragonFireball {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct DragonFireballMetadataBundle {
    _marker: DragonFireball,
    parent: AbstractEntityMetadataBundle,
}
impl Default for DragonFireballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: DragonFireball,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ZombieBaby(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct SpecialType(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct DrownedConversion(pub bool);
#[derive(Component)]
pub struct Drowned;
impl Drowned {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct DrownedMetadataBundle {
    _marker: Drowned,
    parent: ZombieMetadataBundle,
}
impl Default for DrownedMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Drowned,
            parent: ZombieMetadataBundle {
                _marker: Zombie,
                parent: AbstractMonsterMetadataBundle {
                    _marker: AbstractMonster,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct EggItemStack(pub Slot);
#[derive(Component)]
pub struct Egg;
impl Egg {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(EggItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EggMetadataBundle {
    _marker: Egg,
    parent: AbstractEntityMetadataBundle,
    egg_item_stack: EggItemStack,
}
impl Default for EggMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Egg,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            egg_item_stack: EggItemStack(Slot::Empty),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Moving(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AttackTarget(pub i32);
#[derive(Component)]
pub struct ElderGuardian;
impl ElderGuardian {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=17 => Guardian::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ElderGuardianMetadataBundle {
    _marker: ElderGuardian,
    parent: GuardianMetadataBundle,
}
impl Default for ElderGuardianMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ElderGuardian,
            parent: GuardianMetadataBundle {
                _marker: Guardian,
                parent: AbstractMonsterMetadataBundle {
                    _marker: AbstractMonster,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                moving: Moving(false),
                attack_target: AttackTarget(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct BeamTarget(pub Option<BlockPos>);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ShowBottom(pub bool);
#[derive(Component)]
pub struct EndCrystal;
impl EndCrystal {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(BeamTarget(d.value.into_optional_block_pos()?));
            }
            9 => {
                entity.insert(ShowBottom(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EndCrystalMetadataBundle {
    _marker: EndCrystal,
    parent: AbstractEntityMetadataBundle,
    beam_target: BeamTarget,
    show_bottom: ShowBottom,
}
impl Default for EndCrystalMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EndCrystal,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            beam_target: BeamTarget(None),
            show_bottom: ShowBottom(true),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Phase(pub i32);
#[derive(Component)]
pub struct EnderDragon;
impl EnderDragon {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(Phase(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EnderDragonMetadataBundle {
    _marker: EnderDragon,
    parent: AbstractInsentientMetadataBundle,
    phase: Phase,
}
impl Default for EnderDragonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EnderDragon,
            parent: AbstractInsentientMetadataBundle {
                _marker: AbstractInsentient,
                parent: AbstractLivingMetadataBundle {
                    _marker: AbstractLiving,
                    parent: AbstractEntityMetadataBundle {
                        _marker: AbstractEntity,
                        on_fire: OnFire(false),
                        shift_key_down: ShiftKeyDown(false),
                        sprinting: Sprinting(false),
                        swimming: Swimming(false),
                        currently_glowing: CurrentlyGlowing(false),
                        invisible: Invisible(false),
                        fall_flying: FallFlying(false),
                        air_supply: AirSupply(Default::default()),
                        custom_name: CustomName(None),
                        custom_name_visible: CustomNameVisible(false),
                        silent: Silent(false),
                        no_gravity: NoGravity(false),
                        pose: Pose::default(),
                        ticks_frozen: TicksFrozen(0),
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            phase: Phase(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct EnderPearlItemStack(pub Slot);
#[derive(Component)]
pub struct EnderPearl;
impl EnderPearl {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(EnderPearlItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EnderPearlMetadataBundle {
    _marker: EnderPearl,
    parent: AbstractEntityMetadataBundle,
    ender_pearl_item_stack: EnderPearlItemStack,
}
impl Default for EnderPearlMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EnderPearl,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            ender_pearl_item_stack: EnderPearlItemStack(Slot::Empty),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct CarryState(pub azalea_block::BlockState);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Creepy(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct StaredAt(pub bool);
#[derive(Component)]
pub struct Enderman;
impl Enderman {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(CarryState(d.value.into_optional_block_state()?));
            }
            17 => {
                entity.insert(Creepy(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(StaredAt(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EndermanMetadataBundle {
    _marker: Enderman,
    parent: AbstractMonsterMetadataBundle,
    carry_state: CarryState,
    creepy: Creepy,
    stared_at: StaredAt,
}
impl Default for EndermanMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Enderman,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            carry_state: CarryState(azalea_block::BlockState::AIR),
            creepy: Creepy(false),
            stared_at: StaredAt(false),
        }
    }
}

#[derive(Component)]
pub struct Endermite;
impl Endermite {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EndermiteMetadataBundle {
    _marker: Endermite,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for EndermiteMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Endermite,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct EvokerIsCelebrating(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct EvokerSpellCasting(pub u8);
#[derive(Component)]
pub struct Evoker;
impl Evoker {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(EvokerIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(EvokerSpellCasting(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EvokerMetadataBundle {
    _marker: Evoker,
    parent: AbstractMonsterMetadataBundle,
    evoker_is_celebrating: EvokerIsCelebrating,
    evoker_spell_casting: EvokerSpellCasting,
}
impl Default for EvokerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Evoker,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            evoker_is_celebrating: EvokerIsCelebrating(false),
            evoker_spell_casting: EvokerSpellCasting(0),
        }
    }
}

#[derive(Component)]
pub struct EvokerFangs;
impl EvokerFangs {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EvokerFangsMetadataBundle {
    _marker: EvokerFangs,
    parent: AbstractEntityMetadataBundle,
}
impl Default for EvokerFangsMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EvokerFangs,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ExperienceBottleItemStack(pub Slot);
#[derive(Component)]
pub struct ExperienceBottle;
impl ExperienceBottle {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(ExperienceBottleItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ExperienceBottleMetadataBundle {
    _marker: ExperienceBottle,
    parent: AbstractEntityMetadataBundle,
    experience_bottle_item_stack: ExperienceBottleItemStack,
}
impl Default for ExperienceBottleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ExperienceBottle,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            experience_bottle_item_stack: ExperienceBottleItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct ExperienceOrb;
impl ExperienceOrb {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ExperienceOrbMetadataBundle {
    _marker: ExperienceOrb,
    parent: AbstractEntityMetadataBundle,
}
impl Default for ExperienceOrbMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ExperienceOrb,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct EyeOfEnderItemStack(pub Slot);
#[derive(Component)]
pub struct EyeOfEnder;
impl EyeOfEnder {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(EyeOfEnderItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct EyeOfEnderMetadataBundle {
    _marker: EyeOfEnder,
    parent: AbstractEntityMetadataBundle,
    eye_of_ender_item_stack: EyeOfEnderItemStack,
}
impl Default for EyeOfEnderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: EyeOfEnder,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            eye_of_ender_item_stack: EyeOfEnderItemStack(Slot::Empty),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct StartPos(pub BlockPos);
#[derive(Component)]
pub struct FallingBlock;
impl FallingBlock {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(StartPos(d.value.into_block_pos()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct FallingBlockMetadataBundle {
    _marker: FallingBlock,
    parent: AbstractEntityMetadataBundle,
    start_pos: StartPos,
}
impl Default for FallingBlockMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FallingBlock,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            start_pos: StartPos(BlockPos::new(0, 0, 0)),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct FireballItemStack(pub Slot);
#[derive(Component)]
pub struct Fireball;
impl Fireball {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(FireballItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct FireballMetadataBundle {
    _marker: Fireball,
    parent: AbstractEntityMetadataBundle,
    fireball_item_stack: FireballItemStack,
}
impl Default for FireballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Fireball,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            fireball_item_stack: FireballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct FireworksItem(pub Slot);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct AttachedToTarget(pub OptionalUnsignedInt);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ShotAtAngle(pub bool);
#[derive(Component)]
pub struct FireworkRocket;
impl FireworkRocket {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(FireworksItem(d.value.into_item_stack()?));
            }
            9 => {
                entity.insert(AttachedToTarget(d.value.into_optional_unsigned_int()?));
            }
            10 => {
                entity.insert(ShotAtAngle(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct FireworkRocketMetadataBundle {
    _marker: FireworkRocket,
    parent: AbstractEntityMetadataBundle,
    fireworks_item: FireworksItem,
    attached_to_target: AttachedToTarget,
    shot_at_angle: ShotAtAngle,
}
impl Default for FireworkRocketMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FireworkRocket,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            fireworks_item: FireworksItem(Slot::Empty),
            attached_to_target: AttachedToTarget(OptionalUnsignedInt(None)),
            shot_at_angle: ShotAtAngle(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct HookedEntity(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Biting(pub bool);
#[derive(Component)]
pub struct FishingBobber;
impl FishingBobber {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(HookedEntity(d.value.into_int()?));
            }
            9 => {
                entity.insert(Biting(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct FishingBobberMetadataBundle {
    _marker: FishingBobber,
    parent: AbstractEntityMetadataBundle,
    hooked_entity: HookedEntity,
    biting: Biting,
}
impl Default for FishingBobberMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FishingBobber,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            hooked_entity: HookedEntity(0),
            biting: Biting(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct FoxKind(pub i32);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct FoxSitting(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Faceplanted(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Sleeping(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Pouncing(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Crouching(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct FoxInterested(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TrustedId0(pub Option<Uuid>);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TrustedId1(pub Option<Uuid>);
#[derive(Component)]
pub struct Fox;
impl Fox {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(FoxKind(d.value.into_int()?));
            }
            18 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(FoxSitting(bitfield & 0x1 != 0));
                entity.insert(Faceplanted(bitfield & 0x40 != 0));
                entity.insert(Sleeping(bitfield & 0x20 != 0));
                entity.insert(Pouncing(bitfield & 0x10 != 0));
                entity.insert(Crouching(bitfield & 0x4 != 0));
                entity.insert(FoxInterested(bitfield & 0x8 != 0));
            }
            19 => {
                entity.insert(TrustedId0(d.value.into_optional_uuid()?));
            }
            20 => {
                entity.insert(TrustedId1(d.value.into_optional_uuid()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct FoxMetadataBundle {
    _marker: Fox,
    parent: AbstractAnimalMetadataBundle,
    fox_kind: FoxKind,
    fox_sitting: FoxSitting,
    faceplanted: Faceplanted,
    sleeping: Sleeping,
    pouncing: Pouncing,
    crouching: Crouching,
    fox_interested: FoxInterested,
    trusted_id_0: TrustedId0,
    trusted_id_1: TrustedId1,
}
impl Default for FoxMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Fox,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            fox_kind: FoxKind(0),
            fox_sitting: FoxSitting(false),
            faceplanted: Faceplanted(false),
            sleeping: Sleeping(false),
            pouncing: Pouncing(false),
            crouching: Crouching(false),
            fox_interested: FoxInterested(false),
            trusted_id_0: TrustedId0(None),
            trusted_id_1: TrustedId1(None),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct FrogVariant(pub azalea_registry::FrogVariant);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TongueTarget(pub OptionalUnsignedInt);
#[derive(Component)]
pub struct Frog;
impl Frog {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(FrogVariant(d.value.into_frog_variant()?));
            }
            18 => {
                entity.insert(TongueTarget(d.value.into_optional_unsigned_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct FrogMetadataBundle {
    _marker: Frog,
    parent: AbstractAnimalMetadataBundle,
    frog_variant: FrogVariant,
    tongue_target: TongueTarget,
}
impl Default for FrogMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Frog,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            frog_variant: FrogVariant(azalea_registry::FrogVariant::Temperate),
            tongue_target: TongueTarget(OptionalUnsignedInt(None)),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Fuel(pub bool);
#[derive(Component)]
pub struct FurnaceMinecart;
impl FurnaceMinecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::apply_metadata(entity, d)?,
            14 => {
                entity.insert(Fuel(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct FurnaceMinecartMetadataBundle {
    _marker: FurnaceMinecart,
    parent: AbstractMinecartMetadataBundle,
    fuel: Fuel,
}
impl Default for FurnaceMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: FurnaceMinecart,
            parent: AbstractMinecartMetadataBundle {
                _marker: AbstractMinecart,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
            fuel: Fuel(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct IsCharging(pub bool);
#[derive(Component)]
pub struct Ghast;
impl Ghast {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(IsCharging(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct GhastMetadataBundle {
    _marker: Ghast,
    parent: AbstractInsentientMetadataBundle,
    is_charging: IsCharging,
}
impl Default for GhastMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Ghast,
            parent: AbstractInsentientMetadataBundle {
                _marker: AbstractInsentient,
                parent: AbstractLivingMetadataBundle {
                    _marker: AbstractLiving,
                    parent: AbstractEntityMetadataBundle {
                        _marker: AbstractEntity,
                        on_fire: OnFire(false),
                        shift_key_down: ShiftKeyDown(false),
                        sprinting: Sprinting(false),
                        swimming: Swimming(false),
                        currently_glowing: CurrentlyGlowing(false),
                        invisible: Invisible(false),
                        fall_flying: FallFlying(false),
                        air_supply: AirSupply(Default::default()),
                        custom_name: CustomName(None),
                        custom_name_visible: CustomNameVisible(false),
                        silent: Silent(false),
                        no_gravity: NoGravity(false),
                        pose: Pose::default(),
                        ticks_frozen: TicksFrozen(0),
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            is_charging: IsCharging(false),
        }
    }
}

#[derive(Component)]
pub struct Giant;
impl Giant {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct GiantMetadataBundle {
    _marker: Giant,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for GiantMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Giant,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemFrameItem(pub Slot);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Rotation(pub i32);
#[derive(Component)]
pub struct GlowItemFrame;
impl GlowItemFrame {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=9 => ItemFrame::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct GlowItemFrameMetadataBundle {
    _marker: GlowItemFrame,
    parent: ItemFrameMetadataBundle,
}
impl Default for GlowItemFrameMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: GlowItemFrame,
            parent: ItemFrameMetadataBundle {
                _marker: ItemFrame,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                item_frame_item: ItemFrameItem(Slot::Empty),
                rotation: Rotation(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct DarkTicksRemaining(pub i32);
#[derive(Component)]
pub struct GlowSquid;
impl GlowSquid {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => Squid::apply_metadata(entity, d)?,
            16 => {
                entity.insert(DarkTicksRemaining(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct GlowSquidMetadataBundle {
    _marker: GlowSquid,
    parent: SquidMetadataBundle,
    dark_ticks_remaining: DarkTicksRemaining,
}
impl Default for GlowSquidMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: GlowSquid,
            parent: SquidMetadataBundle {
                _marker: Squid,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            dark_ticks_remaining: DarkTicksRemaining(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct IsScreamingGoat(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct HasLeftHorn(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct HasRightHorn(pub bool);
#[derive(Component)]
pub struct Goat;
impl Goat {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(IsScreamingGoat(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(HasLeftHorn(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(HasRightHorn(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct GoatMetadataBundle {
    _marker: Goat,
    parent: AbstractAnimalMetadataBundle,
    is_screaming_goat: IsScreamingGoat,
    has_left_horn: HasLeftHorn,
    has_right_horn: HasRightHorn,
}
impl Default for GoatMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Goat,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            is_screaming_goat: IsScreamingGoat(false),
            has_left_horn: HasLeftHorn(true),
            has_right_horn: HasRightHorn(true),
        }
    }
}

#[derive(Component)]
pub struct Guardian;
impl Guardian {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(Moving(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(AttackTarget(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct GuardianMetadataBundle {
    _marker: Guardian,
    parent: AbstractMonsterMetadataBundle,
    moving: Moving,
    attack_target: AttackTarget,
}
impl Default for GuardianMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Guardian,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            moving: Moving(false),
            attack_target: AttackTarget(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct HoglinImmuneToZombification(pub bool);
#[derive(Component)]
pub struct Hoglin;
impl Hoglin {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(HoglinImmuneToZombification(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct HoglinMetadataBundle {
    _marker: Hoglin,
    parent: AbstractAnimalMetadataBundle,
    hoglin_immune_to_zombification: HoglinImmuneToZombification,
}
impl Default for HoglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Hoglin,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            hoglin_immune_to_zombification: HoglinImmuneToZombification(false),
        }
    }
}

#[derive(Component)]
pub struct HopperMinecart;
impl HopperMinecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct HopperMinecartMetadataBundle {
    _marker: HopperMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for HopperMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: HopperMinecart,
            parent: AbstractMinecartMetadataBundle {
                _marker: AbstractMinecart,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HorseTamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HorseEating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HorseStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HorseBred(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HorseSaddled(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct HorseTypeVariant(pub i32);
#[derive(Component)]
pub struct Horse;
impl Horse {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HorseTamed(bitfield & 0x2 != 0));
                entity.insert(HorseEating(bitfield & 0x10 != 0));
                entity.insert(HorseStanding(bitfield & 0x20 != 0));
                entity.insert(HorseBred(bitfield & 0x8 != 0));
                entity.insert(HorseSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(HorseTypeVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct HorseMetadataBundle {
    _marker: Horse,
    parent: AbstractAnimalMetadataBundle,
    horse_tamed: HorseTamed,
    horse_eating: HorseEating,
    horse_standing: HorseStanding,
    horse_bred: HorseBred,
    horse_saddled: HorseSaddled,
    horse_type_variant: HorseTypeVariant,
}
impl Default for HorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Horse,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            horse_tamed: HorseTamed(false),
            horse_eating: HorseEating(false),
            horse_standing: HorseStanding(false),
            horse_bred: HorseBred(false),
            horse_saddled: HorseSaddled(false),
            horse_type_variant: HorseTypeVariant(0),
        }
    }
}

#[derive(Component)]
pub struct Husk;
impl Husk {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct HuskMetadataBundle {
    _marker: Husk,
    parent: ZombieMetadataBundle,
}
impl Default for HuskMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Husk,
            parent: ZombieMetadataBundle {
                _marker: Zombie,
                parent: AbstractMonsterMetadataBundle {
                    _marker: AbstractMonster,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct IllusionerIsCelebrating(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct IllusionerSpellCasting(pub u8);
#[derive(Component)]
pub struct Illusioner;
impl Illusioner {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(IllusionerIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(IllusionerSpellCasting(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct IllusionerMetadataBundle {
    _marker: Illusioner,
    parent: AbstractMonsterMetadataBundle,
    illusioner_is_celebrating: IllusionerIsCelebrating,
    illusioner_spell_casting: IllusionerSpellCasting,
}
impl Default for IllusionerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Illusioner,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            illusioner_is_celebrating: IllusionerIsCelebrating(false),
            illusioner_spell_casting: IllusionerSpellCasting(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct InteractionWidth(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct InteractionHeight(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Response(pub bool);
#[derive(Component)]
pub struct Interaction;
impl Interaction {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(InteractionWidth(d.value.into_float()?));
            }
            9 => {
                entity.insert(InteractionHeight(d.value.into_float()?));
            }
            10 => {
                entity.insert(Response(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct InteractionMetadataBundle {
    _marker: Interaction,
    parent: AbstractEntityMetadataBundle,
    interaction_width: InteractionWidth,
    interaction_height: InteractionHeight,
    response: Response,
}
impl Default for InteractionMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Interaction,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            interaction_width: InteractionWidth(1.0),
            interaction_height: InteractionHeight(1.0),
            response: Response(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct PlayerCreated(pub bool);
#[derive(Component)]
pub struct IronGolem;
impl IronGolem {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(PlayerCreated(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct IronGolemMetadataBundle {
    _marker: IronGolem,
    parent: AbstractCreatureMetadataBundle,
    player_created: PlayerCreated,
}
impl Default for IronGolemMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: IronGolem,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            player_created: PlayerCreated(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemItem(pub Slot);
#[derive(Component)]
pub struct Item;
impl Item {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(ItemItem(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ItemMetadataBundle {
    _marker: Item,
    parent: AbstractEntityMetadataBundle,
    item_item: ItemItem,
}
impl Default for ItemMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Item,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            item_item: ItemItem(Slot::Empty),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayInterpolationStartDeltaTicks(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayInterpolationDuration(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayTranslation(pub Vec3);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayScale(pub Vec3);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayLeftRotation(pub Quaternion);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayRightRotation(pub Quaternion);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayBillboardRenderConstraints(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayBrightnessOverride(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayViewRange(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayShadowRadius(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayShadowStrength(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayWidth(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayHeight(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayGlowColorOverride(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayItemStack(pub Slot);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ItemDisplayItemDisplay(pub u8);
#[derive(Component)]
pub struct ItemDisplay;
impl ItemDisplay {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(ItemDisplayInterpolationStartDeltaTicks(d.value.into_int()?));
            }
            9 => {
                entity.insert(ItemDisplayInterpolationDuration(d.value.into_int()?));
            }
            10 => {
                entity.insert(ItemDisplayTranslation(d.value.into_vector3()?));
            }
            11 => {
                entity.insert(ItemDisplayScale(d.value.into_vector3()?));
            }
            12 => {
                entity.insert(ItemDisplayLeftRotation(d.value.into_quaternion()?));
            }
            13 => {
                entity.insert(ItemDisplayRightRotation(d.value.into_quaternion()?));
            }
            14 => {
                entity.insert(ItemDisplayBillboardRenderConstraints(d.value.into_byte()?));
            }
            15 => {
                entity.insert(ItemDisplayBrightnessOverride(d.value.into_int()?));
            }
            16 => {
                entity.insert(ItemDisplayViewRange(d.value.into_float()?));
            }
            17 => {
                entity.insert(ItemDisplayShadowRadius(d.value.into_float()?));
            }
            18 => {
                entity.insert(ItemDisplayShadowStrength(d.value.into_float()?));
            }
            19 => {
                entity.insert(ItemDisplayWidth(d.value.into_float()?));
            }
            20 => {
                entity.insert(ItemDisplayHeight(d.value.into_float()?));
            }
            21 => {
                entity.insert(ItemDisplayGlowColorOverride(d.value.into_int()?));
            }
            22 => {
                entity.insert(ItemDisplayItemStack(d.value.into_item_stack()?));
            }
            23 => {
                entity.insert(ItemDisplayItemDisplay(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ItemDisplayMetadataBundle {
    _marker: ItemDisplay,
    parent: AbstractEntityMetadataBundle,
    item_display_interpolation_start_delta_ticks: ItemDisplayInterpolationStartDeltaTicks,
    item_display_interpolation_duration: ItemDisplayInterpolationDuration,
    item_display_translation: ItemDisplayTranslation,
    item_display_scale: ItemDisplayScale,
    item_display_left_rotation: ItemDisplayLeftRotation,
    item_display_right_rotation: ItemDisplayRightRotation,
    item_display_billboard_render_constraints: ItemDisplayBillboardRenderConstraints,
    item_display_brightness_override: ItemDisplayBrightnessOverride,
    item_display_view_range: ItemDisplayViewRange,
    item_display_shadow_radius: ItemDisplayShadowRadius,
    item_display_shadow_strength: ItemDisplayShadowStrength,
    item_display_width: ItemDisplayWidth,
    item_display_height: ItemDisplayHeight,
    item_display_glow_color_override: ItemDisplayGlowColorOverride,
    item_display_item_stack: ItemDisplayItemStack,
    item_display_item_display: ItemDisplayItemDisplay,
}
impl Default for ItemDisplayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ItemDisplay,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            item_display_interpolation_start_delta_ticks: ItemDisplayInterpolationStartDeltaTicks(
                0,
            ),
            item_display_interpolation_duration: ItemDisplayInterpolationDuration(0),
            item_display_translation: ItemDisplayTranslation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            item_display_scale: ItemDisplayScale(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            item_display_left_rotation: ItemDisplayLeftRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            item_display_right_rotation: ItemDisplayRightRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            item_display_billboard_render_constraints: ItemDisplayBillboardRenderConstraints(
                Default::default(),
            ),
            item_display_brightness_override: ItemDisplayBrightnessOverride(-1),
            item_display_view_range: ItemDisplayViewRange(1.0),
            item_display_shadow_radius: ItemDisplayShadowRadius(0.0),
            item_display_shadow_strength: ItemDisplayShadowStrength(1.0),
            item_display_width: ItemDisplayWidth(0.0),
            item_display_height: ItemDisplayHeight(0.0),
            item_display_glow_color_override: ItemDisplayGlowColorOverride(-1),
            item_display_item_stack: ItemDisplayItemStack(Slot::Empty),
            item_display_item_display: ItemDisplayItemDisplay(Default::default()),
        }
    }
}

#[derive(Component)]
pub struct ItemFrame;
impl ItemFrame {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(ItemFrameItem(d.value.into_item_stack()?));
            }
            9 => {
                entity.insert(Rotation(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ItemFrameMetadataBundle {
    _marker: ItemFrame,
    parent: AbstractEntityMetadataBundle,
    item_frame_item: ItemFrameItem,
    rotation: Rotation,
}
impl Default for ItemFrameMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ItemFrame,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            item_frame_item: ItemFrameItem(Slot::Empty),
            rotation: Rotation(0),
        }
    }
}

#[derive(Component)]
pub struct LeashKnot;
impl LeashKnot {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct LeashKnotMetadataBundle {
    _marker: LeashKnot,
    parent: AbstractEntityMetadataBundle,
}
impl Default for LeashKnotMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: LeashKnot,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component)]
pub struct LightningBolt;
impl LightningBolt {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct LightningBoltMetadataBundle {
    _marker: LightningBolt,
    parent: AbstractEntityMetadataBundle,
}
impl Default for LightningBoltMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: LightningBolt,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct LlamaTamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct LlamaEating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct LlamaStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct LlamaBred(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct LlamaSaddled(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LlamaChest(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Strength(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Swag(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LlamaVariant(pub i32);
#[derive(Component)]
pub struct Llama;
impl Llama {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(LlamaTamed(bitfield & 0x2 != 0));
                entity.insert(LlamaEating(bitfield & 0x10 != 0));
                entity.insert(LlamaStanding(bitfield & 0x20 != 0));
                entity.insert(LlamaBred(bitfield & 0x8 != 0));
                entity.insert(LlamaSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(LlamaChest(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(Strength(d.value.into_int()?));
            }
            20 => {
                entity.insert(Swag(d.value.into_int()?));
            }
            21 => {
                entity.insert(LlamaVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct LlamaMetadataBundle {
    _marker: Llama,
    parent: AbstractAnimalMetadataBundle,
    llama_tamed: LlamaTamed,
    llama_eating: LlamaEating,
    llama_standing: LlamaStanding,
    llama_bred: LlamaBred,
    llama_saddled: LlamaSaddled,
    llama_chest: LlamaChest,
    strength: Strength,
    swag: Swag,
    llama_variant: LlamaVariant,
}
impl Default for LlamaMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Llama,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            llama_tamed: LlamaTamed(false),
            llama_eating: LlamaEating(false),
            llama_standing: LlamaStanding(false),
            llama_bred: LlamaBred(false),
            llama_saddled: LlamaSaddled(false),
            llama_chest: LlamaChest(false),
            strength: Strength(0),
            swag: Swag(-1),
            llama_variant: LlamaVariant(0),
        }
    }
}

#[derive(Component)]
pub struct LlamaSpit;
impl LlamaSpit {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct LlamaSpitMetadataBundle {
    _marker: LlamaSpit,
    parent: AbstractEntityMetadataBundle,
}
impl Default for LlamaSpitMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: LlamaSpit,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct SlimeSize(pub i32);
#[derive(Component)]
pub struct MagmaCube;
impl MagmaCube {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Slime::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct MagmaCubeMetadataBundle {
    _marker: MagmaCube,
    parent: SlimeMetadataBundle,
}
impl Default for MagmaCubeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: MagmaCube,
            parent: SlimeMetadataBundle {
                _marker: Slime,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
                slime_size: SlimeSize(1),
            },
        }
    }
}

#[derive(Component)]
pub struct Marker;
impl Marker {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct MarkerMetadataBundle {
    _marker: Marker,
    parent: AbstractEntityMetadataBundle,
}
impl Default for MarkerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Marker,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component)]
pub struct Minecart;
impl Minecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct MinecartMetadataBundle {
    _marker: Minecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for MinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Minecart,
            parent: AbstractMinecartMetadataBundle {
                _marker: AbstractMinecart,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct MooshroomKind(pub String);
#[derive(Component)]
pub struct Mooshroom;
impl Mooshroom {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => Cow::apply_metadata(entity, d)?,
            17 => {
                entity.insert(MooshroomKind(d.value.into_string()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct MooshroomMetadataBundle {
    _marker: Mooshroom,
    parent: CowMetadataBundle,
    mooshroom_kind: MooshroomKind,
}
impl Default for MooshroomMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Mooshroom,
            parent: CowMetadataBundle {
                _marker: Cow,
                parent: AbstractAnimalMetadataBundle {
                    _marker: AbstractAnimal,
                    parent: AbstractAgeableMetadataBundle {
                        _marker: AbstractAgeable,
                        parent: AbstractCreatureMetadataBundle {
                            _marker: AbstractCreature,
                            parent: AbstractInsentientMetadataBundle {
                                _marker: AbstractInsentient,
                                parent: AbstractLivingMetadataBundle {
                                    _marker: AbstractLiving,
                                    parent: AbstractEntityMetadataBundle {
                                        _marker: AbstractEntity,
                                        on_fire: OnFire(false),
                                        shift_key_down: ShiftKeyDown(false),
                                        sprinting: Sprinting(false),
                                        swimming: Swimming(false),
                                        currently_glowing: CurrentlyGlowing(false),
                                        invisible: Invisible(false),
                                        fall_flying: FallFlying(false),
                                        air_supply: AirSupply(Default::default()),
                                        custom_name: CustomName(None),
                                        custom_name_visible: CustomNameVisible(false),
                                        silent: Silent(false),
                                        no_gravity: NoGravity(false),
                                        pose: Pose::default(),
                                        ticks_frozen: TicksFrozen(0),
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
            },
            mooshroom_kind: MooshroomKind(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MuleTamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MuleEating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MuleStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MuleBred(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct MuleSaddled(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct MuleChest(pub bool);
#[derive(Component)]
pub struct Mule;
impl Mule {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(MuleTamed(bitfield & 0x2 != 0));
                entity.insert(MuleEating(bitfield & 0x10 != 0));
                entity.insert(MuleStanding(bitfield & 0x20 != 0));
                entity.insert(MuleBred(bitfield & 0x8 != 0));
                entity.insert(MuleSaddled(bitfield & 0x4 != 0));
            }
            18 => {
                entity.insert(MuleChest(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct MuleMetadataBundle {
    _marker: Mule,
    parent: AbstractAnimalMetadataBundle,
    mule_tamed: MuleTamed,
    mule_eating: MuleEating,
    mule_standing: MuleStanding,
    mule_bred: MuleBred,
    mule_saddled: MuleSaddled,
    mule_chest: MuleChest,
}
impl Default for MuleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Mule,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            mule_tamed: MuleTamed(false),
            mule_eating: MuleEating(false),
            mule_standing: MuleStanding(false),
            mule_bred: MuleBred(false),
            mule_saddled: MuleSaddled(false),
            mule_chest: MuleChest(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Trusting(pub bool);
#[derive(Component)]
pub struct Ocelot;
impl Ocelot {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(Trusting(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct OcelotMetadataBundle {
    _marker: Ocelot,
    parent: AbstractAnimalMetadataBundle,
    trusting: Trusting,
}
impl Default for OcelotMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Ocelot,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            trusting: Trusting(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PaintingVariant(pub azalea_registry::PaintingVariant);
#[derive(Component)]
pub struct Painting;
impl Painting {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(PaintingVariant(d.value.into_painting_variant()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PaintingMetadataBundle {
    _marker: Painting,
    parent: AbstractEntityMetadataBundle,
    painting_variant: PaintingVariant,
}
impl Default for PaintingMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Painting,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            painting_variant: PaintingVariant(azalea_registry::PaintingVariant::Kebab),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PandaUnhappyCounter(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct SneezeCounter(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct EatCounter(pub i32);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Sneezing(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct PandaSitting(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct OnBack(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct PandaRolling(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct HiddenGene(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PandaFlags(pub u8);
#[derive(Component)]
pub struct Panda;
impl Panda {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PandaUnhappyCounter(d.value.into_int()?));
            }
            18 => {
                entity.insert(SneezeCounter(d.value.into_int()?));
            }
            19 => {
                entity.insert(EatCounter(d.value.into_int()?));
            }
            20 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Sneezing(bitfield & 0x2 != 0));
                entity.insert(PandaSitting(bitfield & 0x8 != 0));
                entity.insert(OnBack(bitfield & 0x10 != 0));
                entity.insert(PandaRolling(bitfield & 0x4 != 0));
            }
            21 => {
                entity.insert(HiddenGene(d.value.into_byte()?));
            }
            22 => {
                entity.insert(PandaFlags(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PandaMetadataBundle {
    _marker: Panda,
    parent: AbstractAnimalMetadataBundle,
    panda_unhappy_counter: PandaUnhappyCounter,
    sneeze_counter: SneezeCounter,
    eat_counter: EatCounter,
    sneezing: Sneezing,
    panda_sitting: PandaSitting,
    on_back: OnBack,
    panda_rolling: PandaRolling,
    hidden_gene: HiddenGene,
    panda_flags: PandaFlags,
}
impl Default for PandaMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Panda,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            panda_unhappy_counter: PandaUnhappyCounter(0),
            sneeze_counter: SneezeCounter(0),
            eat_counter: EatCounter(0),
            sneezing: Sneezing(false),
            panda_sitting: PandaSitting(false),
            on_back: OnBack(false),
            panda_rolling: PandaRolling(false),
            hidden_gene: HiddenGene(0),
            panda_flags: PandaFlags(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ParrotVariant(pub i32);
#[derive(Component)]
pub struct Parrot;
impl Parrot {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
            19 => {
                entity.insert(ParrotVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ParrotMetadataBundle {
    _marker: Parrot,
    parent: AbstractTameableMetadataBundle,
    parrot_variant: ParrotVariant,
}
impl Default for ParrotMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Parrot,
            parent: AbstractTameableMetadataBundle {
                _marker: AbstractTameable,
                parent: AbstractAnimalMetadataBundle {
                    _marker: AbstractAnimal,
                    parent: AbstractAgeableMetadataBundle {
                        _marker: AbstractAgeable,
                        parent: AbstractCreatureMetadataBundle {
                            _marker: AbstractCreature,
                            parent: AbstractInsentientMetadataBundle {
                                _marker: AbstractInsentient,
                                parent: AbstractLivingMetadataBundle {
                                    _marker: AbstractLiving,
                                    parent: AbstractEntityMetadataBundle {
                                        _marker: AbstractEntity,
                                        on_fire: OnFire(false),
                                        shift_key_down: ShiftKeyDown(false),
                                        sprinting: Sprinting(false),
                                        swimming: Swimming(false),
                                        currently_glowing: CurrentlyGlowing(false),
                                        invisible: Invisible(false),
                                        fall_flying: FallFlying(false),
                                        air_supply: AirSupply(Default::default()),
                                        custom_name: CustomName(None),
                                        custom_name_visible: CustomNameVisible(false),
                                        silent: Silent(false),
                                        no_gravity: NoGravity(false),
                                        pose: Pose::default(),
                                        ticks_frozen: TicksFrozen(0),
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
                tame: Tame(false),
                in_sitting_pose: InSittingPose(false),
                owneruuid: Owneruuid(None),
            },
            parrot_variant: ParrotVariant(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PhantomSize(pub i32);
#[derive(Component)]
pub struct Phantom;
impl Phantom {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(PhantomSize(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PhantomMetadataBundle {
    _marker: Phantom,
    parent: AbstractInsentientMetadataBundle,
    phantom_size: PhantomSize,
}
impl Default for PhantomMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Phantom,
            parent: AbstractInsentientMetadataBundle {
                _marker: AbstractInsentient,
                parent: AbstractLivingMetadataBundle {
                    _marker: AbstractLiving,
                    parent: AbstractEntityMetadataBundle {
                        _marker: AbstractEntity,
                        on_fire: OnFire(false),
                        shift_key_down: ShiftKeyDown(false),
                        sprinting: Sprinting(false),
                        swimming: Swimming(false),
                        currently_glowing: CurrentlyGlowing(false),
                        invisible: Invisible(false),
                        fall_flying: FallFlying(false),
                        air_supply: AirSupply(Default::default()),
                        custom_name: CustomName(None),
                        custom_name_visible: CustomNameVisible(false),
                        silent: Silent(false),
                        no_gravity: NoGravity(false),
                        pose: Pose::default(),
                        ticks_frozen: TicksFrozen(0),
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            phantom_size: PhantomSize(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PigSaddle(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PigBoostTime(pub i32);
#[derive(Component)]
pub struct Pig;
impl Pig {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PigSaddle(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(PigBoostTime(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PigMetadataBundle {
    _marker: Pig,
    parent: AbstractAnimalMetadataBundle,
    pig_saddle: PigSaddle,
    pig_boost_time: PigBoostTime,
}
impl Default for PigMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Pig,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            pig_saddle: PigSaddle(false),
            pig_boost_time: PigBoostTime(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PiglinImmuneToZombification(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PiglinBaby(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PiglinIsChargingCrossbow(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct IsDancing(pub bool);
#[derive(Component)]
pub struct Piglin;
impl Piglin {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(PiglinImmuneToZombification(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(PiglinBaby(d.value.into_boolean()?));
            }
            18 => {
                entity.insert(PiglinIsChargingCrossbow(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(IsDancing(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PiglinMetadataBundle {
    _marker: Piglin,
    parent: AbstractMonsterMetadataBundle,
    piglin_immune_to_zombification: PiglinImmuneToZombification,
    piglin_baby: PiglinBaby,
    piglin_is_charging_crossbow: PiglinIsChargingCrossbow,
    is_dancing: IsDancing,
}
impl Default for PiglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Piglin,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            piglin_immune_to_zombification: PiglinImmuneToZombification(false),
            piglin_baby: PiglinBaby(false),
            piglin_is_charging_crossbow: PiglinIsChargingCrossbow(false),
            is_dancing: IsDancing(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PiglinBruteImmuneToZombification(pub bool);
#[derive(Component)]
pub struct PiglinBrute;
impl PiglinBrute {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(PiglinBruteImmuneToZombification(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PiglinBruteMetadataBundle {
    _marker: PiglinBrute,
    parent: AbstractMonsterMetadataBundle,
    piglin_brute_immune_to_zombification: PiglinBruteImmuneToZombification,
}
impl Default for PiglinBruteMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: PiglinBrute,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            piglin_brute_immune_to_zombification: PiglinBruteImmuneToZombification(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PillagerIsCelebrating(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PillagerIsChargingCrossbow(pub bool);
#[derive(Component)]
pub struct Pillager;
impl Pillager {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(PillagerIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(PillagerIsChargingCrossbow(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PillagerMetadataBundle {
    _marker: Pillager,
    parent: AbstractMonsterMetadataBundle,
    pillager_is_celebrating: PillagerIsCelebrating,
    pillager_is_charging_crossbow: PillagerIsChargingCrossbow,
}
impl Default for PillagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Pillager,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            pillager_is_celebrating: PillagerIsCelebrating(false),
            pillager_is_charging_crossbow: PillagerIsChargingCrossbow(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PlayerAbsorption(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Score(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PlayerModeCustomisation(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PlayerMainHand(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ShoulderLeft(pub azalea_nbt::NbtTag);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ShoulderRight(pub azalea_nbt::NbtTag);
#[derive(Component)]
pub struct Player;
impl Player {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::apply_metadata(entity, d)?,
            15 => {
                entity.insert(PlayerAbsorption(d.value.into_float()?));
            }
            16 => {
                entity.insert(Score(d.value.into_int()?));
            }
            17 => {
                entity.insert(PlayerModeCustomisation(d.value.into_byte()?));
            }
            18 => {
                entity.insert(PlayerMainHand(d.value.into_byte()?));
            }
            19 => {
                entity.insert(ShoulderLeft(d.value.into_compound_tag()?));
            }
            20 => {
                entity.insert(ShoulderRight(d.value.into_compound_tag()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PlayerMetadataBundle {
    _marker: Player,
    parent: AbstractLivingMetadataBundle,
    player_absorption: PlayerAbsorption,
    score: Score,
    player_mode_customisation: PlayerModeCustomisation,
    player_main_hand: PlayerMainHand,
    shoulder_left: ShoulderLeft,
    shoulder_right: ShoulderRight,
}
impl Default for PlayerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Player,
            parent: AbstractLivingMetadataBundle {
                _marker: AbstractLiving,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                auto_spin_attack: AutoSpinAttack(false),
                abstract_living_using_item: AbstractLivingUsingItem(false),
                health: Health(1.0),
                abstract_living_effect_color: AbstractLivingEffectColor(0),
                effect_ambience: EffectAmbience(false),
                arrow_count: ArrowCount(0),
                stinger_count: StingerCount(0),
                sleeping_pos: SleepingPos(None),
            },
            player_absorption: PlayerAbsorption(0.0),
            score: Score(0),
            player_mode_customisation: PlayerModeCustomisation(0),
            player_main_hand: PlayerMainHand(1),
            shoulder_left: ShoulderLeft(azalea_nbt::NbtTag::Compound(Default::default())),
            shoulder_right: ShoulderRight(azalea_nbt::NbtTag::Compound(Default::default())),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PolarBearStanding(pub bool);
#[derive(Component)]
pub struct PolarBear;
impl PolarBear {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(PolarBearStanding(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PolarBearMetadataBundle {
    _marker: PolarBear,
    parent: AbstractAnimalMetadataBundle,
    polar_bear_standing: PolarBearStanding,
}
impl Default for PolarBearMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: PolarBear,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            polar_bear_standing: PolarBearStanding(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PotionItemStack(pub Slot);
#[derive(Component)]
pub struct Potion;
impl Potion {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(PotionItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PotionMetadataBundle {
    _marker: Potion,
    parent: AbstractEntityMetadataBundle,
    potion_item_stack: PotionItemStack,
}
impl Default for PotionMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Potion,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            potion_item_stack: PotionItemStack(Slot::Empty),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct PufferfishFromBucket(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct PuffState(pub i32);
#[derive(Component)]
pub struct Pufferfish;
impl Pufferfish {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(PufferfishFromBucket(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(PuffState(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct PufferfishMetadataBundle {
    _marker: Pufferfish,
    parent: AbstractCreatureMetadataBundle,
    pufferfish_from_bucket: PufferfishFromBucket,
    puff_state: PuffState,
}
impl Default for PufferfishMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Pufferfish,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            pufferfish_from_bucket: PufferfishFromBucket(false),
            puff_state: PuffState(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct RabbitKind(pub i32);
#[derive(Component)]
pub struct Rabbit;
impl Rabbit {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(RabbitKind(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct RabbitMetadataBundle {
    _marker: Rabbit,
    parent: AbstractAnimalMetadataBundle,
    rabbit_kind: RabbitKind,
}
impl Default for RabbitMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Rabbit,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            rabbit_kind: RabbitKind(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct RavagerIsCelebrating(pub bool);
#[derive(Component)]
pub struct Ravager;
impl Ravager {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(RavagerIsCelebrating(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct RavagerMetadataBundle {
    _marker: Ravager,
    parent: AbstractMonsterMetadataBundle,
    ravager_is_celebrating: RavagerIsCelebrating,
}
impl Default for RavagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Ravager,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            ravager_is_celebrating: RavagerIsCelebrating(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct SalmonFromBucket(pub bool);
#[derive(Component)]
pub struct Salmon;
impl Salmon {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(SalmonFromBucket(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SalmonMetadataBundle {
    _marker: Salmon,
    parent: AbstractCreatureMetadataBundle,
    salmon_from_bucket: SalmonFromBucket,
}
impl Default for SalmonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Salmon,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            salmon_from_bucket: SalmonFromBucket(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct Sheared(pub bool);
#[derive(Component)]
pub struct Sheep;
impl Sheep {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Sheared(bitfield & 0x10 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SheepMetadataBundle {
    _marker: Sheep,
    parent: AbstractAnimalMetadataBundle,
    sheared: Sheared,
}
impl Default for SheepMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Sheep,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            sheared: Sheared(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct AttachFace(pub Direction);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Peek(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ShulkerColor(pub u8);
#[derive(Component)]
pub struct Shulker;
impl Shulker {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(AttachFace(d.value.into_direction()?));
            }
            17 => {
                entity.insert(Peek(d.value.into_byte()?));
            }
            18 => {
                entity.insert(ShulkerColor(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ShulkerMetadataBundle {
    _marker: Shulker,
    parent: AbstractCreatureMetadataBundle,
    attach_face: AttachFace,
    peek: Peek,
    shulker_color: ShulkerColor,
}
impl Default for ShulkerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Shulker,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            attach_face: AttachFace(Default::default()),
            peek: Peek(0),
            shulker_color: ShulkerColor(16),
        }
    }
}

#[derive(Component)]
pub struct ShulkerBullet;
impl ShulkerBullet {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ShulkerBulletMetadataBundle {
    _marker: ShulkerBullet,
    parent: AbstractEntityMetadataBundle,
}
impl Default for ShulkerBulletMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ShulkerBullet,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
        }
    }
}

#[derive(Component)]
pub struct Silverfish;
impl Silverfish {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SilverfishMetadataBundle {
    _marker: Silverfish,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for SilverfishMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Silverfish,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct StrayConversion(pub bool);
#[derive(Component)]
pub struct Skeleton;
impl Skeleton {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(StrayConversion(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SkeletonMetadataBundle {
    _marker: Skeleton,
    parent: AbstractMonsterMetadataBundle,
    stray_conversion: StrayConversion,
}
impl Default for SkeletonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Skeleton,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            stray_conversion: StrayConversion(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SkeletonHorseTamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SkeletonHorseEating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SkeletonHorseStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SkeletonHorseBred(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SkeletonHorseSaddled(pub bool);
#[derive(Component)]
pub struct SkeletonHorse;
impl SkeletonHorse {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(SkeletonHorseTamed(bitfield & 0x2 != 0));
                entity.insert(SkeletonHorseEating(bitfield & 0x10 != 0));
                entity.insert(SkeletonHorseStanding(bitfield & 0x20 != 0));
                entity.insert(SkeletonHorseBred(bitfield & 0x8 != 0));
                entity.insert(SkeletonHorseSaddled(bitfield & 0x4 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SkeletonHorseMetadataBundle {
    _marker: SkeletonHorse,
    parent: AbstractAnimalMetadataBundle,
    skeleton_horse_tamed: SkeletonHorseTamed,
    skeleton_horse_eating: SkeletonHorseEating,
    skeleton_horse_standing: SkeletonHorseStanding,
    skeleton_horse_bred: SkeletonHorseBred,
    skeleton_horse_saddled: SkeletonHorseSaddled,
}
impl Default for SkeletonHorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SkeletonHorse,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            skeleton_horse_tamed: SkeletonHorseTamed(false),
            skeleton_horse_eating: SkeletonHorseEating(false),
            skeleton_horse_standing: SkeletonHorseStanding(false),
            skeleton_horse_bred: SkeletonHorseBred(false),
            skeleton_horse_saddled: SkeletonHorseSaddled(false),
        }
    }
}

#[derive(Component)]
pub struct Slime;
impl Slime {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            16 => {
                entity.insert(SlimeSize(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SlimeMetadataBundle {
    _marker: Slime,
    parent: AbstractInsentientMetadataBundle,
    slime_size: SlimeSize,
}
impl Default for SlimeMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Slime,
            parent: AbstractInsentientMetadataBundle {
                _marker: AbstractInsentient,
                parent: AbstractLivingMetadataBundle {
                    _marker: AbstractLiving,
                    parent: AbstractEntityMetadataBundle {
                        _marker: AbstractEntity,
                        on_fire: OnFire(false),
                        shift_key_down: ShiftKeyDown(false),
                        sprinting: Sprinting(false),
                        swimming: Swimming(false),
                        currently_glowing: CurrentlyGlowing(false),
                        invisible: Invisible(false),
                        fall_flying: FallFlying(false),
                        air_supply: AirSupply(Default::default()),
                        custom_name: CustomName(None),
                        custom_name_visible: CustomNameVisible(false),
                        silent: Silent(false),
                        no_gravity: NoGravity(false),
                        pose: Pose::default(),
                        ticks_frozen: TicksFrozen(0),
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
            slime_size: SlimeSize(1),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct SmallFireballItemStack(pub Slot);
#[derive(Component)]
pub struct SmallFireball;
impl SmallFireball {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(SmallFireballItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SmallFireballMetadataBundle {
    _marker: SmallFireball,
    parent: AbstractEntityMetadataBundle,
    small_fireball_item_stack: SmallFireballItemStack,
}
impl Default for SmallFireballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SmallFireball,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            small_fireball_item_stack: SmallFireballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct State(pub SnifferState);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct DropSeedAtTick(pub i32);
#[derive(Component)]
pub struct Sniffer;
impl Sniffer {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(State(d.value.into_sniffer_state()?));
            }
            18 => {
                entity.insert(DropSeedAtTick(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SnifferMetadataBundle {
    _marker: Sniffer,
    parent: AbstractAnimalMetadataBundle,
    state: State,
    drop_seed_at_tick: DropSeedAtTick,
}
impl Default for SnifferMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Sniffer,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            state: State(Default::default()),
            drop_seed_at_tick: DropSeedAtTick(Default::default()),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct HasPumpkin(pub bool);
#[derive(Component)]
pub struct SnowGolem;
impl SnowGolem {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(HasPumpkin(bitfield & 0x10 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SnowGolemMetadataBundle {
    _marker: SnowGolem,
    parent: AbstractCreatureMetadataBundle,
    has_pumpkin: HasPumpkin,
}
impl Default for SnowGolemMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SnowGolem,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            has_pumpkin: HasPumpkin(true),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct SnowballItemStack(pub Slot);
#[derive(Component)]
pub struct Snowball;
impl Snowball {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(SnowballItemStack(d.value.into_item_stack()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SnowballMetadataBundle {
    _marker: Snowball,
    parent: AbstractEntityMetadataBundle,
    snowball_item_stack: SnowballItemStack,
}
impl Default for SnowballMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Snowball,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            snowball_item_stack: SnowballItemStack(Slot::Empty),
        }
    }
}

#[derive(Component)]
pub struct SpawnerMinecart;
impl SpawnerMinecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SpawnerMinecartMetadataBundle {
    _marker: SpawnerMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for SpawnerMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SpawnerMinecart,
            parent: AbstractMinecartMetadataBundle {
                _marker: AbstractMinecart,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SpectralArrowCritArrow(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SpectralArrowShotFromCrossbow(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct SpectralArrowNoPhysics(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct SpectralArrowPierceLevel(pub u8);
#[derive(Component)]
pub struct SpectralArrow;
impl SpectralArrow {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(SpectralArrowCritArrow(bitfield & 0x1 != 0));
                entity.insert(SpectralArrowShotFromCrossbow(bitfield & 0x4 != 0));
                entity.insert(SpectralArrowNoPhysics(bitfield & 0x2 != 0));
            }
            9 => {
                entity.insert(SpectralArrowPierceLevel(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SpectralArrowMetadataBundle {
    _marker: SpectralArrow,
    parent: AbstractEntityMetadataBundle,
    spectral_arrow_crit_arrow: SpectralArrowCritArrow,
    spectral_arrow_shot_from_crossbow: SpectralArrowShotFromCrossbow,
    spectral_arrow_no_physics: SpectralArrowNoPhysics,
    spectral_arrow_pierce_level: SpectralArrowPierceLevel,
}
impl Default for SpectralArrowMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: SpectralArrow,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            spectral_arrow_crit_arrow: SpectralArrowCritArrow(false),
            spectral_arrow_shot_from_crossbow: SpectralArrowShotFromCrossbow(false),
            spectral_arrow_no_physics: SpectralArrowNoPhysics(false),
            spectral_arrow_pierce_level: SpectralArrowPierceLevel(0),
        }
    }
}

#[derive(Component)]
pub struct Spider;
impl Spider {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Climbing(bitfield & 0x1 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SpiderMetadataBundle {
    _marker: Spider,
    parent: AbstractMonsterMetadataBundle,
    climbing: Climbing,
}
impl Default for SpiderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Spider,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            climbing: Climbing(false),
        }
    }
}

#[derive(Component)]
pub struct Squid;
impl Squid {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct SquidMetadataBundle {
    _marker: Squid,
    parent: AbstractCreatureMetadataBundle,
}
impl Default for SquidMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Squid,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
        }
    }
}

#[derive(Component)]
pub struct Stray;
impl Stray {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct StrayMetadataBundle {
    _marker: Stray,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for StrayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Stray,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct StriderBoostTime(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Suffocating(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct StriderSaddle(pub bool);
#[derive(Component)]
pub struct Strider;
impl Strider {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(StriderBoostTime(d.value.into_int()?));
            }
            18 => {
                entity.insert(Suffocating(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(StriderSaddle(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct StriderMetadataBundle {
    _marker: Strider,
    parent: AbstractAnimalMetadataBundle,
    strider_boost_time: StriderBoostTime,
    suffocating: Suffocating,
    strider_saddle: StriderSaddle,
}
impl Default for StriderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Strider,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            strider_boost_time: StriderBoostTime(0),
            suffocating: Suffocating(false),
            strider_saddle: StriderSaddle(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct TadpoleFromBucket(pub bool);
#[derive(Component)]
pub struct Tadpole;
impl Tadpole {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(TadpoleFromBucket(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TadpoleMetadataBundle {
    _marker: Tadpole,
    parent: AbstractCreatureMetadataBundle,
    tadpole_from_bucket: TadpoleFromBucket,
}
impl Default for TadpoleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Tadpole,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            tadpole_from_bucket: TadpoleFromBucket(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayInterpolationStartDeltaTicks(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayInterpolationDuration(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayTranslation(pub Vec3);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayScale(pub Vec3);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayLeftRotation(pub Quaternion);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayRightRotation(pub Quaternion);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayBillboardRenderConstraints(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayBrightnessOverride(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayViewRange(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayShadowRadius(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayShadowStrength(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayWidth(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayHeight(pub f32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextDisplayGlowColorOverride(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Text(pub FormattedText);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LineWidth(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct BackgroundColor(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TextOpacity(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct StyleFlags(pub u8);
#[derive(Component)]
pub struct TextDisplay;
impl TextDisplay {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(TextDisplayInterpolationStartDeltaTicks(d.value.into_int()?));
            }
            9 => {
                entity.insert(TextDisplayInterpolationDuration(d.value.into_int()?));
            }
            10 => {
                entity.insert(TextDisplayTranslation(d.value.into_vector3()?));
            }
            11 => {
                entity.insert(TextDisplayScale(d.value.into_vector3()?));
            }
            12 => {
                entity.insert(TextDisplayLeftRotation(d.value.into_quaternion()?));
            }
            13 => {
                entity.insert(TextDisplayRightRotation(d.value.into_quaternion()?));
            }
            14 => {
                entity.insert(TextDisplayBillboardRenderConstraints(d.value.into_byte()?));
            }
            15 => {
                entity.insert(TextDisplayBrightnessOverride(d.value.into_int()?));
            }
            16 => {
                entity.insert(TextDisplayViewRange(d.value.into_float()?));
            }
            17 => {
                entity.insert(TextDisplayShadowRadius(d.value.into_float()?));
            }
            18 => {
                entity.insert(TextDisplayShadowStrength(d.value.into_float()?));
            }
            19 => {
                entity.insert(TextDisplayWidth(d.value.into_float()?));
            }
            20 => {
                entity.insert(TextDisplayHeight(d.value.into_float()?));
            }
            21 => {
                entity.insert(TextDisplayGlowColorOverride(d.value.into_int()?));
            }
            22 => {
                entity.insert(Text(d.value.into_formatted_text()?));
            }
            23 => {
                entity.insert(LineWidth(d.value.into_int()?));
            }
            24 => {
                entity.insert(BackgroundColor(d.value.into_int()?));
            }
            25 => {
                entity.insert(TextOpacity(d.value.into_byte()?));
            }
            26 => {
                entity.insert(StyleFlags(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TextDisplayMetadataBundle {
    _marker: TextDisplay,
    parent: AbstractEntityMetadataBundle,
    text_display_interpolation_start_delta_ticks: TextDisplayInterpolationStartDeltaTicks,
    text_display_interpolation_duration: TextDisplayInterpolationDuration,
    text_display_translation: TextDisplayTranslation,
    text_display_scale: TextDisplayScale,
    text_display_left_rotation: TextDisplayLeftRotation,
    text_display_right_rotation: TextDisplayRightRotation,
    text_display_billboard_render_constraints: TextDisplayBillboardRenderConstraints,
    text_display_brightness_override: TextDisplayBrightnessOverride,
    text_display_view_range: TextDisplayViewRange,
    text_display_shadow_radius: TextDisplayShadowRadius,
    text_display_shadow_strength: TextDisplayShadowStrength,
    text_display_width: TextDisplayWidth,
    text_display_height: TextDisplayHeight,
    text_display_glow_color_override: TextDisplayGlowColorOverride,
    text: Text,
    line_width: LineWidth,
    background_color: BackgroundColor,
    text_opacity: TextOpacity,
    style_flags: StyleFlags,
}
impl Default for TextDisplayMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TextDisplay,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            text_display_interpolation_start_delta_ticks: TextDisplayInterpolationStartDeltaTicks(
                0,
            ),
            text_display_interpolation_duration: TextDisplayInterpolationDuration(0),
            text_display_translation: TextDisplayTranslation(Vec3 {
                x: 0.0,
                y: 0.0,
                z: 0.0,
            }),
            text_display_scale: TextDisplayScale(Vec3 {
                x: 1.0,
                y: 1.0,
                z: 1.0,
            }),
            text_display_left_rotation: TextDisplayLeftRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            text_display_right_rotation: TextDisplayRightRotation(Quaternion {
                x: 0.0,
                y: 0.0,
                z: 0.0,
                w: 1.0,
            }),
            text_display_billboard_render_constraints: TextDisplayBillboardRenderConstraints(
                Default::default(),
            ),
            text_display_brightness_override: TextDisplayBrightnessOverride(-1),
            text_display_view_range: TextDisplayViewRange(1.0),
            text_display_shadow_radius: TextDisplayShadowRadius(0.0),
            text_display_shadow_strength: TextDisplayShadowStrength(1.0),
            text_display_width: TextDisplayWidth(0.0),
            text_display_height: TextDisplayHeight(0.0),
            text_display_glow_color_override: TextDisplayGlowColorOverride(-1),
            text: Text(Default::default()),
            line_width: LineWidth(200),
            background_color: BackgroundColor(1073741824),
            text_opacity: TextOpacity(127),
            style_flags: StyleFlags(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Fuse(pub i32);
#[derive(Component)]
pub struct Tnt;
impl Tnt {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Fuse(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TntMetadataBundle {
    _marker: Tnt,
    parent: AbstractEntityMetadataBundle,
    fuse: Fuse,
}
impl Default for TntMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Tnt,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            fuse: Fuse(80),
        }
    }
}

#[derive(Component)]
pub struct TntMinecart;
impl TntMinecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=13 => AbstractMinecart::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TntMinecartMetadataBundle {
    _marker: TntMinecart,
    parent: AbstractMinecartMetadataBundle,
}
impl Default for TntMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TntMinecart,
            parent: AbstractMinecartMetadataBundle {
                _marker: AbstractMinecart,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                abstract_minecart_hurt: AbstractMinecartHurt(0),
                abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
                abstract_minecart_damage: AbstractMinecartDamage(0.0),
                display_block: DisplayBlock(Default::default()),
                display_offset: DisplayOffset(6),
                custom_display: CustomDisplay(false),
            },
        }
    }
}

#[derive(Component)]
pub struct TraderLlama;
impl TraderLlama {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=21 => Llama::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TraderLlamaMetadataBundle {
    _marker: TraderLlama,
    parent: LlamaMetadataBundle,
}
impl Default for TraderLlamaMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TraderLlama,
            parent: LlamaMetadataBundle {
                _marker: Llama,
                parent: AbstractAnimalMetadataBundle {
                    _marker: AbstractAnimal,
                    parent: AbstractAgeableMetadataBundle {
                        _marker: AbstractAgeable,
                        parent: AbstractCreatureMetadataBundle {
                            _marker: AbstractCreature,
                            parent: AbstractInsentientMetadataBundle {
                                _marker: AbstractInsentient,
                                parent: AbstractLivingMetadataBundle {
                                    _marker: AbstractLiving,
                                    parent: AbstractEntityMetadataBundle {
                                        _marker: AbstractEntity,
                                        on_fire: OnFire(false),
                                        shift_key_down: ShiftKeyDown(false),
                                        sprinting: Sprinting(false),
                                        swimming: Swimming(false),
                                        currently_glowing: CurrentlyGlowing(false),
                                        invisible: Invisible(false),
                                        fall_flying: FallFlying(false),
                                        air_supply: AirSupply(Default::default()),
                                        custom_name: CustomName(None),
                                        custom_name_visible: CustomNameVisible(false),
                                        silent: Silent(false),
                                        no_gravity: NoGravity(false),
                                        pose: Pose::default(),
                                        ticks_frozen: TicksFrozen(0),
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
                llama_tamed: LlamaTamed(false),
                llama_eating: LlamaEating(false),
                llama_standing: LlamaStanding(false),
                llama_bred: LlamaBred(false),
                llama_saddled: LlamaSaddled(false),
                llama_chest: LlamaChest(false),
                strength: Strength(0),
                swag: Swag(-1),
                llama_variant: LlamaVariant(0),
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct TridentCritArrow(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct TridentShotFromCrossbow(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct TridentNoPhysics(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TridentPierceLevel(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Loyalty(pub u8);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Foil(pub bool);
#[derive(Component)]
pub struct Trident;
impl Trident {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(TridentCritArrow(bitfield & 0x1 != 0));
                entity.insert(TridentShotFromCrossbow(bitfield & 0x4 != 0));
                entity.insert(TridentNoPhysics(bitfield & 0x2 != 0));
            }
            9 => {
                entity.insert(TridentPierceLevel(d.value.into_byte()?));
            }
            10 => {
                entity.insert(Loyalty(d.value.into_byte()?));
            }
            11 => {
                entity.insert(Foil(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TridentMetadataBundle {
    _marker: Trident,
    parent: AbstractEntityMetadataBundle,
    trident_crit_arrow: TridentCritArrow,
    trident_shot_from_crossbow: TridentShotFromCrossbow,
    trident_no_physics: TridentNoPhysics,
    trident_pierce_level: TridentPierceLevel,
    loyalty: Loyalty,
    foil: Foil,
}
impl Default for TridentMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Trident,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            trident_crit_arrow: TridentCritArrow(false),
            trident_shot_from_crossbow: TridentShotFromCrossbow(false),
            trident_no_physics: TridentNoPhysics(false),
            trident_pierce_level: TridentPierceLevel(0),
            loyalty: Loyalty(0),
            foil: Foil(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct TropicalFishFromBucket(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TropicalFishTypeVariant(pub i32);
#[derive(Component)]
pub struct TropicalFish;
impl TropicalFish {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(TropicalFishFromBucket(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(TropicalFishTypeVariant(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TropicalFishMetadataBundle {
    _marker: TropicalFish,
    parent: AbstractCreatureMetadataBundle,
    tropical_fish_from_bucket: TropicalFishFromBucket,
    tropical_fish_type_variant: TropicalFishTypeVariant,
}
impl Default for TropicalFishMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: TropicalFish,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            tropical_fish_from_bucket: TropicalFishFromBucket(false),
            tropical_fish_type_variant: TropicalFishTypeVariant(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct HomePos(pub BlockPos);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct HasEgg(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct LayingEgg(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TravelPos(pub BlockPos);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct GoingHome(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Travelling(pub bool);
#[derive(Component)]
pub struct Turtle;
impl Turtle {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                entity.insert(HomePos(d.value.into_block_pos()?));
            }
            18 => {
                entity.insert(HasEgg(d.value.into_boolean()?));
            }
            19 => {
                entity.insert(LayingEgg(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(TravelPos(d.value.into_block_pos()?));
            }
            21 => {
                entity.insert(GoingHome(d.value.into_boolean()?));
            }
            22 => {
                entity.insert(Travelling(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct TurtleMetadataBundle {
    _marker: Turtle,
    parent: AbstractAnimalMetadataBundle,
    home_pos: HomePos,
    has_egg: HasEgg,
    laying_egg: LayingEgg,
    travel_pos: TravelPos,
    going_home: GoingHome,
    travelling: Travelling,
}
impl Default for TurtleMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Turtle,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            home_pos: HomePos(BlockPos::new(0, 0, 0)),
            has_egg: HasEgg(false),
            laying_egg: LayingEgg(false),
            travel_pos: TravelPos(BlockPos::new(0, 0, 0)),
            going_home: GoingHome(false),
            travelling: Travelling(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct VexFlags(pub u8);
#[derive(Component)]
pub struct Vex;
impl Vex {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(VexFlags(d.value.into_byte()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct VexMetadataBundle {
    _marker: Vex,
    parent: AbstractMonsterMetadataBundle,
    vex_flags: VexFlags,
}
impl Default for VexMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Vex,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            vex_flags: VexFlags(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct VillagerUnhappyCounter(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct VillagerVillagerData(pub VillagerData);
#[derive(Component)]
pub struct Villager;
impl Villager {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::apply_metadata(entity, d)?,
            17 => {
                entity.insert(VillagerUnhappyCounter(d.value.into_int()?));
            }
            18 => {
                entity.insert(VillagerVillagerData(d.value.into_villager_data()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct VillagerMetadataBundle {
    _marker: Villager,
    parent: AbstractAgeableMetadataBundle,
    villager_unhappy_counter: VillagerUnhappyCounter,
    villager_villager_data: VillagerVillagerData,
}
impl Default for VillagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Villager,
            parent: AbstractAgeableMetadataBundle {
                _marker: AbstractAgeable,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
                abstract_ageable_baby: AbstractAgeableBaby(false),
            },
            villager_unhappy_counter: VillagerUnhappyCounter(0),
            villager_villager_data: VillagerVillagerData(VillagerData {
                kind: azalea_registry::VillagerKind::Plains,
                profession: azalea_registry::VillagerProfession::None,
                level: 0,
            }),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct VindicatorIsCelebrating(pub bool);
#[derive(Component)]
pub struct Vindicator;
impl Vindicator {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(VindicatorIsCelebrating(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct VindicatorMetadataBundle {
    _marker: Vindicator,
    parent: AbstractMonsterMetadataBundle,
    vindicator_is_celebrating: VindicatorIsCelebrating,
}
impl Default for VindicatorMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Vindicator,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            vindicator_is_celebrating: VindicatorIsCelebrating(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct WanderingTraderUnhappyCounter(pub i32);
#[derive(Component)]
pub struct WanderingTrader;
impl WanderingTrader {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::apply_metadata(entity, d)?,
            17 => {
                entity.insert(WanderingTraderUnhappyCounter(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct WanderingTraderMetadataBundle {
    _marker: WanderingTrader,
    parent: AbstractAgeableMetadataBundle,
    wandering_trader_unhappy_counter: WanderingTraderUnhappyCounter,
}
impl Default for WanderingTraderMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: WanderingTrader,
            parent: AbstractAgeableMetadataBundle {
                _marker: AbstractAgeable,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
                abstract_ageable_baby: AbstractAgeableBaby(false),
            },
            wandering_trader_unhappy_counter: WanderingTraderUnhappyCounter(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ClientAngerLevel(pub i32);
#[derive(Component)]
pub struct Warden;
impl Warden {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(ClientAngerLevel(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct WardenMetadataBundle {
    _marker: Warden,
    parent: AbstractMonsterMetadataBundle,
    client_anger_level: ClientAngerLevel,
}
impl Default for WardenMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Warden,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            client_anger_level: ClientAngerLevel(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct WitchIsCelebrating(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct WitchUsingItem(pub bool);
#[derive(Component)]
pub struct Witch;
impl Witch {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(WitchIsCelebrating(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(WitchUsingItem(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct WitchMetadataBundle {
    _marker: Witch,
    parent: AbstractMonsterMetadataBundle,
    witch_is_celebrating: WitchIsCelebrating,
    witch_using_item: WitchUsingItem,
}
impl Default for WitchMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Witch,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            witch_is_celebrating: WitchIsCelebrating(false),
            witch_using_item: WitchUsingItem(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct TargetA(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TargetB(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct TargetC(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct Inv(pub i32);
#[derive(Component)]
pub struct Wither;
impl Wither {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(TargetA(d.value.into_int()?));
            }
            17 => {
                entity.insert(TargetB(d.value.into_int()?));
            }
            18 => {
                entity.insert(TargetC(d.value.into_int()?));
            }
            19 => {
                entity.insert(Inv(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct WitherMetadataBundle {
    _marker: Wither,
    parent: AbstractMonsterMetadataBundle,
    target_a: TargetA,
    target_b: TargetB,
    target_c: TargetC,
    inv: Inv,
}
impl Default for WitherMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Wither,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            target_a: TargetA(0),
            target_b: TargetB(0),
            target_c: TargetC(0),
            inv: Inv(0),
        }
    }
}

#[derive(Component)]
pub struct WitherSkeleton;
impl WitherSkeleton {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct WitherSkeletonMetadataBundle {
    _marker: WitherSkeleton,
    parent: AbstractMonsterMetadataBundle,
}
impl Default for WitherSkeletonMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: WitherSkeleton,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Dangerous(pub bool);
#[derive(Component)]
pub struct WitherSkull;
impl WitherSkull {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(Dangerous(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct WitherSkullMetadataBundle {
    _marker: WitherSkull,
    parent: AbstractEntityMetadataBundle,
    dangerous: Dangerous,
}
impl Default for WitherSkullMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: WitherSkull,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            dangerous: Dangerous(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct WolfInterested(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct WolfCollarColor(pub i32);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct WolfRemainingAngerTime(pub i32);
#[derive(Component)]
pub struct Wolf;
impl Wolf {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => AbstractTameable::apply_metadata(entity, d)?,
            19 => {
                entity.insert(WolfInterested(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(WolfCollarColor(d.value.into_int()?));
            }
            21 => {
                entity.insert(WolfRemainingAngerTime(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct WolfMetadataBundle {
    _marker: Wolf,
    parent: AbstractTameableMetadataBundle,
    wolf_interested: WolfInterested,
    wolf_collar_color: WolfCollarColor,
    wolf_remaining_anger_time: WolfRemainingAngerTime,
}
impl Default for WolfMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Wolf,
            parent: AbstractTameableMetadataBundle {
                _marker: AbstractTameable,
                parent: AbstractAnimalMetadataBundle {
                    _marker: AbstractAnimal,
                    parent: AbstractAgeableMetadataBundle {
                        _marker: AbstractAgeable,
                        parent: AbstractCreatureMetadataBundle {
                            _marker: AbstractCreature,
                            parent: AbstractInsentientMetadataBundle {
                                _marker: AbstractInsentient,
                                parent: AbstractLivingMetadataBundle {
                                    _marker: AbstractLiving,
                                    parent: AbstractEntityMetadataBundle {
                                        _marker: AbstractEntity,
                                        on_fire: OnFire(false),
                                        shift_key_down: ShiftKeyDown(false),
                                        sprinting: Sprinting(false),
                                        swimming: Swimming(false),
                                        currently_glowing: CurrentlyGlowing(false),
                                        invisible: Invisible(false),
                                        fall_flying: FallFlying(false),
                                        air_supply: AirSupply(Default::default()),
                                        custom_name: CustomName(None),
                                        custom_name_visible: CustomNameVisible(false),
                                        silent: Silent(false),
                                        no_gravity: NoGravity(false),
                                        pose: Pose::default(),
                                        ticks_frozen: TicksFrozen(0),
                                    },
                                    auto_spin_attack: AutoSpinAttack(false),
                                    abstract_living_using_item: AbstractLivingUsingItem(false),
                                    health: Health(1.0),
                                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                                    effect_ambience: EffectAmbience(false),
                                    arrow_count: ArrowCount(0),
                                    stinger_count: StingerCount(0),
                                    sleeping_pos: SleepingPos(None),
                                },
                                no_ai: NoAi(false),
                                left_handed: LeftHanded(false),
                                aggressive: Aggressive(false),
                            },
                        },
                        abstract_ageable_baby: AbstractAgeableBaby(false),
                    },
                },
                tame: Tame(false),
                in_sitting_pose: InSittingPose(false),
                owneruuid: Owneruuid(None),
            },
            wolf_interested: WolfInterested(false),
            wolf_collar_color: WolfCollarColor(Default::default()),
            wolf_remaining_anger_time: WolfRemainingAngerTime(0),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct ZoglinBaby(pub bool);
#[derive(Component)]
pub struct Zoglin;
impl Zoglin {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(ZoglinBaby(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ZoglinMetadataBundle {
    _marker: Zoglin,
    parent: AbstractMonsterMetadataBundle,
    zoglin_baby: ZoglinBaby,
}
impl Default for ZoglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Zoglin,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            zoglin_baby: ZoglinBaby(false),
        }
    }
}

#[derive(Component)]
pub struct Zombie;
impl Zombie {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractMonster::apply_metadata(entity, d)?,
            16 => {
                entity.insert(ZombieBaby(d.value.into_boolean()?));
            }
            17 => {
                entity.insert(SpecialType(d.value.into_int()?));
            }
            18 => {
                entity.insert(DrownedConversion(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ZombieMetadataBundle {
    _marker: Zombie,
    parent: AbstractMonsterMetadataBundle,
    zombie_baby: ZombieBaby,
    special_type: SpecialType,
    drowned_conversion: DrownedConversion,
}
impl Default for ZombieMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: Zombie,
            parent: AbstractMonsterMetadataBundle {
                _marker: AbstractMonster,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
            },
            zombie_baby: ZombieBaby(false),
            special_type: SpecialType(0),
            drowned_conversion: DrownedConversion(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ZombieHorseTamed(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ZombieHorseEating(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ZombieHorseStanding(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ZombieHorseBred(pub bool);
#[derive(Component, Deref, DerefMut, Clone, Copy)]
pub struct ZombieHorseSaddled(pub bool);
#[derive(Component)]
pub struct ZombieHorse;
impl ZombieHorse {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(ZombieHorseTamed(bitfield & 0x2 != 0));
                entity.insert(ZombieHorseEating(bitfield & 0x10 != 0));
                entity.insert(ZombieHorseStanding(bitfield & 0x20 != 0));
                entity.insert(ZombieHorseBred(bitfield & 0x8 != 0));
                entity.insert(ZombieHorseSaddled(bitfield & 0x4 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ZombieHorseMetadataBundle {
    _marker: ZombieHorse,
    parent: AbstractAnimalMetadataBundle,
    zombie_horse_tamed: ZombieHorseTamed,
    zombie_horse_eating: ZombieHorseEating,
    zombie_horse_standing: ZombieHorseStanding,
    zombie_horse_bred: ZombieHorseBred,
    zombie_horse_saddled: ZombieHorseSaddled,
}
impl Default for ZombieHorseMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ZombieHorse,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            zombie_horse_tamed: ZombieHorseTamed(false),
            zombie_horse_eating: ZombieHorseEating(false),
            zombie_horse_standing: ZombieHorseStanding(false),
            zombie_horse_bred: ZombieHorseBred(false),
            zombie_horse_saddled: ZombieHorseSaddled(false),
        }
    }
}

#[derive(Component, Deref, DerefMut, Clone)]
pub struct Converting(pub bool);
#[derive(Component, Deref, DerefMut, Clone)]
pub struct ZombieVillagerVillagerData(pub VillagerData);
#[derive(Component)]
pub struct ZombieVillager;
impl ZombieVillager {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            19 => {
                entity.insert(Converting(d.value.into_boolean()?));
            }
            20 => {
                entity.insert(ZombieVillagerVillagerData(d.value.into_villager_data()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ZombieVillagerMetadataBundle {
    _marker: ZombieVillager,
    parent: ZombieMetadataBundle,
    converting: Converting,
    zombie_villager_villager_data: ZombieVillagerVillagerData,
}
impl Default for ZombieVillagerMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ZombieVillager,
            parent: ZombieMetadataBundle {
                _marker: Zombie,
                parent: AbstractMonsterMetadataBundle {
                    _marker: AbstractMonster,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
            converting: Converting(false),
            zombie_villager_villager_data: ZombieVillagerVillagerData(VillagerData {
                kind: azalea_registry::VillagerKind::Plains,
                profession: azalea_registry::VillagerProfession::None,
                level: 0,
            }),
        }
    }
}

#[derive(Component)]
pub struct ZombifiedPiglin;
impl ZombifiedPiglin {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=18 => Zombie::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct ZombifiedPiglinMetadataBundle {
    _marker: ZombifiedPiglin,
    parent: ZombieMetadataBundle,
}
impl Default for ZombifiedPiglinMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: ZombifiedPiglin,
            parent: ZombieMetadataBundle {
                _marker: Zombie,
                parent: AbstractMonsterMetadataBundle {
                    _marker: AbstractMonster,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                },
                zombie_baby: ZombieBaby(false),
                special_type: SpecialType(0),
                drowned_conversion: DrownedConversion(false),
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractAgeable;
impl AbstractAgeable {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            16 => {
                entity.insert(AbstractAgeableBaby(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractAgeableMetadataBundle {
    _marker: AbstractAgeable,
    parent: AbstractCreatureMetadataBundle,
    abstract_ageable_baby: AbstractAgeableBaby,
}
impl Default for AbstractAgeableMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractAgeable,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
            abstract_ageable_baby: AbstractAgeableBaby(false),
        }
    }
}

#[derive(Component)]
pub struct AbstractAnimal;
impl AbstractAnimal {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAgeable::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractAnimalMetadataBundle {
    _marker: AbstractAnimal,
    parent: AbstractAgeableMetadataBundle,
}
impl Default for AbstractAnimalMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractAnimal,
            parent: AbstractAgeableMetadataBundle {
                _marker: AbstractAgeable,
                parent: AbstractCreatureMetadataBundle {
                    _marker: AbstractCreature,
                    parent: AbstractInsentientMetadataBundle {
                        _marker: AbstractInsentient,
                        parent: AbstractLivingMetadataBundle {
                            _marker: AbstractLiving,
                            parent: AbstractEntityMetadataBundle {
                                _marker: AbstractEntity,
                                on_fire: OnFire(false),
                                shift_key_down: ShiftKeyDown(false),
                                sprinting: Sprinting(false),
                                swimming: Swimming(false),
                                currently_glowing: CurrentlyGlowing(false),
                                invisible: Invisible(false),
                                fall_flying: FallFlying(false),
                                air_supply: AirSupply(Default::default()),
                                custom_name: CustomName(None),
                                custom_name_visible: CustomNameVisible(false),
                                silent: Silent(false),
                                no_gravity: NoGravity(false),
                                pose: Pose::default(),
                                ticks_frozen: TicksFrozen(0),
                            },
                            auto_spin_attack: AutoSpinAttack(false),
                            abstract_living_using_item: AbstractLivingUsingItem(false),
                            health: Health(1.0),
                            abstract_living_effect_color: AbstractLivingEffectColor(0),
                            effect_ambience: EffectAmbience(false),
                            arrow_count: ArrowCount(0),
                            stinger_count: StingerCount(0),
                            sleeping_pos: SleepingPos(None),
                        },
                        no_ai: NoAi(false),
                        left_handed: LeftHanded(false),
                        aggressive: Aggressive(false),
                    },
                },
                abstract_ageable_baby: AbstractAgeableBaby(false),
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractCreature;
impl AbstractCreature {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractInsentient::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractCreatureMetadataBundle {
    _marker: AbstractCreature,
    parent: AbstractInsentientMetadataBundle,
}
impl Default for AbstractCreatureMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractCreature,
            parent: AbstractInsentientMetadataBundle {
                _marker: AbstractInsentient,
                parent: AbstractLivingMetadataBundle {
                    _marker: AbstractLiving,
                    parent: AbstractEntityMetadataBundle {
                        _marker: AbstractEntity,
                        on_fire: OnFire(false),
                        shift_key_down: ShiftKeyDown(false),
                        sprinting: Sprinting(false),
                        swimming: Swimming(false),
                        currently_glowing: CurrentlyGlowing(false),
                        invisible: Invisible(false),
                        fall_flying: FallFlying(false),
                        air_supply: AirSupply(Default::default()),
                        custom_name: CustomName(None),
                        custom_name_visible: CustomNameVisible(false),
                        silent: Silent(false),
                        no_gravity: NoGravity(false),
                        pose: Pose::default(),
                        ticks_frozen: TicksFrozen(0),
                    },
                    auto_spin_attack: AutoSpinAttack(false),
                    abstract_living_using_item: AbstractLivingUsingItem(false),
                    health: Health(1.0),
                    abstract_living_effect_color: AbstractLivingEffectColor(0),
                    effect_ambience: EffectAmbience(false),
                    arrow_count: ArrowCount(0),
                    stinger_count: StingerCount(0),
                    sleeping_pos: SleepingPos(None),
                },
                no_ai: NoAi(false),
                left_handed: LeftHanded(false),
                aggressive: Aggressive(false),
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractEntity;
impl AbstractEntity {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(OnFire(bitfield & 0x1 != 0));
                entity.insert(ShiftKeyDown(bitfield & 0x2 != 0));
                entity.insert(Sprinting(bitfield & 0x8 != 0));
                entity.insert(Swimming(bitfield & 0x10 != 0));
                entity.insert(CurrentlyGlowing(bitfield & 0x40 != 0));
                entity.insert(Invisible(bitfield & 0x20 != 0));
                entity.insert(FallFlying(bitfield & 0x80 != 0));
            }
            1 => {
                entity.insert(AirSupply(d.value.into_int()?));
            }
            2 => {
                entity.insert(CustomName(d.value.into_optional_formatted_text()?));
            }
            3 => {
                entity.insert(CustomNameVisible(d.value.into_boolean()?));
            }
            4 => {
                entity.insert(Silent(d.value.into_boolean()?));
            }
            5 => {
                entity.insert(NoGravity(d.value.into_boolean()?));
            }
            6 => {
                entity.insert(d.value.into_pose()?);
            }
            7 => {
                entity.insert(TicksFrozen(d.value.into_int()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractEntityMetadataBundle {
    _marker: AbstractEntity,
    on_fire: OnFire,
    shift_key_down: ShiftKeyDown,
    sprinting: Sprinting,
    swimming: Swimming,
    currently_glowing: CurrentlyGlowing,
    invisible: Invisible,
    fall_flying: FallFlying,
    air_supply: AirSupply,
    custom_name: CustomName,
    custom_name_visible: CustomNameVisible,
    silent: Silent,
    no_gravity: NoGravity,
    pose: Pose,
    ticks_frozen: TicksFrozen,
}
impl Default for AbstractEntityMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractEntity,
            on_fire: OnFire(false),
            shift_key_down: ShiftKeyDown(false),
            sprinting: Sprinting(false),
            swimming: Swimming(false),
            currently_glowing: CurrentlyGlowing(false),
            invisible: Invisible(false),
            fall_flying: FallFlying(false),
            air_supply: AirSupply(Default::default()),
            custom_name: CustomName(None),
            custom_name_visible: CustomNameVisible(false),
            silent: Silent(false),
            no_gravity: NoGravity(false),
            pose: Pose::default(),
            ticks_frozen: TicksFrozen(0),
        }
    }
}

#[derive(Component)]
pub struct AbstractInsentient;
impl AbstractInsentient {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=14 => AbstractLiving::apply_metadata(entity, d)?,
            15 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(NoAi(bitfield & 0x1 != 0));
                entity.insert(LeftHanded(bitfield & 0x2 != 0));
                entity.insert(Aggressive(bitfield & 0x4 != 0));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractInsentientMetadataBundle {
    _marker: AbstractInsentient,
    parent: AbstractLivingMetadataBundle,
    no_ai: NoAi,
    left_handed: LeftHanded,
    aggressive: Aggressive,
}
impl Default for AbstractInsentientMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractInsentient,
            parent: AbstractLivingMetadataBundle {
                _marker: AbstractLiving,
                parent: AbstractEntityMetadataBundle {
                    _marker: AbstractEntity,
                    on_fire: OnFire(false),
                    shift_key_down: ShiftKeyDown(false),
                    sprinting: Sprinting(false),
                    swimming: Swimming(false),
                    currently_glowing: CurrentlyGlowing(false),
                    invisible: Invisible(false),
                    fall_flying: FallFlying(false),
                    air_supply: AirSupply(Default::default()),
                    custom_name: CustomName(None),
                    custom_name_visible: CustomNameVisible(false),
                    silent: Silent(false),
                    no_gravity: NoGravity(false),
                    pose: Pose::default(),
                    ticks_frozen: TicksFrozen(0),
                },
                auto_spin_attack: AutoSpinAttack(false),
                abstract_living_using_item: AbstractLivingUsingItem(false),
                health: Health(1.0),
                abstract_living_effect_color: AbstractLivingEffectColor(0),
                effect_ambience: EffectAmbience(false),
                arrow_count: ArrowCount(0),
                stinger_count: StingerCount(0),
                sleeping_pos: SleepingPos(None),
            },
            no_ai: NoAi(false),
            left_handed: LeftHanded(false),
            aggressive: Aggressive(false),
        }
    }
}

#[derive(Component)]
pub struct AbstractLiving;
impl AbstractLiving {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(AutoSpinAttack(bitfield & 0x4 != 0));
                entity.insert(AbstractLivingUsingItem(bitfield & 0x1 != 0));
            }
            9 => {
                entity.insert(Health(d.value.into_float()?));
            }
            10 => {
                entity.insert(AbstractLivingEffectColor(d.value.into_int()?));
            }
            11 => {
                entity.insert(EffectAmbience(d.value.into_boolean()?));
            }
            12 => {
                entity.insert(ArrowCount(d.value.into_int()?));
            }
            13 => {
                entity.insert(StingerCount(d.value.into_int()?));
            }
            14 => {
                entity.insert(SleepingPos(d.value.into_optional_block_pos()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractLivingMetadataBundle {
    _marker: AbstractLiving,
    parent: AbstractEntityMetadataBundle,
    auto_spin_attack: AutoSpinAttack,
    abstract_living_using_item: AbstractLivingUsingItem,
    health: Health,
    abstract_living_effect_color: AbstractLivingEffectColor,
    effect_ambience: EffectAmbience,
    arrow_count: ArrowCount,
    stinger_count: StingerCount,
    sleeping_pos: SleepingPos,
}
impl Default for AbstractLivingMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractLiving,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            auto_spin_attack: AutoSpinAttack(false),
            abstract_living_using_item: AbstractLivingUsingItem(false),
            health: Health(1.0),
            abstract_living_effect_color: AbstractLivingEffectColor(0),
            effect_ambience: EffectAmbience(false),
            arrow_count: ArrowCount(0),
            stinger_count: StingerCount(0),
            sleeping_pos: SleepingPos(None),
        }
    }
}

#[derive(Component)]
pub struct AbstractMinecart;
impl AbstractMinecart {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=7 => AbstractEntity::apply_metadata(entity, d)?,
            8 => {
                entity.insert(AbstractMinecartHurt(d.value.into_int()?));
            }
            9 => {
                entity.insert(AbstractMinecartHurtdir(d.value.into_int()?));
            }
            10 => {
                entity.insert(AbstractMinecartDamage(d.value.into_float()?));
            }
            11 => {
                entity.insert(DisplayBlock(d.value.into_int()?));
            }
            12 => {
                entity.insert(DisplayOffset(d.value.into_int()?));
            }
            13 => {
                entity.insert(CustomDisplay(d.value.into_boolean()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractMinecartMetadataBundle {
    _marker: AbstractMinecart,
    parent: AbstractEntityMetadataBundle,
    abstract_minecart_hurt: AbstractMinecartHurt,
    abstract_minecart_hurtdir: AbstractMinecartHurtdir,
    abstract_minecart_damage: AbstractMinecartDamage,
    display_block: DisplayBlock,
    display_offset: DisplayOffset,
    custom_display: CustomDisplay,
}
impl Default for AbstractMinecartMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractMinecart,
            parent: AbstractEntityMetadataBundle {
                _marker: AbstractEntity,
                on_fire: OnFire(false),
                shift_key_down: ShiftKeyDown(false),
                sprinting: Sprinting(false),
                swimming: Swimming(false),
                currently_glowing: CurrentlyGlowing(false),
                invisible: Invisible(false),
                fall_flying: FallFlying(false),
                air_supply: AirSupply(Default::default()),
                custom_name: CustomName(None),
                custom_name_visible: CustomNameVisible(false),
                silent: Silent(false),
                no_gravity: NoGravity(false),
                pose: Pose::default(),
                ticks_frozen: TicksFrozen(0),
            },
            abstract_minecart_hurt: AbstractMinecartHurt(0),
            abstract_minecart_hurtdir: AbstractMinecartHurtdir(1),
            abstract_minecart_damage: AbstractMinecartDamage(0.0),
            display_block: DisplayBlock(Default::default()),
            display_offset: DisplayOffset(6),
            custom_display: CustomDisplay(false),
        }
    }
}

#[derive(Component)]
pub struct AbstractMonster;
impl AbstractMonster {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=15 => AbstractCreature::apply_metadata(entity, d)?,
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractMonsterMetadataBundle {
    _marker: AbstractMonster,
    parent: AbstractCreatureMetadataBundle,
}
impl Default for AbstractMonsterMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractMonster,
            parent: AbstractCreatureMetadataBundle {
                _marker: AbstractCreature,
                parent: AbstractInsentientMetadataBundle {
                    _marker: AbstractInsentient,
                    parent: AbstractLivingMetadataBundle {
                        _marker: AbstractLiving,
                        parent: AbstractEntityMetadataBundle {
                            _marker: AbstractEntity,
                            on_fire: OnFire(false),
                            shift_key_down: ShiftKeyDown(false),
                            sprinting: Sprinting(false),
                            swimming: Swimming(false),
                            currently_glowing: CurrentlyGlowing(false),
                            invisible: Invisible(false),
                            fall_flying: FallFlying(false),
                            air_supply: AirSupply(Default::default()),
                            custom_name: CustomName(None),
                            custom_name_visible: CustomNameVisible(false),
                            silent: Silent(false),
                            no_gravity: NoGravity(false),
                            pose: Pose::default(),
                            ticks_frozen: TicksFrozen(0),
                        },
                        auto_spin_attack: AutoSpinAttack(false),
                        abstract_living_using_item: AbstractLivingUsingItem(false),
                        health: Health(1.0),
                        abstract_living_effect_color: AbstractLivingEffectColor(0),
                        effect_ambience: EffectAmbience(false),
                        arrow_count: ArrowCount(0),
                        stinger_count: StingerCount(0),
                        sleeping_pos: SleepingPos(None),
                    },
                    no_ai: NoAi(false),
                    left_handed: LeftHanded(false),
                    aggressive: Aggressive(false),
                },
            },
        }
    }
}

#[derive(Component)]
pub struct AbstractTameable;
impl AbstractTameable {
    pub fn apply_metadata(
        entity: &mut bevy_ecs::system::EntityCommands,
        d: EntityDataItem,
    ) -> Result<(), UpdateMetadataError> {
        match d.index {
            0..=16 => AbstractAnimal::apply_metadata(entity, d)?,
            17 => {
                let bitfield = d.value.into_byte()?;
                entity.insert(Tame(bitfield & 0x4 != 0));
                entity.insert(InSittingPose(bitfield & 0x1 != 0));
            }
            18 => {
                entity.insert(Owneruuid(d.value.into_optional_uuid()?));
            }
            _ => {}
        }
        Ok(())
    }
}

#[derive(Bundle)]
pub struct AbstractTameableMetadataBundle {
    _marker: AbstractTameable,
    parent: AbstractAnimalMetadataBundle,
    tame: Tame,
    in_sitting_pose: InSittingPose,
    owneruuid: Owneruuid,
}
impl Default for AbstractTameableMetadataBundle {
    fn default() -> Self {
        Self {
            _marker: AbstractTameable,
            parent: AbstractAnimalMetadataBundle {
                _marker: AbstractAnimal,
                parent: AbstractAgeableMetadataBundle {
                    _marker: AbstractAgeable,
                    parent: AbstractCreatureMetadataBundle {
                        _marker: AbstractCreature,
                        parent: AbstractInsentientMetadataBundle {
                            _marker: AbstractInsentient,
                            parent: AbstractLivingMetadataBundle {
                                _marker: AbstractLiving,
                                parent: AbstractEntityMetadataBundle {
                                    _marker: AbstractEntity,
                                    on_fire: OnFire(false),
                                    shift_key_down: ShiftKeyDown(false),
                                    sprinting: Sprinting(false),
                                    swimming: Swimming(false),
                                    currently_glowing: CurrentlyGlowing(false),
                                    invisible: Invisible(false),
                                    fall_flying: FallFlying(false),
                                    air_supply: AirSupply(Default::default()),
                                    custom_name: CustomName(None),
                                    custom_name_visible: CustomNameVisible(false),
                                    silent: Silent(false),
                                    no_gravity: NoGravity(false),
                                    pose: Pose::default(),
                                    ticks_frozen: TicksFrozen(0),
                                },
                                auto_spin_attack: AutoSpinAttack(false),
                                abstract_living_using_item: AbstractLivingUsingItem(false),
                                health: Health(1.0),
                                abstract_living_effect_color: AbstractLivingEffectColor(0),
                                effect_ambience: EffectAmbience(false),
                                arrow_count: ArrowCount(0),
                                stinger_count: StingerCount(0),
                                sleeping_pos: SleepingPos(None),
                            },
                            no_ai: NoAi(false),
                            left_handed: LeftHanded(false),
                            aggressive: Aggressive(false),
                        },
                    },
                    abstract_ageable_baby: AbstractAgeableBaby(false),
                },
            },
            tame: Tame(false),
            in_sitting_pose: InSittingPose(false),
            owneruuid: Owneruuid(None),
        }
    }
}

pub fn apply_metadata(
    entity: &mut bevy_ecs::system::EntityCommands,
    entity_kind: azalea_registry::EntityKind,
    items: Vec<EntityDataItem>,
) -> Result<(), UpdateMetadataError> {
    match entity_kind {
        azalea_registry::EntityKind::Allay => {
            for d in items {
                Allay::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::AreaEffectCloud => {
            for d in items {
                AreaEffectCloud::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ArmorStand => {
            for d in items {
                ArmorStand::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Arrow => {
            for d in items {
                Arrow::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Axolotl => {
            for d in items {
                Axolotl::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Bat => {
            for d in items {
                Bat::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Bee => {
            for d in items {
                Bee::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Blaze => {
            for d in items {
                Blaze::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::BlockDisplay => {
            for d in items {
                BlockDisplay::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Boat => {
            for d in items {
                Boat::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Camel => {
            for d in items {
                Camel::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Cat => {
            for d in items {
                Cat::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::CaveSpider => {
            for d in items {
                CaveSpider::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ChestBoat => {
            for d in items {
                ChestBoat::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ChestMinecart => {
            for d in items {
                ChestMinecart::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Chicken => {
            for d in items {
                Chicken::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Cod => {
            for d in items {
                Cod::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::CommandBlockMinecart => {
            for d in items {
                CommandBlockMinecart::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Cow => {
            for d in items {
                Cow::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Creeper => {
            for d in items {
                Creeper::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Dolphin => {
            for d in items {
                Dolphin::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Donkey => {
            for d in items {
                Donkey::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::DragonFireball => {
            for d in items {
                DragonFireball::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Drowned => {
            for d in items {
                Drowned::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Egg => {
            for d in items {
                Egg::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ElderGuardian => {
            for d in items {
                ElderGuardian::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::EndCrystal => {
            for d in items {
                EndCrystal::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::EnderDragon => {
            for d in items {
                EnderDragon::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::EnderPearl => {
            for d in items {
                EnderPearl::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Enderman => {
            for d in items {
                Enderman::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Endermite => {
            for d in items {
                Endermite::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Evoker => {
            for d in items {
                Evoker::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::EvokerFangs => {
            for d in items {
                EvokerFangs::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ExperienceBottle => {
            for d in items {
                ExperienceBottle::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ExperienceOrb => {
            for d in items {
                ExperienceOrb::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::EyeOfEnder => {
            for d in items {
                EyeOfEnder::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::FallingBlock => {
            for d in items {
                FallingBlock::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Fireball => {
            for d in items {
                Fireball::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::FireworkRocket => {
            for d in items {
                FireworkRocket::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::FishingBobber => {
            for d in items {
                FishingBobber::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Fox => {
            for d in items {
                Fox::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Frog => {
            for d in items {
                Frog::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::FurnaceMinecart => {
            for d in items {
                FurnaceMinecart::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Ghast => {
            for d in items {
                Ghast::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Giant => {
            for d in items {
                Giant::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::GlowItemFrame => {
            for d in items {
                GlowItemFrame::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::GlowSquid => {
            for d in items {
                GlowSquid::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Goat => {
            for d in items {
                Goat::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Guardian => {
            for d in items {
                Guardian::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Hoglin => {
            for d in items {
                Hoglin::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::HopperMinecart => {
            for d in items {
                HopperMinecart::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Horse => {
            for d in items {
                Horse::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Husk => {
            for d in items {
                Husk::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Illusioner => {
            for d in items {
                Illusioner::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Interaction => {
            for d in items {
                Interaction::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::IronGolem => {
            for d in items {
                IronGolem::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Item => {
            for d in items {
                Item::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ItemDisplay => {
            for d in items {
                ItemDisplay::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ItemFrame => {
            for d in items {
                ItemFrame::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::LeashKnot => {
            for d in items {
                LeashKnot::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::LightningBolt => {
            for d in items {
                LightningBolt::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Llama => {
            for d in items {
                Llama::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::LlamaSpit => {
            for d in items {
                LlamaSpit::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::MagmaCube => {
            for d in items {
                MagmaCube::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Marker => {
            for d in items {
                Marker::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Minecart => {
            for d in items {
                Minecart::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Mooshroom => {
            for d in items {
                Mooshroom::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Mule => {
            for d in items {
                Mule::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Ocelot => {
            for d in items {
                Ocelot::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Painting => {
            for d in items {
                Painting::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Panda => {
            for d in items {
                Panda::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Parrot => {
            for d in items {
                Parrot::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Phantom => {
            for d in items {
                Phantom::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Pig => {
            for d in items {
                Pig::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Piglin => {
            for d in items {
                Piglin::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::PiglinBrute => {
            for d in items {
                PiglinBrute::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Pillager => {
            for d in items {
                Pillager::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Player => {
            for d in items {
                Player::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::PolarBear => {
            for d in items {
                PolarBear::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Potion => {
            for d in items {
                Potion::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Pufferfish => {
            for d in items {
                Pufferfish::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Rabbit => {
            for d in items {
                Rabbit::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Ravager => {
            for d in items {
                Ravager::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Salmon => {
            for d in items {
                Salmon::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Sheep => {
            for d in items {
                Sheep::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Shulker => {
            for d in items {
                Shulker::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ShulkerBullet => {
            for d in items {
                ShulkerBullet::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Silverfish => {
            for d in items {
                Silverfish::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Skeleton => {
            for d in items {
                Skeleton::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::SkeletonHorse => {
            for d in items {
                SkeletonHorse::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Slime => {
            for d in items {
                Slime::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::SmallFireball => {
            for d in items {
                SmallFireball::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Sniffer => {
            for d in items {
                Sniffer::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::SnowGolem => {
            for d in items {
                SnowGolem::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Snowball => {
            for d in items {
                Snowball::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::SpawnerMinecart => {
            for d in items {
                SpawnerMinecart::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::SpectralArrow => {
            for d in items {
                SpectralArrow::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Spider => {
            for d in items {
                Spider::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Squid => {
            for d in items {
                Squid::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Stray => {
            for d in items {
                Stray::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Strider => {
            for d in items {
                Strider::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Tadpole => {
            for d in items {
                Tadpole::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::TextDisplay => {
            for d in items {
                TextDisplay::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Tnt => {
            for d in items {
                Tnt::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::TntMinecart => {
            for d in items {
                TntMinecart::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::TraderLlama => {
            for d in items {
                TraderLlama::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Trident => {
            for d in items {
                Trident::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::TropicalFish => {
            for d in items {
                TropicalFish::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Turtle => {
            for d in items {
                Turtle::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Vex => {
            for d in items {
                Vex::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Villager => {
            for d in items {
                Villager::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Vindicator => {
            for d in items {
                Vindicator::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::WanderingTrader => {
            for d in items {
                WanderingTrader::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Warden => {
            for d in items {
                Warden::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Witch => {
            for d in items {
                Witch::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Wither => {
            for d in items {
                Wither::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::WitherSkeleton => {
            for d in items {
                WitherSkeleton::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::WitherSkull => {
            for d in items {
                WitherSkull::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Wolf => {
            for d in items {
                Wolf::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Zoglin => {
            for d in items {
                Zoglin::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::Zombie => {
            for d in items {
                Zombie::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ZombieHorse => {
            for d in items {
                ZombieHorse::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ZombieVillager => {
            for d in items {
                ZombieVillager::apply_metadata(entity, d)?;
            }
        }
        azalea_registry::EntityKind::ZombifiedPiglin => {
            for d in items {
                ZombifiedPiglin::apply_metadata(entity, d)?;
            }
        }
    }
    Ok(())
}

pub fn apply_default_metadata(
    entity: &mut bevy_ecs::system::EntityCommands,
    kind: azalea_registry::EntityKind,
) {
    match kind {
        azalea_registry::EntityKind::Allay => {
            entity.insert(AllayMetadataBundle::default());
        }
        azalea_registry::EntityKind::AreaEffectCloud => {
            entity.insert(AreaEffectCloudMetadataBundle::default());
        }
        azalea_registry::EntityKind::ArmorStand => {
            entity.insert(ArmorStandMetadataBundle::default());
        }
        azalea_registry::EntityKind::Arrow => {
            entity.insert(ArrowMetadataBundle::default());
        }
        azalea_registry::EntityKind::Axolotl => {
            entity.insert(AxolotlMetadataBundle::default());
        }
        azalea_registry::EntityKind::Bat => {
            entity.insert(BatMetadataBundle::default());
        }
        azalea_registry::EntityKind::Bee => {
            entity.insert(BeeMetadataBundle::default());
        }
        azalea_registry::EntityKind::Blaze => {
            entity.insert(BlazeMetadataBundle::default());
        }
        azalea_registry::EntityKind::BlockDisplay => {
            entity.insert(BlockDisplayMetadataBundle::default());
        }
        azalea_registry::EntityKind::Boat => {
            entity.insert(BoatMetadataBundle::default());
        }
        azalea_registry::EntityKind::Camel => {
            entity.insert(CamelMetadataBundle::default());
        }
        azalea_registry::EntityKind::Cat => {
            entity.insert(CatMetadataBundle::default());
        }
        azalea_registry::EntityKind::CaveSpider => {
            entity.insert(CaveSpiderMetadataBundle::default());
        }
        azalea_registry::EntityKind::ChestBoat => {
            entity.insert(ChestBoatMetadataBundle::default());
        }
        azalea_registry::EntityKind::ChestMinecart => {
            entity.insert(ChestMinecartMetadataBundle::default());
        }
        azalea_registry::EntityKind::Chicken => {
            entity.insert(ChickenMetadataBundle::default());
        }
        azalea_registry::EntityKind::Cod => {
            entity.insert(CodMetadataBundle::default());
        }
        azalea_registry::EntityKind::CommandBlockMinecart => {
            entity.insert(CommandBlockMinecartMetadataBundle::default());
        }
        azalea_registry::EntityKind::Cow => {
            entity.insert(CowMetadataBundle::default());
        }
        azalea_registry::EntityKind::Creeper => {
            entity.insert(CreeperMetadataBundle::default());
        }
        azalea_registry::EntityKind::Dolphin => {
            entity.insert(DolphinMetadataBundle::default());
        }
        azalea_registry::EntityKind::Donkey => {
            entity.insert(DonkeyMetadataBundle::default());
        }
        azalea_registry::EntityKind::DragonFireball => {
            entity.insert(DragonFireballMetadataBundle::default());
        }
        azalea_registry::EntityKind::Drowned => {
            entity.insert(DrownedMetadataBundle::default());
        }
        azalea_registry::EntityKind::Egg => {
            entity.insert(EggMetadataBundle::default());
        }
        azalea_registry::EntityKind::ElderGuardian => {
            entity.insert(ElderGuardianMetadataBundle::default());
        }
        azalea_registry::EntityKind::EndCrystal => {
            entity.insert(EndCrystalMetadataBundle::default());
        }
        azalea_registry::EntityKind::EnderDragon => {
            entity.insert(EnderDragonMetadataBundle::default());
        }
        azalea_registry::EntityKind::EnderPearl => {
            entity.insert(EnderPearlMetadataBundle::default());
        }
        azalea_registry::EntityKind::Enderman => {
            entity.insert(EndermanMetadataBundle::default());
        }
        azalea_registry::EntityKind::Endermite => {
            entity.insert(EndermiteMetadataBundle::default());
        }
        azalea_registry::EntityKind::Evoker => {
            entity.insert(EvokerMetadataBundle::default());
        }
        azalea_registry::EntityKind::EvokerFangs => {
            entity.insert(EvokerFangsMetadataBundle::default());
        }
        azalea_registry::EntityKind::ExperienceBottle => {
            entity.insert(ExperienceBottleMetadataBundle::default());
        }
        azalea_registry::EntityKind::ExperienceOrb => {
            entity.insert(ExperienceOrbMetadataBundle::default());
        }
        azalea_registry::EntityKind::EyeOfEnder => {
            entity.insert(EyeOfEnderMetadataBundle::default());
        }
        azalea_registry::EntityKind::FallingBlock => {
            entity.insert(FallingBlockMetadataBundle::default());
        }
        azalea_registry::EntityKind::Fireball => {
            entity.insert(FireballMetadataBundle::default());
        }
        azalea_registry::EntityKind::FireworkRocket => {
            entity.insert(FireworkRocketMetadataBundle::default());
        }
        azalea_registry::EntityKind::FishingBobber => {
            entity.insert(FishingBobberMetadataBundle::default());
        }
        azalea_registry::EntityKind::Fox => {
            entity.insert(FoxMetadataBundle::default());
        }
        azalea_registry::EntityKind::Frog => {
            entity.insert(FrogMetadataBundle::default());
        }
        azalea_registry::EntityKind::FurnaceMinecart => {
            entity.insert(FurnaceMinecartMetadataBundle::default());
        }
        azalea_registry::EntityKind::Ghast => {
            entity.insert(GhastMetadataBundle::default());
        }
        azalea_registry::EntityKind::Giant => {
            entity.insert(GiantMetadataBundle::default());
        }
        azalea_registry::EntityKind::GlowItemFrame => {
            entity.insert(GlowItemFrameMetadataBundle::default());
        }
        azalea_registry::EntityKind::GlowSquid => {
            entity.insert(GlowSquidMetadataBundle::default());
        }
        azalea_registry::EntityKind::Goat => {
            entity.insert(GoatMetadataBundle::default());
        }
        azalea_registry::EntityKind::Guardian => {
            entity.insert(GuardianMetadataBundle::default());
        }
        azalea_registry::EntityKind::Hoglin => {
            entity.insert(HoglinMetadataBundle::default());
        }
        azalea_registry::EntityKind::HopperMinecart => {
            entity.insert(HopperMinecartMetadataBundle::default());
        }
        azalea_registry::EntityKind::Horse => {
            entity.insert(HorseMetadataBundle::default());
        }
        azalea_registry::EntityKind::Husk => {
            entity.insert(HuskMetadataBundle::default());
        }
        azalea_registry::EntityKind::Illusioner => {
            entity.insert(IllusionerMetadataBundle::default());
        }
        azalea_registry::EntityKind::Interaction => {
            entity.insert(InteractionMetadataBundle::default());
        }
        azalea_registry::EntityKind::IronGolem => {
            entity.insert(IronGolemMetadataBundle::default());
        }
        azalea_registry::EntityKind::Item => {
            entity.insert(ItemMetadataBundle::default());
        }
        azalea_registry::EntityKind::ItemDisplay => {
            entity.insert(ItemDisplayMetadataBundle::default());
        }
        azalea_registry::EntityKind::ItemFrame => {
            entity.insert(ItemFrameMetadataBundle::default());
        }
        azalea_registry::EntityKind::LeashKnot => {
            entity.insert(LeashKnotMetadataBundle::default());
        }
        azalea_registry::EntityKind::LightningBolt => {
            entity.insert(LightningBoltMetadataBundle::default());
        }
        azalea_registry::EntityKind::Llama => {
            entity.insert(LlamaMetadataBundle::default());
        }
        azalea_registry::EntityKind::LlamaSpit => {
            entity.insert(LlamaSpitMetadataBundle::default());
        }
        azalea_registry::EntityKind::MagmaCube => {
            entity.insert(MagmaCubeMetadataBundle::default());
        }
        azalea_registry::EntityKind::Marker => {
            entity.insert(MarkerMetadataBundle::default());
        }
        azalea_registry::EntityKind::Minecart => {
            entity.insert(MinecartMetadataBundle::default());
        }
        azalea_registry::EntityKind::Mooshroom => {
            entity.insert(MooshroomMetadataBundle::default());
        }
        azalea_registry::EntityKind::Mule => {
            entity.insert(MuleMetadataBundle::default());
        }
        azalea_registry::EntityKind::Ocelot => {
            entity.insert(OcelotMetadataBundle::default());
        }
        azalea_registry::EntityKind::Painting => {
            entity.insert(PaintingMetadataBundle::default());
        }
        azalea_registry::EntityKind::Panda => {
            entity.insert(PandaMetadataBundle::default());
        }
        azalea_registry::EntityKind::Parrot => {
            entity.insert(ParrotMetadataBundle::default());
        }
        azalea_registry::EntityKind::Phantom => {
            entity.insert(PhantomMetadataBundle::default());
        }
        azalea_registry::EntityKind::Pig => {
            entity.insert(PigMetadataBundle::default());
        }
        azalea_registry::EntityKind::Piglin => {
            entity.insert(PiglinMetadataBundle::default());
        }
        azalea_registry::EntityKind::PiglinBrute => {
            entity.insert(PiglinBruteMetadataBundle::default());
        }
        azalea_registry::EntityKind::Pillager => {
            entity.insert(PillagerMetadataBundle::default());
        }
        azalea_registry::EntityKind::Player => {
            entity.insert(PlayerMetadataBundle::default());
        }
        azalea_registry::EntityKind::PolarBear => {
            entity.insert(PolarBearMetadataBundle::default());
        }
        azalea_registry::EntityKind::Potion => {
            entity.insert(PotionMetadataBundle::default());
        }
        azalea_registry::EntityKind::Pufferfish => {
            entity.insert(PufferfishMetadataBundle::default());
        }
        azalea_registry::EntityKind::Rabbit => {
            entity.insert(RabbitMetadataBundle::default());
        }
        azalea_registry::EntityKind::Ravager => {
            entity.insert(RavagerMetadataBundle::default());
        }
        azalea_registry::EntityKind::Salmon => {
            entity.insert(SalmonMetadataBundle::default());
        }
        azalea_registry::EntityKind::Sheep => {
            entity.insert(SheepMetadataBundle::default());
        }
        azalea_registry::EntityKind::Shulker => {
            entity.insert(ShulkerMetadataBundle::default());
        }
        azalea_registry::EntityKind::ShulkerBullet => {
            entity.insert(ShulkerBulletMetadataBundle::default());
        }
        azalea_registry::EntityKind::Silverfish => {
            entity.insert(SilverfishMetadataBundle::default());
        }
        azalea_registry::EntityKind::Skeleton => {
            entity.insert(SkeletonMetadataBundle::default());
        }
        azalea_registry::EntityKind::SkeletonHorse => {
            entity.insert(SkeletonHorseMetadataBundle::default());
        }
        azalea_registry::EntityKind::Slime => {
            entity.insert(SlimeMetadataBundle::default());
        }
        azalea_registry::EntityKind::SmallFireball => {
            entity.insert(SmallFireballMetadataBundle::default());
        }
        azalea_registry::EntityKind::Sniffer => {
            entity.insert(SnifferMetadataBundle::default());
        }
        azalea_registry::EntityKind::SnowGolem => {
            entity.insert(SnowGolemMetadataBundle::default());
        }
        azalea_registry::EntityKind::Snowball => {
            entity.insert(SnowballMetadataBundle::default());
        }
        azalea_registry::EntityKind::SpawnerMinecart => {
            entity.insert(SpawnerMinecartMetadataBundle::default());
        }
        azalea_registry::EntityKind::SpectralArrow => {
            entity.insert(SpectralArrowMetadataBundle::default());
        }
        azalea_registry::EntityKind::Spider => {
            entity.insert(SpiderMetadataBundle::default());
        }
        azalea_registry::EntityKind::Squid => {
            entity.insert(SquidMetadataBundle::default());
        }
        azalea_registry::EntityKind::Stray => {
            entity.insert(StrayMetadataBundle::default());
        }
        azalea_registry::EntityKind::Strider => {
            entity.insert(StriderMetadataBundle::default());
        }
        azalea_registry::EntityKind::Tadpole => {
            entity.insert(TadpoleMetadataBundle::default());
        }
        azalea_registry::EntityKind::TextDisplay => {
            entity.insert(TextDisplayMetadataBundle::default());
        }
        azalea_registry::EntityKind::Tnt => {
            entity.insert(TntMetadataBundle::default());
        }
        azalea_registry::EntityKind::TntMinecart => {
            entity.insert(TntMinecartMetadataBundle::default());
        }
        azalea_registry::EntityKind::TraderLlama => {
            entity.insert(TraderLlamaMetadataBundle::default());
        }
        azalea_registry::EntityKind::Trident => {
            entity.insert(TridentMetadataBundle::default());
        }
        azalea_registry::EntityKind::TropicalFish => {
            entity.insert(TropicalFishMetadataBundle::default());
        }
        azalea_registry::EntityKind::Turtle => {
            entity.insert(TurtleMetadataBundle::default());
        }
        azalea_registry::EntityKind::Vex => {
            entity.insert(VexMetadataBundle::default());
        }
        azalea_registry::EntityKind::Villager => {
            entity.insert(VillagerMetadataBundle::default());
        }
        azalea_registry::EntityKind::Vindicator => {
            entity.insert(VindicatorMetadataBundle::default());
        }
        azalea_registry::EntityKind::WanderingTrader => {
            entity.insert(WanderingTraderMetadataBundle::default());
        }
        azalea_registry::EntityKind::Warden => {
            entity.insert(WardenMetadataBundle::default());
        }
        azalea_registry::EntityKind::Witch => {
            entity.insert(WitchMetadataBundle::default());
        }
        azalea_registry::EntityKind::Wither => {
            entity.insert(WitherMetadataBundle::default());
        }
        azalea_registry::EntityKind::WitherSkeleton => {
            entity.insert(WitherSkeletonMetadataBundle::default());
        }
        azalea_registry::EntityKind::WitherSkull => {
            entity.insert(WitherSkullMetadataBundle::default());
        }
        azalea_registry::EntityKind::Wolf => {
            entity.insert(WolfMetadataBundle::default());
        }
        azalea_registry::EntityKind::Zoglin => {
            entity.insert(ZoglinMetadataBundle::default());
        }
        azalea_registry::EntityKind::Zombie => {
            entity.insert(ZombieMetadataBundle::default());
        }
        azalea_registry::EntityKind::ZombieHorse => {
            entity.insert(ZombieHorseMetadataBundle::default());
        }
        azalea_registry::EntityKind::ZombieVillager => {
            entity.insert(ZombieVillagerMetadataBundle::default());
        }
        azalea_registry::EntityKind::ZombifiedPiglin => {
            entity.insert(ZombifiedPiglinMetadataBundle::default());
        }
    }
}
