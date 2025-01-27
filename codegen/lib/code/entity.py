from lib.utils import to_camel_case, to_snake_case, get_dir_location, upper_first_letter
from lib.code.packet import burger_instruction_to_code
from lib.code.utils import burger_type_to_rust_type
from lib.mappings import Mappings
from typing import Optional
import re

METADATA_RS_DIR = get_dir_location(
    '../azalea-entity/src/metadata.rs')

DATA_RS_DIR = get_dir_location(
    '../azalea-entity/src/data.rs')

DIMENSIONS_RS_DIR = get_dir_location(
    '../azalea-entity/src/dimensions.rs')

def generate_metadata_names(burger_dataserializers: dict, mappings: Mappings):
    serializer_names: list[Optional[str]] = [None] * len(burger_dataserializers)
    for burger_serializer in burger_dataserializers.values():
        print(burger_serializer)

        # burger gives us the wrong class, so we do this instead
        data_serializers_class = mappings.get_class_from_deobfuscated_name('net.minecraft.network.syncher.EntityDataSerializers')
        mojmap_name = mappings.get_field(data_serializers_class, burger_serializer['field']).lower()

        if mojmap_name == 'component':
            mojmap_name = 'formatted_text'
        elif mojmap_name == 'optional_component':
            mojmap_name = 'optional_formatted_text'

        serializer_names[burger_serializer['id']] = upper_first_letter(to_camel_case(mojmap_name))
    return serializer_names

def parse_metadata_types_from_code():
    with open(DATA_RS_DIR, 'r') as f:
        lines = f.read().splitlines()
    
    data = []

    in_enum = False
    for line in lines:
        if line == 'pub enum EntityDataValue {':
            in_enum = True
        elif line == '}':
            in_enum = False
        elif in_enum:
            line = line.strip()
            if line.startswith('//'): continue
            name, type = line.rstrip('),').split('(')
            is_var = False
            if type.startswith('#[var] '):
                is_var = True
                type = type[len('#[var] '):]
            data.append({
                'name': name,
                'type': type,
                'var': is_var
            })
    print(data)
    return data

