from lib.utils import to_camel_case, to_snake_case, get_dir_location, upper_first_letter
from lib.mappings import Mappings
from typing import Optional
import re

METADATA_RS_DIR = get_dir_location(
    '../azalea-world/src/entity/metadata.rs')


def generate_entity_metadata(burger_entity_data: dict, mappings: Mappings):
    # TODO: auto generate this and use it for generating the EntityDataValue enum
    metadata_types = [
        {'name': 'Byte', 'type': 'u8'},
        {'name': 'Int', 'type': 'i32', 'var': True},
        {'name': 'Long', 'type': 'i64'},
        {'name': 'Float', 'type': 'f32'},
        {'name': 'String', 'type': 'String'},
        {'name': 'Component', 'type': 'Component'},
        {'name': 'OptionalComponent', 'type': 'Option<Component>'},
        {'name': 'ItemStack', 'type': 'Slot'},
        {'name': 'Boolean', 'type': 'bool'},
        {'name': 'Rotations', 'type': 'Rotations'},
        {'name': 'BlockPos', 'type': 'BlockPos'},
        {'name': 'OptionalBlockPos', 'type': 'Option<BlockPos>'},
        {'name': 'Direction', 'type': 'Direction'},
        {'name': 'OptionalUuid', 'type': 'Option<Uuid>'},
        {'name': 'BlockState', 'type': 'BlockState'},
        {'name': 'CompoundTag', 'type': 'azalea_nbt::Tag'},
        {'name': 'Particle', 'type': 'Particle'},
        {'name': 'VillagerData', 'type': 'VillagerData'},
        {'name': 'OptionalUnsignedInt', 'type': 'OptionalUnsignedInt'},
        {'name': 'Pose', 'type': 'Pose'},
        {'name': 'CatVariant', 'type': 'azalea_registry::CatVariant'},
        {'name': 'FrogVariant', 'type': 'azalea_registry::FrogVariant'},
        {'name': 'GlobalPos', 'type': 'GlobalPos'},
        {'name': 'PaintingVariant', 'type': 'azalea_registry::PaintingVariant'}
    ]

    code = []
    code.append('''// This file is generated from codegen/lib/code/entity.py.
// Don't change it manually!

#![allow(clippy::clone_on_copy, clippy::derivable_impls)]
use super::{
    EntityDataValue, EntityMetadataItems, OptionalUnsignedInt, Pose, Rotations, VillagerData,
};
use azalea_block::BlockState;
use azalea_chat::Component;
use azalea_core::{BlockPos, Direction, Particle, Slot};
use hecs::{Query, EntityBuilder, BuiltEntity};
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
    for entity_id in burger_entity_data.keys():
        field_name_map[entity_id] = {}
        for field_name_or_bitfield in get_entity_metadata_names(entity_id, burger_entity_data, mappings).values():
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
    for entity_id in burger_entity_data.keys():
        for index, field_name_or_bitfield in get_entity_metadata_names(entity_id, burger_entity_data, mappings).items():
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

        for parent_id in list(reversed(get_entity_parents(entity_id, burger_entity_data))):
            for index, name_or_bitfield in get_entity_metadata_names(parent_id, burger_entity_data, mappings).items():
                assert index == len(all_field_names_or_bitfields)
                all_field_names_or_bitfields.append(name_or_bitfield)
                entity_ids_for_all_field_names_or_bitfields.append(parent_id)
            entity_metadatas.extend(get_entity_metadata(
                parent_id, burger_entity_data))

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
                type_id = next(filter(lambda i: i['index'] == index, entity_metadatas))[
                    'type_id']
                metadata_type_data = metadata_types[type_id]
                rust_type = metadata_type_data['type']

                code.append(f'pub struct {struct_name}(pub {rust_type});')
            else:
                # if it's a bitfield just make a struct for each bit
                for mask, name in name_or_bitfield.items():
                    name = maybe_rename_field(name, index)
                    struct_name = upper_first_letter(to_camel_case(name))
                    code.append(f'pub struct {struct_name}(pub bool);')

        # add the entity struct and Query struct
        is_actually_entity = not entity_id.startswith('~')
        if is_actually_entity:
            struct_name: str = upper_first_letter(to_camel_case(entity_id))
            code.append(f'pub struct {struct_name};')

            # impl Allay {
            #     pub fn default(builder: &mut EntityBuilder) {
            #         builder
            #             .add(OnFire(false))
            #             .add(ShiftKeyDown(false))
            #             .add(Sprinting(false))
            #             .add(Swimming(false));
            #     }
            # }
            code.append(f'impl {struct_name} {{')
            code.append(
                '    pub fn default(builder: &mut EntityBuilder) -> BuiltEntity {')
            code.append('        builder')
            for index, name_or_bitfield in enumerate(all_field_names_or_bitfields):
                default = next(filter(lambda i: i['index'] == index, entity_metadatas)).get(
                    'default', 'Default::default()')
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
                            default = 'azalea_nbt::Tag::Compound(Default::default())'
                        elif type_name == 'CatVariant':
                            default = 'azalea_registry::CatVariant::Tabby'
                        elif type_name == 'PaintingVariant':
                            default = 'azalea_registry::PaintingVariant::Kebab'
                        elif type_name == 'FrogVariant':
                            default = 'azalea_registry::FrogVariant::Temperate'
                        elif type_name == 'VillagerData':
                            default = 'VillagerData { kind: azalea_registry::VillagerType::Plains, profession: azalea_registry::VillagerProfession::None, level: 0 }'
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
                        elif type_name == 'OptionalUuid':
                            default = f'Some(uuid::uuid!({default}))' if default != 'Empty' else 'None'
                        elif type_name == 'OptionalUnsignedInt':
                            default = f'OptionalUnsignedInt(Some({default}))' if default != 'Empty' else 'OptionalUnsignedInt(None)'
                        elif type_name == 'ItemStack':
                            default = f'Slot::Present({default})' if default != 'Empty' else 'Slot::Empty'
                        elif type_name == 'BlockState':
                            default = f'{default}' if default != 'Empty' else 'BlockState::Air'
                        elif type_name == 'OptionalComponent':
                            default = f'Some({default})' if default != 'Empty' else 'None'
                        elif type_name == 'CompoundTag':
                            default = f'azalea_nbt::Tag::Compound({default})' if default != 'Empty' else 'azalea_nbt::Tag::Compound(Default::default())'
                    if name in single_use_imported_types:
                        code.append(f'            .add({default})')
                    else:
                        code.append(
                            f'            .add({upper_first_letter(to_camel_case(name))}({default}))')
                else:
                    # if it's a bitfield, we'll have to extract the default for
                    # each bool from each bit in the default
                    for mask, name in name_or_bitfield.items():
                        name = maybe_rename_field(name, index)
                        mask = int(mask, 0)
                        bit_default = 'true' if (
                            default & mask != 0) else 'false'
                        code.append(
                            f'            .add({upper_first_letter(to_camel_case(name))}({bit_default}))')
            code.append(f'            .build()')
            code.append('    }')
            code.append('}')

            # #[derive(Query)]
            # struct AllayQuery<'a> {
            #     health: &'a mut Health,
            #     ...
            #     dancing: &'a mut Dancing,
            #     can_duplicate: &'a mut CanDuplicate,
            # }
            query_struct_name = f'{struct_name}Query'
            code.append(f'')
            code.append(f'#[derive(Query)]')
            code.append(f'struct {query_struct_name}<\'a> {{')
            for index, name_or_bitfield in enumerate(all_field_names_or_bitfields):
                if isinstance(name_or_bitfield, str):
                    name_or_bitfield = maybe_rename_field(
                        name_or_bitfield, index)
                    struct_name = upper_first_letter(
                        to_camel_case(name_or_bitfield))
                    code.append(
                        f'    {name_or_bitfield}: &\'a mut {struct_name},')
                else:
                    for mask, name in name_or_bitfield.items():
                        name = maybe_rename_field(name, index)

                        struct_name = upper_first_letter(to_camel_case(name))
                        code.append(f'    {name}: &\'a mut {struct_name},')
            code.append('}')

            # impl AllayQuery<'_> {
            #     pub fn update_metadata(
            #         &mut self,
            #         world: hecs::World,
            #         entity: hecs::Entity,
            #         data: EntityMetadataItems,
            #     ) -> Result<(), UpdateMetadataError> {
            #         for d in data.0 {
            #             match d.index {
            #                 0 => *self.health = Health(d.value.into_float()?),
            #                 1 => *self.dancing = Dancing(d.value.into_boolean()?),
            #                 2 => *self.can_duplicate = CanDuplicate(d.value.into_boolean()?),
            #             }
            #         }
            #         Ok(())
            #     }
            # }
            code.append(f'impl {query_struct_name}<\'_> {{')
            code.append(
                f'    pub fn update_metadata(&mut self, world: hecs::World, entity: hecs::Entity, data: EntityMetadataItems) -> Result<(), UpdateMetadataError> {{')
            code.append(f'        for d in data.0 {{')
            code.append(f'            match d.index {{')
            for index, name_or_bitfield in enumerate(all_field_names_or_bitfields):
                if isinstance(name_or_bitfield, str):
                    name_or_bitfield = maybe_rename_field(
                        name_or_bitfield, index)

                    struct_name = upper_first_letter(
                        to_camel_case(name_or_bitfield))
                    if name_or_bitfield in single_use_imported_types:
                        struct_name = ''

                    type_id = next(filter(lambda i: i['index'] == index, entity_metadatas))[
                        'type_id']
                    metadata_type_data = metadata_types[type_id]
                    rust_type = metadata_type_data['type']
                    type_name = metadata_type_data['name']

                    type_name_field = to_snake_case(type_name)
                    read_field_code = f'{struct_name}(d.value.into_{type_name_field}()?)' if struct_name else f'd.value.into_{type_name_field}()?'
                    code.append(
                        f'                {index} => *self.{name_or_bitfield} = {read_field_code},')
                else:
                    code.append(f'                {index} => {{')
                    code.append(
                        f'let bitfield = d.value.into_byte()?;')
                    for mask, name in name_or_bitfield.items():
                        name = maybe_rename_field(name, index)
                        struct_name = upper_first_letter(to_camel_case(name))

                        code.append(
                            f'*self.{name} = {struct_name}(bitfield & {mask} != 0);')
                    code.append('                },')
            code.append('            }')
            code.append('        }')
            code.append('        Ok(())')
            code.append('    }')
            code.append('}')
            code.append('')

    # parent_field_name = None
    for entity_id in burger_entity_data:
        new_entity(entity_id)

    # and now make the main update_metadata
    # fn update_metadata(
    #     world: hecs::World,
    #     entity: hecs::Entity,
    #     data: EntityMetadataItems,
    # ) -> Result<(), UpdateMetadataError> {
    #     if let Ok(e) = world.query_one_mut::<AllayQuery>(entity) {
    #         e.update_metadata(world, entity, data)?;
    #         return Ok(());
    #     }

    #     Ok(())
    # }
    code.append(
        f'fn update_metadata(world: hecs::World, entity: hecs::Entity, data: EntityMetadataItems) -> Result<(), UpdateMetadataError> {{')
    for entity_id in burger_entity_data:
        if entity_id.startswith('~'):
            # not actually an entiry
            continue
        struct_name: str = upper_first_letter(to_camel_case(entity_id))
        query_struct_name = f'{struct_name}Query'
        code.append(
            f'    if let Ok(e) = world.query_one_mut::<{query_struct_name}>(entity) {{')
        code.append(
            f'        e.update_metadata(world, entity, data)?;')
        code.append(f'        return Ok(());')
        code.append('    }')
    code.append('    Ok(())')
    code.append('}')
    code.append('')

    #     entity_parents = get_entity_parents(entity_id, burger_entity_data)
    #     entity_metadata = get_entity_metadata(entity_id, burger_entity_data)
    #     entity_metadata_names = get_entity_metadata_names(
    #         entity_id, burger_entity_data, mappings)

    #     struct_name: str = upper_first_letter(
    #         to_camel_case(entity_parents[0].replace('~', '')))
    #     parent_struct_name: Optional[str] = upper_first_letter(to_camel_case(
    #         entity_parents[1].replace('~', ''))) if (len(entity_parents) >= 2) else None
    #     if parent_struct_name:
    #         parent_field_name = to_snake_case(parent_struct_name)
    #     if not entity_parents[0].startswith('~'):
    #         entity_structs.append(struct_name)

    #     reader_code = []
    #     writer_code = []
    #     set_index_code = []
    #     field_names = []

    #     code.append(f'#[derive(Debug, Clone)]')
    #     code.append(f'pub struct {struct_name} {{')

    #     if parent_struct_name:
    #         assert parent_field_name
    #         code.append(f'pub {parent_field_name}: {parent_struct_name},')
    #         reader_code.append(
    #             f'let {parent_field_name} = {parent_struct_name}::read(metadata)?;')
    #         writer_code.append(
    #             f'metadata.extend(self.{parent_field_name}.write());')
    #     for index, name_or_bitfield in entity_metadata_names.items():
    #         if isinstance(name_or_bitfield, str):
    #             # normal field (can be any type)
    #             name = name_or_bitfield
    #             if name == 'type':
    #                 name = 'kind'
    #             field_names.append(name)
    #             type_id = next(filter(lambda i: i['index'] == index, entity_metadata))[
    #                 'type_id']
    #             metadata_type_data = metadata_types[type_id]
    #             rust_type = metadata_type_data['type']
    #             type_name = metadata_type_data['name']
    #             code.append(f'pub {name}: {rust_type},')

    #             type_name_field = to_snake_case(type_name)
    #             reader_code.append(
    #                 f'let {name} = metadata.pop_front()?.into_{type_name_field}().ok()?;')
    #             writer_code.append(
    #                 f'metadata.push(EntityDataValue::{type_name}(self.{name}.clone()));')

    #             # 1 => self.dancing = value.into_boolean().ok()?,
    #             set_index_code.append(
    #                 f'{index} => self.{name} = value.into_{type_name_field}().ok()?,'
    #             )
    #         else:
    #             # bitfield (sent as a byte, each bit in the byte is used as a boolean)
    #             reader_code.append(
    #                 'let bitfield = metadata.pop_front()?.into_byte().ok()?;')
    #             writer_code.append('let mut bitfield = 0u8;')
    #             set_index_code.append(f'{index} => {{')
    #             set_index_code.append(
    #                 f'let bitfield = value.into_byte().ok()?;')
    #             for mask, name in name_or_bitfield.items():
    #                 if name == 'type':
    #                     name = 'kind'

    #                 field_names.append(name)
    #                 code.append(f'pub {name}: bool,')
    #                 reader_code.append(f'let {name} = bitfield & {mask} != 0;')
    #                 writer_code.append(
    #                     f'if self.{name} {{ bitfield &= {mask}; }}')
    #                 set_index_code.append(
    #                     f'self.{name} = bitfield & {mask} != 0;')
    #             writer_code.append(
    #                 'metadata.push(EntityDataValue::Byte(bitfield));')
    #             set_index_code.append('},')

    #     code.append('}')
    #     code.append('')

    #     code.append(f'impl {struct_name} {{')

    #     code.append(
    #         'pub fn read(metadata: &mut VecDeque<EntityDataValue>) -> Option<Self> {')
    #     code.extend(reader_code)

    #     self_args = []
    #     if parent_struct_name:
    #         self_args.append(
    #             f'{parent_field_name}')
    #     self_args.extend(field_names)
    #     code.append(f'Some(Self {{ {",".join(self_args)} }})')
    #     code.append('}')
    #     code.append('')

    #     code.append('pub fn write(&self) -> Vec<EntityDataValue> {')
    #     code.append('let mut metadata = Vec::new();')
    #     code.extend(writer_code)
    #     code.append('metadata')
    #     code.append('}')

    #     code.append('}')
    #     code.append('')

    #     # default
    #     code.append(f'impl Default for {struct_name} {{')
    #     code.append('fn default() -> Self {')
    #     default_fields_code = []
    #     if parent_struct_name:
    #         assert parent_field_name
    #         default_fields_code.append(
    #             f'{parent_field_name}: Default::default()')
    #     for index, name_or_bitfield in entity_metadata_names.items():
    #         default = next(filter(lambda i: i['index'] == index, entity_metadata)).get(
    #             'default', 'Default::default()')
    #         if isinstance(name_or_bitfield, str):
    #             type_id = next(filter(lambda i: i['index'] == index, entity_metadata))[
    #                 'type_id']
    #             metadata_type_data = metadata_types[type_id]
    #             type_name = metadata_type_data['name']

    #             # TODO: burger doesn't get the default if it's a complex type
    #             # like `Rotations`, so entities like armor stands will have the
    #             # wrong default metadatas. This should be added to Burger.
    #             if default is None:
    #                 # some types don't have Default implemented
    #                 if type_name == 'CompoundTag':
    #                     default = 'azalea_nbt::Tag::Compound(Default::default())'
    #                 elif type_name == 'CatVariant':
    #                     default = 'azalea_registry::CatVariant::Tabby'
    #                 elif type_name == 'PaintingVariant':
    #                     default = 'azalea_registry::PaintingVariant::Kebab'
    #                 elif type_name == 'FrogVariant':
    #                     default = 'azalea_registry::FrogVariant::Temperate'
    #                 elif type_name == 'VillagerData':
    #                     default = 'VillagerData { kind: azalea_registry::VillagerType::Plains, profession: azalea_registry::VillagerProfession::None, level: 0 }'
    #                 else:
    #                     default = 'Default::default()'
    #             else:
    #                 if type_name == 'Boolean':
    #                     default = 'true' if default else 'false'
    #                 elif type_name == 'String':
    #                     string_escaped = default.replace('"', '\\"')
    #                     default = f'"{string_escaped}".to_string()'
    #                 elif type_name == 'BlockPos':
    #                     default = f'BlockPos::new{default}'
    #                 elif type_name == 'OptionalBlockPos':  # Option<BlockPos>
    #                     default = f'Some(BlockPos::new{default})' if default != 'Empty' else 'None'
    #                 elif type_name == 'OptionalUuid':
    #                     default = f'Some(uuid::uuid!({default}))' if default != 'Empty' else 'None'
    #                 elif type_name == 'OptionalUnsignedInt':
    #                     default = f'OptionalUnsignedInt(Some({default}))' if default != 'Empty' else 'OptionalUnsignedInt(None)'
    #                 elif type_name == 'ItemStack':
    #                     default = f'Slot::Present({default})' if default != 'Empty' else 'Slot::Empty'
    #                 elif type_name == 'BlockState':
    #                     default = f'{default}' if default != 'Empty' else 'BlockState::Air'
    #                 elif type_name == 'OptionalComponent':
    #                     default = f'Some({default})' if default != 'Empty' else 'None'
    #                 elif type_name == 'CompoundTag':
    #                     default = f'azalea_nbt::Tag::Compound({default})' if default != 'Empty' else 'azalea_nbt::Tag::Compound(Default::default())'

    #             print(default, name_or_bitfield, type_name)
    #             name = name_or_bitfield
    #             if name == 'type':
    #                 name = 'kind'
    #             default_fields_code.append(f'{name}: {default}')
    #         else:
    #             # if it's a bitfield, we'll have to extract the default for
    #             # each bool from each bit in the default
    #             for mask, name in name_or_bitfield.items():
    #                 if name == 'type':
    #                     name = 'kind'
    #                 mask = int(mask, 0)
    #                 field_names.append(name)
    #                 bit_default = 'true' if (default & mask != 0) else 'false'
    #                 default_fields_code.append(f'{name}: {bit_default}')

    #             # Self { abstract_creature: Default::default(), dancing: Default::default(), can_duplicate: Default::default() }
    #     code.append(f'Self {{ {", ".join(default_fields_code)} }}')
    #     code.append('}')
    #     code.append('}')
    #     code.append('')

    #     # impl Allay {
    #     #     pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {
    #     #         match index {
    #     #             0..=0 => self.abstract_creature.set_index(index, value),
    #     #             1 => self.dancing = value.into_boolean().ok()?,
    #     #             2 => self.can_duplicate = value.into_boolean().ok()?,
    #     #             _ => {}
    #     #         }
    #     #         Some(())
    #     #     }
    #     # }
    #     code.append(f'impl {struct_name} {{')
    #     code.append(
    #         'pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {')
    #     if len(entity_metadata_names) > 0:
    #         code.append('match index {')
    #         # get the smallest index for this entity
    #         smallest_index = min(entity_metadata_names.keys())
    #         if parent_struct_name:
    #             code.append(
    #                 f'0..={smallest_index-1} => self.{parent_field_name}.set_index(index, value)?,')
    #         code.extend(set_index_code)
    #         code.append('_ => {}')
    #         code.append('}')
    #         code.append('Some(())')
    #     elif parent_struct_name:
    #         code.append(f'self.{parent_field_name}.set_index(index, value)')
    #     else:
    #         code.append('Some(())')
    #     code.append('}')
    #     code.append('}')

    #     # deref
    #     if parent_struct_name:
    #         code.append(f'impl Deref for {struct_name} {{')
    #         code.append(f'type Target = {parent_struct_name};')
    #         code.append(
    #             f'fn deref(&self) -> &Self::Target {{ &self.{parent_field_name} }}')
    #         code.append('}')
    #         code.append(f'impl DerefMut for {struct_name} {{')
    #         code.append(
    #             f'fn deref_mut(&mut self) -> &mut Self::Target {{ &mut self.{parent_field_name} }}')
    #         code.append('}')
    #         code.append('')

    # # make the EntityMetadata enum from entity_structs
    # code.append(f'#[derive(Debug, Clone, EnumAsInner)]')
    # code.append('pub enum EntityMetadata {')
    # for struct_name in entity_structs:
    #     code.append(f'{struct_name}({struct_name}),')
    # code.append('}')
    # code.append('')

    # # impl From<azalea_registry::EntityKind> for EntityMetadata {
    # code.append('impl From<azalea_registry::EntityKind> for EntityMetadata {')
    # code.append('fn from(value: azalea_registry::EntityKind) -> Self {')
    # code.append('match value {')
    # # azalea_registry::EntityKind::Allay => EntityMetadata::Allay(Allay::default()),
    # for struct_name in entity_structs:
    #     code.append(
    #         f'azalea_registry::EntityKind::{struct_name} => EntityMetadata::{struct_name}({struct_name}::default()),')
    # code.append('}')
    # code.append('}')
    # code.append('}')
    # code.append('')

    # # impl From<EntityMetadata> for azalea_registry::EntityKind {
    # code.append('impl From<&EntityMetadata> for azalea_registry::EntityKind {')
    # code.append('fn from(value: &EntityMetadata) -> Self {')
    # code.append('match value {')
    # # EntityMetadata::Allay(_) => azalea_registry::EntityKind::Allay,
    # for struct_name in entity_structs:
    #     code.append(
    #         f'EntityMetadata::{struct_name}(_) => azalea_registry::EntityKind::{struct_name},')
    # code.append('}')
    # code.append('}')
    # code.append('}')
    # code.append('')

    # # impl EntityMetadata
    # # pub fn set_index(&mut self, index: u8, value: EntityDataValue)
    # code.append('impl EntityMetadata {')
    # code.append(
    #     'pub fn set_index(&mut self, index: u8, value: EntityDataValue) -> Option<()> {')
    # code.append('match self {')
    # # EntityMetadata::Allay(allay) => allay.set_index(index, value),
    # for struct_name in entity_structs:
    #     code.append(
    #         f'EntityMetadata::{struct_name}(entity) => entity.set_index(index, value),')
    # code.append('}')
    # code.append('}')
    # code.append('}')
    # code.append('')

    # # impl Deref for EntityMetadata {
    # #     type Target = AbstractEntity;
    # #     fn deref(&self) -> &Self::Target {
    # #         match self {
    # #             EntityMetadata::Allay(entity) => entity,
    # #             _ => {}
    # #         }
    # #     }
    # # }
    # code.append('impl Deref for EntityMetadata {')
    # code.append('type Target = AbstractEntity;')
    # code.append('fn deref(&self) -> &Self::Target {')
    # code.append('match self {')
    # for struct_name in entity_structs:
    #     code.append(
    #         f'EntityMetadata::{struct_name}(entity) => entity,')
    # code.append('}')
    # code.append('}')
    # code.append('}')
    # code.append('impl DerefMut for EntityMetadata {')
    # code.append('fn deref_mut(&mut self) -> &mut Self::Target {')
    # code.append('match self {')
    # for struct_name in entity_structs:
    #     code.append(
    #         f'EntityMetadata::{struct_name}(entity) => entity,')
    # code.append('}')
    # code.append('}')
    # code.append('}')
    # code.append('')

    with open(METADATA_RS_DIR, 'w') as f:
        f.write('\n'.join(code))


def get_entity_parents(entity_id: str, burger_entity_data: dict):
    parents = []
    while entity_id:
        parents.append(entity_id)
        entity_id = get_entity_parent(entity_id, burger_entity_data)
    return parents


def get_entity_parent(entity_id: str, burger_entity_data: dict):
    entity_metadata = burger_entity_data[entity_id]['metadata']
    first_metadata = entity_metadata[0]
    return first_metadata.get('entity')


def get_entity_metadata(entity_id: str, burger_entity_data: dict):
    entity_metadata = burger_entity_data[entity_id]['metadata']
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


def get_entity_metadata_names(entity_id: str, burger_entity_data: dict, mappings: Mappings):
    entity_metadata = burger_entity_data[entity_id]['metadata']
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