def generate_entity_metadata(burger_entities_data: dict, mappings: Mappings):
    burger_entity_metadata = burger_entities_data['entity']

    new_metadata_names = generate_metadata_names(burger_entities_data['dataserializers'], mappings)
    parsed_metadata_types = parse_metadata_types_from_code()

    parsed_metadata_names = []
    for t in parsed_metadata_types:
        parsed_metadata_names.append(t['name'])

    with open(DATA_RS_DIR, 'r') as f:
        lines = f.read().splitlines()
    # add the metadata names that weren't there before to the end of the enum.
    # this technically might cause them to be in the wrong order but i decided
    # making it correct while preserving comments was too annoying so i didn't
    added_metadata_names = []
    for n in new_metadata_names:
        if n not in parsed_metadata_names:
            added_metadata_names.append(n)
    if added_metadata_names != []:
        in_enum = False
        for i, line in enumerate(list(lines)):
            if line == 'pub enum EntityDataValue {':
                in_enum = True
            elif in_enum and line == '}':
                in_enum = False
                for n in added_metadata_names:
                    lines.insert(i, f'{n}(TODO),')
                    i += 1
        print(lines)
        with open(DATA_RS_DIR, 'w') as f:
            f.write('\n'.join(lines))
        print('Expected metadata types:\n' + '\n'.join(new_metadata_names))
        print('Updated metadata types in azalea-entity/src/data.rs, go make sure they\'re correct (check EntityDataSerializers.java) and then press enter')
        input()
    
    metadata_types = parse_metadata_types_from_code()

    code = []
    code.append('''#![allow(clippy::single_match)]

// This file is generated from codegen/lib/code/entity.py.
// Don't change it manually!

use crate::particle::Particle;

use super::{
    ArmadilloStateKind, EntityDataItem, EntityDataValue, OptionalUnsignedInt, Pose, Quaternion,
    Rotations, SnifferStateKind, VillagerData,
};
use azalea_chat::FormattedText;
use azalea_core::{
    direction::Direction,
    position::{BlockPos, Vec3},
};
use azalea_inventory::ItemStack;
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
''')

    # types that are only ever used in one entity
    single_use_imported_types = {'particle', 'pose'}

    added_metadata_fields = set()

    # a dict of { entity_id: { field_name: new_name } }
    field_name_map = {}

    # build the duplicate_field_names set
    previous_field_names = set()
    duplicate_field_names = set()

    # some generic names... we don't like these
    duplicate_field_names.add('state') # SnifferState instead of State

    for entity_id in burger_entity_metadata.keys():
        field_name_map[entity_id] = {}
        for field_name_or_bitfield in get_entity_metadata_names(entity_id, burger_entity_metadata, mappings).values():
            if isinstance(field_name_or_bitfield, str):
                if field_name_or_bitfield in previous_field_names:
                    duplicate_field_names.add(field_name_or_bitfield)
                else:
                    previous_field_names.add(field_name_or_bitfield)
            else:
                for mask, name in field_name_or_bitfield.items():
                    if name in previous_field_names:
                        duplicate_field_names.add(name)
                    else:
                        previous_field_names.add(name)

        # oh and also just add the entity id to the duplicate field names to
        # make sure entity names don't clash with field names
        duplicate_field_names.add(entity_id)

    # make sure these types are only ever made once
    for name in single_use_imported_types:
        if name in duplicate_field_names:
            raise Exception(f'{name} should only exist once')

    # and now figure out what to rename them to
    for entity_id in burger_entity_metadata.keys():
        for index, field_name_or_bitfield in get_entity_metadata_names(entity_id, burger_entity_metadata, mappings).items():
            if isinstance(field_name_or_bitfield, str):
                new_field_name = field_name_or_bitfield
                if new_field_name == 'type':
                    new_field_name = 'kind'
                if field_name_or_bitfield in duplicate_field_names:
                    field_name_map[entity_id][
                        field_name_or_bitfield] = f'{entity_id.strip("~")}_{new_field_name}'
            else:
                for mask, name in field_name_or_bitfield.items():
                    new_field_name = name
                    if new_field_name == 'type':
                        new_field_name = 'kind'
                    if name in duplicate_field_names:
                        field_name_map[entity_id][name] = f'{entity_id.strip("~")}_{new_field_name}'

    def new_entity(entity_id: str):
        # note: fields are components

        # if it doesn't start with ~ then also make a marker struct and Query struct for it
        all_field_names_or_bitfields = []
        entity_ids_for_all_field_names_or_bitfields = []
        entity_metadatas = []

        def maybe_rename_field(name: str, index: int) -> str:
            if name in field_name_map[entity_ids_for_all_field_names_or_bitfields[index]]:
                return field_name_map[entity_ids_for_all_field_names_or_bitfields[index]][name]
            return name

        parents = get_entity_parents(entity_id, burger_entity_metadata)
        for parent_id in list(reversed(parents)):
            for index, name_or_bitfield in get_entity_metadata_names(parent_id, burger_entity_metadata, mappings).items():
                assert index == len(all_field_names_or_bitfields)
                all_field_names_or_bitfields.append(name_or_bitfield)
                entity_ids_for_all_field_names_or_bitfields.append(parent_id)
            entity_metadatas.extend(get_entity_metadata(
                parent_id, burger_entity_metadata))
        parent_id = parents[1] if len(parents) > 1 else None

        # now add all the fields/component structs
        for index, name_or_bitfield in enumerate(all_field_names_or_bitfields):
            # make sure we only ever make these structs once
            hashable_name_or_bitfield = str(
                name_or_bitfield) + entity_ids_for_all_field_names_or_bitfields[index]
            if hashable_name_or_bitfield in added_metadata_fields:
                continue
            added_metadata_fields.add(hashable_name_or_bitfield)

            if isinstance(name_or_bitfield, str):
                # we just use the imported type instead of making our own
                if name_or_bitfield in single_use_imported_types:
                    continue

                name_or_bitfield = maybe_rename_field(name_or_bitfield, index)

                struct_name = upper_first_letter(
                    to_camel_case(name_or_bitfield))
                type_id = next(filter(lambda i: i['index'] == index, entity_metadatas))['type_id']
                metadata_type_data = metadata_types[type_id]
                rust_type = metadata_type_data['type']

                code.append(f'#[derive(Component, Deref, DerefMut, Clone)]')
                code.append(f'pub struct {struct_name}(pub {rust_type});')
            else:
                # if it's a bitfield just make a struct for each bit
                for mask, name in name_or_bitfield.items():
                    name = maybe_rename_field(name, index)
                    struct_name = upper_first_letter(to_camel_case(name))
                    code.append(f'#[derive(Component, Deref, DerefMut, Clone, Copy)]')
                    code.append(f'pub struct {struct_name}(pub bool);')

        # add the entity struct and Bundle struct
        struct_name: str = upper_first_letter(
            to_camel_case(entity_id.lstrip('~')))
        code.append(f'#[derive(Component)]')
        code.append(f'pub struct {struct_name};')

        parent_struct_name = upper_first_letter(
            to_camel_case(parent_id.lstrip("~"))) if parent_id else None

        # impl Allay {
        #     pub fn apply_metadata(
        #         entity: &mut bevy_ecs::system::EntityCommands,
        #         d: EntityDataItem,
        #     ) -> Result<(), UpdateMetadataError> {
        #         match d.index {
        #             0..=15 => AbstractCreatureBundle::apply_metadata(entity, d)?,
        #             16 => entity.insert(Dancing(d.value.into_boolean()?)),
        #             17 => entity.insert(CanDuplicate(d.value.into_boolean()?)),
        #         }
        #         Ok(())
        #     }
        # }
        code.append(f'impl {struct_name} {{')
        code.append(
            f'    pub fn apply_metadata(entity: &mut bevy_ecs::system::EntityCommands, d: EntityDataItem) -> Result<(), UpdateMetadataError> {{')
        code.append(f'        match d.index {{')

        parent_last_index = -1
        for index, name_or_bitfield in enumerate(all_field_names_or_bitfields):
            is_from_parent = entity_ids_for_all_field_names_or_bitfields[index] != entity_id
            if is_from_parent:
                parent_last_index = index
        if parent_last_index != -1:
            code.append(
                f'            0..={parent_last_index} => {parent_struct_name}::apply_metadata(entity, d)?,')

        for index, name_or_bitfield in enumerate(all_field_names_or_bitfields):
            if index <= parent_last_index:
                continue
            if isinstance(name_or_bitfield, str):
                name_or_bitfield = maybe_rename_field(
                    name_or_bitfield, index)

                field_struct_name = upper_first_letter(
                    to_camel_case(name_or_bitfield))
                if name_or_bitfield in single_use_imported_types:
                    field_struct_name = ''

                type_id = next(filter(lambda i: i['index'] == index, entity_metadatas))['type_id']
                metadata_type_data = metadata_types[type_id]
                rust_type = metadata_type_data['type']
                type_name = metadata_type_data['name']

                type_name_field = to_snake_case(type_name)
                read_field_code = f'{field_struct_name}(d.value.into_{type_name_field}()?)' if field_struct_name else f'd.value.into_{type_name_field}()?'
                code.append(
                    f'            {index} => {{ entity.insert({read_field_code}); }},')
            else:
                code.append(f'                {index} => {{')
                code.append(
                    f'let bitfield = d.value.into_byte()?;')
                for mask, name in name_or_bitfield.items():
                    name = maybe_rename_field(name, index)
                    field_struct_name = upper_first_letter(to_camel_case(name))

                    code.append(
                        f'entity.insert({field_struct_name}(bitfield & {mask} != 0));')
                code.append('            },')
        code.append('            _ => {}')
        code.append('        }')
        code.append('        Ok(())')
        code.append('    }')
        code.append('}')
        code.append('')

        # #[derive(Bundle)]
        # struct AllayBundle {
        #     health: Health,
        #     ...
        #     dancing: Dancing,
        #     can_duplicate: CanDuplicate,
        # }
        bundle_struct_name = f'{struct_name}MetadataBundle'
        code.append(f'')
        code.append(f'#[derive(Bundle)]')
        code.append(f'pub struct {bundle_struct_name} {{')
        code.append(
            f'    _marker: {struct_name},')
        if parent_struct_name:
            code.append(
                f'    parent: {parent_struct_name}MetadataBundle,')
        for index, name_or_bitfield in get_entity_metadata_names(entity_id, burger_entity_metadata, mappings).items():
            if isinstance(name_or_bitfield, str):
                name_or_bitfield = maybe_rename_field(
                    name_or_bitfield, index)
                struct_name = upper_first_letter(
                    to_camel_case(name_or_bitfield))
                code.append(
                    f'    {name_or_bitfield}: {struct_name},')
            else:
                for mask, name in name_or_bitfield.items():
                    name = maybe_rename_field(name, index)

                    struct_name = upper_first_letter(to_camel_case(name))
                    code.append(f'    {name}: {struct_name},')
        code.append('}')

        # impl Default for AllayBundle {
        #     fn default() -> Self {
        #         Self {
        #             _marker: Allay,
        #             parent: AbstractCreatureBundle {
        #                 on_fire: OnFire(false),
        #                 shift_key_down: ShiftKeyDown(false),
        #             },
        #             sprinting: Sprinting(false),
        #             swimming: Swimming(false)
        #        }
        #     }
        # }
        code.append(f'impl Default for {bundle_struct_name} {{')
        code.append(
            '    fn default() -> Self {')

        def generate_fields(this_entity_id: str):
            # on_fire: OnFire(false),
            # shift_key_down: ShiftKeyDown(false),

            # _marker
            this_entity_struct_name = upper_first_letter(
                to_camel_case(this_entity_id.lstrip('~')))
            code.append(
                f'            _marker: {this_entity_struct_name},')

            # if it has a parent, put it (do recursion)
            # parent: AbstractCreatureBundle { ... },
            this_entity_parent_ids = get_entity_parents(
                this_entity_id, burger_entity_metadata)
            this_entity_parent_id = this_entity_parent_ids[1] if len(
                this_entity_parent_ids) > 1 else None
            if this_entity_parent_id:
                bundle_struct_name = upper_first_letter(
                    to_camel_case(this_entity_parent_id.lstrip('~'))) + 'MetadataBundle'
                code.append(
                    f'            parent: {bundle_struct_name} {{')
                generate_fields(this_entity_parent_id)
                code.append(
                    '            },')

            for index, name_or_bitfield in get_entity_metadata_names(this_entity_id, burger_entity_metadata, mappings).items():
                default = next(filter(lambda i: i['index'] == index, entity_metadatas)).get('default', 'Default::default()')
                if isinstance(name_or_bitfield, str):
                    type_id = next(filter(lambda i: i['index'] == index, entity_metadatas))[
                        'type_id']
                    metadata_type_data = metadata_types[type_id]
                    type_name = metadata_type_data['name']

                    name = maybe_rename_field(name_or_bitfield, index)

                    # TODO: burger doesn't get the default if it's a complex type
                    # like `Rotations`, so entities like armor stands will have the
                    # wrong default metadatas. This should be added to Burger.
                    if default is None:
                        # some types don't have Default implemented
                        if type_name == 'CompoundTag':
                            default = 'simdnbt::owned::NbtCompound::default()'
                        elif type_name == 'CatVariant':
                            default = 'azalea_registry::CatVariant::Tabby'
                        elif type_name == 'PaintingVariant':
                            default = 'azalea_registry::PaintingVariant::Kebab'
                        elif type_name == 'FrogVariant':
                            default = 'azalea_registry::FrogVariant::Temperate'
                        elif type_name == 'VillagerData':
                            default = 'VillagerData { kind: azalea_registry::VillagerKind::Plains, profession: azalea_registry::VillagerProfession::None, level: 0 }'
                        else:
                            default = f'{type_name}::default()' if name in single_use_imported_types else 'Default::default()'
                    else:
                        if type_name == 'Boolean':
                            default = 'true' if default else 'false'
                        elif type_name == 'String':
                            string_escaped = default.replace('"', '\\"')
                            default = f'"{string_escaped}".to_string()'
                        elif type_name == 'BlockPos':
                            default = f'BlockPos::new{default}'
                        elif type_name == 'OptionalBlockPos':  # Option<BlockPos>
                            default = f'Some(BlockPos::new{default})' if default != 'Empty' else 'None'
                        elif type_name == 'OptionalLivingEntityReference':
                            default = f'Some(uuid::uuid!({default}))' if default != 'Empty' else 'None'
                        elif type_name == 'OptionalUnsignedInt':
                            default = f'OptionalUnsignedInt(Some({default}))' if default != 'Empty' else 'OptionalUnsignedInt(None)'
                        elif type_name == 'ItemStack':
                            default = f'ItemStack::Present({default})' if default != 'Empty' else 'ItemStack::Empty'
                        elif type_name == 'BlockState':
                            default = f'{default}' if default != 'Empty' else 'azalea_block::BlockState::AIR'
                        elif type_name == 'OptionalBlockState':
                            default = f'{default}' if default != 'Empty' else 'azalea_block::BlockState::AIR'
                        elif type_name == 'OptionalFormattedText':
                            default = f'Some({default})' if default != 'Empty' else 'None'
                        elif type_name == 'CompoundTag':
                            default = f'simdnbt::owned::NbtCompound({default})' if default != 'Empty' else 'simdnbt::owned::NbtCompound::default()'
                        elif type_name == 'Quaternion':
                            default = f'Quaternion {{ x: {float(default["x"])}, y: {float(default["y"])}, z: {float(default["z"])}, w: {float(default["w"])} }}'
                        elif type_name == 'Vector3':
                            default = f'Vec3 {{ x: {float(default["x"])}, y: {float(default["y"])}, z: {float(default["z"])} }}'
                        elif type_name == 'Byte':
                            # in 1.19.4 TextOpacity is a -1 by default
                            if default < 0:
                                default += 128
                    if name in single_use_imported_types:
                        code.append(f'            {name}: {default},')
                    else:
                        code.append(
                            f'            {name}: {upper_first_letter(to_camel_case(name))}({default}),')
                else:
                    # if it's a bitfield, we'll have to extract the default for
                    # each bool from each bit in the default
                    for mask, name in name_or_bitfield.items():
                        name = maybe_rename_field(name, index)
                        mask = int(mask, 0)
                        if default is None:
                            bit_default = 'false'
                        else:
                            bit_default = 'true' if (default & mask != 0) else 'false'
                        code.append(
                            f'            {name}: {upper_first_letter(to_camel_case(name))}({bit_default}),')
        code.append('        Self {')
        generate_fields(entity_id)
        code.append('        }')
        code.append('    }')
        code.append('}')
        code.append('')

    # parent_field_name = None
    for entity_id in burger_entity_metadata:
        new_entity(entity_id)

    # and now make the main apply_metadata
    # pub fn apply_metadata(
    #     entity: &mut bevy_ecs::system::EntityCommands,
    #     items: Vec<EntityDataItem>,
    # ) -> Result<(), UpdateMetadataError> {
    #     if entity.contains::<Allay>() {
    #         for d in items {
    #             Allay::apply_metadata(entity, d)?;
    #         }
    #         return Ok(());
    #     }
    #
    #     Ok(())
    # }
    code.append(
        f'''pub fn apply_metadata(
    entity: &mut bevy_ecs::system::EntityCommands,
    entity_kind: azalea_registry::EntityKind,
    items: Vec<EntityDataItem>,
) -> Result<(), UpdateMetadataError> {{
    match entity_kind {{''')
    for entity_id in burger_entity_metadata:
        if entity_id.startswith('~'):
            # not actually an entity
            continue
        struct_name: str = upper_first_letter(to_camel_case(entity_id))
        code.append(
            f'        azalea_registry::EntityKind::{struct_name} => {{')
        code.append('            for d in items {')
        code.append(
            f'                {struct_name}::apply_metadata(entity, d)?;')
        code.append('            }')
        code.append('        },')
    code.append('    }')
    code.append('    Ok(())')
    code.append('}')
    code.append('')

    # pub fn apply_default_metadata(entity: &mut bevy_ecs::system::EntityCommands, kind: azalea_registry::EntityKind) {
    #     match kind {
    #         azalea_registry::EntityKind::AreaEffectCloud => {
    #             entity.insert(AreaEffectCloudMetadataBundle::default());
    #         }
    #     }
    # }
    code.append(
        'pub fn apply_default_metadata(entity: &mut bevy_ecs::system::EntityCommands, kind: azalea_registry::EntityKind) {')
    code.append('    match kind {')
    for entity_id in burger_entity_metadata:
        if entity_id.startswith('~'):
            # not actually an entity
            continue
        struct_name: str = upper_first_letter(to_camel_case(entity_id))
        code.append(
            f'        azalea_registry::EntityKind::{struct_name} => {{')
        code.append(
            f'            entity.insert({struct_name}MetadataBundle::default());')
        code.append('        },')
    code.append('    }')
    code.append('}')
    code.append('')

    with open(METADATA_RS_DIR, 'w') as f:
        f.write('\n'.join(code))

def generate_entity_dimensions(burger_entities_data: dict):
    # lines look like
    # EntityKind::Player => EntityDimensions::new(0.6, 1.8),
    new_match_lines = []
    for entity_id, entity_data in burger_entities_data['entity'].items():
        if entity_id.startswith('~'):
            # not actually an entity
            continue
        variant_name: str = upper_first_letter(to_camel_case(entity_id))
        width = entity_data['width']
        height = entity_data['height']
        new_match_lines.append(
            f'        EntityKind::{variant_name} => EntityDimensions::new({width}, {height}),')

    with open(DIMENSIONS_RS_DIR, 'r') as f:
        lines = f.read().split('\n')
    new_lines = []

    in_match = False
    for i, line in enumerate(lines):
        if not in_match:
            new_lines.append(line)
            if line.strip() == 'match entity {':
                in_match = True
        else:
            if line.strip() == '}':
                new_lines.extend(new_match_lines)
                new_lines.extend(lines[i:])
                break

    with open(DIMENSIONS_RS_DIR, 'w') as f:
        f.write('\n'.join(new_lines))

def get_entity_parents(entity_id: str, burger_entity_metadata: dict):
    parents = []
    while entity_id:
        parents.append(entity_id)
        entity_id = get_entity_parent(entity_id, burger_entity_metadata)
    return parents


def get_entity_parent(entity_id: str, burger_entity_metadata: dict):
    entity_metadata = burger_entity_metadata[entity_id]['metadata']
    first_metadata = entity_metadata[0]
    return first_metadata.get('entity')


def get_entity_metadata(entity_id: str, burger_entity_metadata: dict):
    entity_metadata = burger_entity_metadata[entity_id]['metadata']
    entity_useful_metadata = []
    for metadata_item in entity_metadata:
        if 'data' in metadata_item:
            for metadata_attribute in metadata_item['data']:
                entity_useful_metadata.append({
                    'index': metadata_attribute['index'],
                    'type_id': metadata_attribute['serializer_id'],
                    'default': metadata_attribute.get('default')
                })
    return entity_useful_metadata

# returns a dict of {index: (name or bitfield)}


def get_entity_metadata_names(entity_id: str, burger_entity_metadata: dict, mappings: Mappings):
    entity_metadata = burger_entity_metadata[entity_id]['metadata']
    mapped_metadata_names = {}

    for metadata_item in entity_metadata:
        if 'data' in metadata_item:
            obfuscated_class = metadata_item['class']
            mojang_class = mappings.get_class(obfuscated_class)

            first_byte_index = None

            for metadata_attribute in metadata_item['data']:
                obfuscated_field = metadata_attribute['field']
                mojang_field = mappings.get_field(
                    obfuscated_class, obfuscated_field)
                pretty_mojang_name = prettify_mojang_field(mojang_field)
                mapped_metadata_names[metadata_attribute['index']
                                      ] = pretty_mojang_name

                if metadata_attribute['serializer'] == 'Byte' and first_byte_index is None:
                    first_byte_index = metadata_attribute['index']

            if metadata_item['bitfields'] and first_byte_index is not None:
                clean_bitfield = {}
                for bitfield_item in metadata_item['bitfields']:
                    bitfield_item_obfuscated_class = bitfield_item.get(
                        'class', obfuscated_class)
                    mojang_bitfield_item_name = mappings.get_method(
                        bitfield_item_obfuscated_class, bitfield_item['method'], '')
                    bitfield_item_name = prettify_mojang_method(
                        mojang_bitfield_item_name)
                    bitfield_hex_mask = hex(bitfield_item['mask'])
                    clean_bitfield[bitfield_hex_mask] = bitfield_item_name
                mapped_metadata_names[first_byte_index] = clean_bitfield
    return mapped_metadata_names


def prettify_mojang_field(mojang_field: str):
    # mojang names are like "DATA_AIR_SUPPLY" and that's ugly
    better_name = mojang_field
    if better_name.startswith('DATA_'):
        better_name = better_name[5:]

    # remove the weird "Id" from the end of names
    if better_name.endswith('_ID'):
        better_name = better_name[:-3]
    # remove the weird "id" from the front of names
    if better_name.startswith('ID_'):
        better_name = better_name[3:]

    return better_name.lower()


def prettify_mojang_method(mojang_method: str):
    better_name = mojang_method
    if better_name.endswith('()'):
        better_name = better_name[:-2]
    if re.match(r'is[A-Z]', better_name):
        better_name = better_name[2:]
    return to_snake_case(better_name)

