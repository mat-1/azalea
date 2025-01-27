from lib.utils import padded_hex, to_snake_case, to_camel_case, get_dir_location
from lib.code.utils import burger_type_to_rust_type, write_packet_file
from lib.mappings import Mappings
from typing import Optional
import os
import re


MOJMAP_TO_AZALEA_STATE_NAME_MAPPING = {
    # shorter name, i like it more
    'configuration': 'config',
    # in the files mojang calls the directory "game" so we do that too
    'play': 'game'
}
AZALEA_TO_MOJMAP_STATE_NAME_MAPPING = {v: k for k, v in MOJMAP_TO_AZALEA_STATE_NAME_MAPPING.items()}

PACKETS_DIR = '../azalea-protocol/src/packets'

def generate_packet(packets_report, packet_name, direction, state):
    mojmap_state = AZALEA_TO_MOJMAP_STATE_NAME_MAPPING.get(state, state)
    _packet_report = packets_report[mojmap_state][direction]['minecraft:' + packet_name]

    code = []
    uses = set()

    packet_derive_name = f'{to_camel_case(direction)}{to_camel_case(state)}Packet'

    packet_struct_name = to_camel_case(f'{direction}_{packet_name}')
    packet_module_name = get_packet_module_name(packet_name, direction)

    code.append(f'use azalea_buf::AzBuf;')
    code.append(f'use azalea_protocol_macros::{packet_derive_name};')
    code.append('')
   
    code.append(
        f'#[derive(Clone, Debug, AzBuf, {packet_derive_name})]')
    code.append(
        f'pub struct {packet_struct_name} {{')
    code.append('    TODO')
    code.append('}')

    print(code)
    write_packet_file(state, packet_module_name, '\n'.join(code))

    # this won't handle writing to the packets/{state}/mod.rs file since we'd need to know the full packet list

def get_packet_module_name(packet_name: str, direction: str):
    return f'{direction[0]}_{packet_name}'

def set_packets(packets_report):    
    for mojmap_state in packets_report:
        state = MOJMAP_TO_AZALEA_STATE_NAME_MAPPING.get(mojmap_state, mojmap_state)

        expected_packet_module_names = set()
        state_dir = get_dir_location(f'{PACKETS_DIR}/{state}')
        mod_rs_dir = get_dir_location(f'{state_dir}/mod.rs')

        serverbound_packets = packet_direction_report_to_packet_names(packets_report[mojmap_state]['serverbound'])
        clientbound_packets = packet_direction_report_to_packet_names(packets_report[mojmap_state].get('clientbound', {}))

        code = []
        code.append('// NOTE: This file is generated automatically by codegen/packet.py.')
        code.append("// Don't edit it directly!")
        code.append('')
        code.append('use azalea_protocol_macros::declare_state_packets;')
        code.append('')
        code.append(f'declare_state_packets!({to_camel_case(state)}Packet,')
        code.append('    Clientbound => [')
        for packet_id, packet_name in enumerate(clientbound_packets):
            code.append(f'        {packet_name}, // {padded_hex(packet_id)}')
            expected_packet_module_names.add(get_packet_module_name(packet_name, 'clientbound'))
        code.append('    ],')
        code.append('    Serverbound => [')
        for packet_id, packet_name in enumerate(serverbound_packets):
            code.append(f'        {packet_name}, // {padded_hex(packet_id)}')
            expected_packet_module_names.add(get_packet_module_name(packet_name, 'serverbound'))
        code.append('    ]')
        code.append(');')
        code.append('')

        with open(mod_rs_dir, 'w') as f:
            f.write('\n'.join(code))

        existing_packet_module_names = set()
        # iterate over the directory
        for file in os.listdir(state_dir):
            if file.endswith('.rs') and file != 'mod.rs':
                existing_packet_module_names.add(file[:-len('.rs')])
        for packet_module_name in expected_packet_module_names - existing_packet_module_names:
            direction = None
            if packet_module_name.startswith('c_'):
                direction = 'clientbound'
            elif packet_module_name.startswith('s_'):
                direction = 'serverbound'
            else:
                raise Exception(f'Invalid packet module name: {packet_module_name}')
            packet = packet_module_name[2:]
            generate_packet(packets_report, packet, direction, state)


def packet_direction_report_to_packet_names(report):
    name_to_id = {}
    for resource_location, packet in report.items():
        packet_id = packet['protocol_id']
        name_to_id[resource_location.split(':')[-1]] = packet_id
    
    names_sorted = [name for name in sorted(name_to_id, key=lambda x: name_to_id[x])]
    return names_sorted

def get_packets(direction: str, state: str):
    mod_rs_dir = get_dir_location(f'{PACKETS_DIR}/{state}/mod.rs')
    with open(mod_rs_dir, 'r') as f:
        mod_rs = f.read().splitlines()

    in_serverbound = False
    in_clientbound = False

    packet_ids: list[int] = []
    packet_class_names: list[str] = []

    for line in mod_rs:
        if line.strip() == 'Serverbound => {':
            in_serverbound = True
            continue
        elif line.strip() == 'Clientbound => {':
            in_clientbound = True
            continue
        elif line.strip() in ('}', '},'):
            if (in_serverbound and direction == 'serverbound') or (in_clientbound and direction == 'clientbound'):
                break
            in_serverbound = in_clientbound = False
            continue

        if line.strip() == '' or line.strip().startswith('//') or (not in_serverbound and direction == 'serverbound') or (not in_clientbound and direction == 'clientbound'):
            continue

        line_packet_id_hex = line.strip().split(':')[0]
        assert line_packet_id_hex.startswith('0x')
        line_packet_id = int(line_packet_id_hex[2:], 16)
        packet_ids.append(line_packet_id)

        packet_class_name = line.strip().split(':')[1].strip()
        packet_class_names.append(packet_class_name)

    return packet_ids, packet_class_names


def burger_instruction_to_code(instructions: list[dict], index: int, generated_packet_code: list[str], mappings: Mappings, obfuscated_class_name: str, uses: set, extra_code: list[str], known_variable_types={}) -> Optional[int]:
    '''
    Generate a field for an instruction, returns the number of instructions to skip (if any).
    '''
    instruction = instructions[index]
    next_instruction = instructions[index +
                                    1] if index + 1 < len(instructions) else None
    next_next_instruction = instructions[index +
                                         2] if index + 2 < len(instructions) else None

    is_var = False
    skip = 0
    field_type_rs = None
    field_comment = None

    print('instruction', instruction, next_instruction, next_next_instruction)

    # iterators
    if instruction['operation'] == 'write'\
        and instruction['field'].endswith('.size()')\
        and next_instruction\
        and next_instruction['type'] == 'Iterator'\
        and next_next_instruction\
        and next_next_instruction['operation'] == 'loop':
        obfuscated_field_name = instruction['field'].split('.')[0]
        field_name = mappings.get_field(
            obfuscated_class_name, obfuscated_field_name)

        # figure out what kind of iterator it is
        loop_instructions = next_next_instruction['instructions']
        if len(loop_instructions) == 2:
            entry_type_rs, is_var, value_uses, extra_code = burger_type_to_rust_type(
                loop_instructions[1]['type'], None, loop_instructions[1], mappings, obfuscated_class_name)
            field_type_rs = f'Vec<{entry_type_rs}>'
            uses.update(value_uses)
        elif len(loop_instructions) == 3:
            is_map = loop_instructions[0]['type'].startswith(
                'Map.Entry<')
            if is_map:
                assert loop_instructions[1]['field'].endswith(
                    '.getKey()')
                assert loop_instructions[2]['field'].endswith(
                    '.getValue()')

                # generate the type for the key
                key_type_rs, is_key_var, key_uses, key_extra_code = burger_type_to_rust_type(
                    loop_instructions[1]['type'], None, loop_instructions[1], mappings, obfuscated_class_name)
                uses.update(key_uses)
                extra_code.extend(key_extra_code)

                # generate the type for the value
                value_type_rs, is_value_var, value_uses, value_extra_code = burger_type_to_rust_type(
                    loop_instructions[2]['type'], None, loop_instructions[2], mappings, obfuscated_class_name)
                uses.update(value_uses)
                extra_code.extend(value_extra_code)

                field_type_rs = f'HashMap<{key_type_rs}, {value_type_rs}>'
                uses.add('std::collections::HashMap')

                # only the key is var since the value can be made var in other ways
                is_var = is_key_var

        skip = 2  # skip the next 2 instructions

    # Option<T>
    elif instruction['operation'] == 'write' and (instruction['field'].endswith('.isPresent()') or instruction['field'].endswith(' != null')) and next_instruction and (next_instruction.get('condition', '').endswith('.isPresent()') or next_instruction.get('condition', '').endswith(' != null')):
        print('ok is option')
        obfuscated_field_name = instruction['field'].split('.')[
            0].split(' ')[0]
    
        if obfuscated_field_name in known_variable_types:
            # just use the known name since it's not gonna be in the mappings
            obfuscated_field_name = known_variable_types[obfuscated_field_name]
    
        field_name = mappings.get_field(
            obfuscated_class_name, obfuscated_field_name)

        if field_name is None: field_name = obfuscated_field_name.split('/')[-1]
        if '<' in field_name:
            field_name = 'value'

        condition_instructions = next_instruction['instructions']

        condition_types_rs = []
        for condition_instruction in condition_instructions:
            print('condition_instruction', condition_instruction)
            if 'type' not in condition_instruction:
                # weird type, maybe it's a loop or something
                condition_types_rs.append('todo!("weird type, maybe it\'s a loop or something")')
                continue
            condition_type_rs, is_var, this_uses, this_extra_code = burger_type_to_rust_type(
                condition_instruction['type'], None, condition_instruction, mappings, obfuscated_class_name)
            condition_types_rs.append(condition_type_rs)
            uses.update(this_uses)
            extra_code.extend(this_extra_code)
        field_type_rs = f'Option<({", ".join(condition_types_rs)})>' if len(
            condition_types_rs) != 1 else f'Option<{condition_types_rs[0]}>'
        skip = 1
    else:
        field_type = instruction['type']
        obfuscated_field_name = instruction['field']

        if obfuscated_field_name.startswith('(float)'):
            obfuscated_field_name = obfuscated_field_name[len('(float)'):]

        field_name = mappings.get_field(
            obfuscated_class_name, obfuscated_field_name) or mappings.get_field(
            obfuscated_class_name.split('$')[0], obfuscated_field_name)

        field_type_rs, is_var, instruction_uses, instruction_extra_code = burger_type_to_rust_type(
            field_type, field_name, instruction, mappings, obfuscated_class_name)

        if obfuscated_field_name in known_variable_types:
            # just use the known name since it's not gonna be in the mappings
            field_name = obfuscated_field_name

        elif '.' in obfuscated_field_name or ' ' in obfuscated_field_name or '(' in obfuscated_field_name:
            field_type_rs2, obfuscated_field_name, field_comment = burger_field_to_type(
                obfuscated_field_name, mappings, obfuscated_class_name, known_variable_types)
            if not field_type_rs2:
                generated_packet_code.append(f'// TODO: {instruction}')
                return
            if obfuscated_field_name in known_variable_types:
                # just use the known name since it's not gonna be in the mappings
                obfuscated_field_name = known_variable_types[obfuscated_field_name]
                print('got obfuscated_field_name', obfuscated_field_name)

            # try to get the field name again with the new stuff we know
            field_name = mappings.get_field(
                obfuscated_class_name, obfuscated_field_name) or mappings.get_field(
                obfuscated_class_name.split('$')[0], obfuscated_field_name)
            if field_name is None:
                field_name = obfuscated_field_name.split('/')[-1]
        uses.update(instruction_uses)
        extra_code.extend(instruction_extra_code)

    if not field_name:
        generated_packet_code.append(
            f'// TODO: unknown field {instruction}')
        return skip

    if is_var:
        generated_packet_code.append('#[var]')
    line = f'pub {to_snake_case(field_name)}: {field_type_rs or "todo!()"},'
    if field_comment:
        line += f' // {field_comment}'
    generated_packet_code.append(line)

    return skip


def burger_field_to_type(field, mappings: Mappings, obfuscated_class_name: str, known_variable_types={}) -> tuple[Optional[str], str, Optional[str]]:
    '''
    Returns field_type_rs, obfuscated_field_name, field_comment
    '''
    # match `(x) ? 1 : 0`
    match = re.match(r'\((.*)\) \? 1 : 0', field)
    if match:
        return ('bool', match.group(1), None)
    match = re.match(r'^\w+\.\w+\(\)$', field)
    if match:
        print('field', field)
        obfuscated_first = field.split('.')[0]
        obfuscated_second = field.split('.')[1].split('(')[0]
        # first = mappings.get_field(obfuscated_class_name, obfuscated_first)
        if obfuscated_first in known_variable_types:
            first_type = known_variable_types[obfuscated_first]
        else:
            try:
                first_type = mappings.get_field_type(
                    obfuscated_class_name, obfuscated_first)
            except:
                first_type = 'TODO'
        first_obfuscated_class_name: Optional[str] = mappings.get_class_from_deobfuscated_name(
            first_type)
        if first_obfuscated_class_name:
            try:
                second = mappings.get_method(
                    first_obfuscated_class_name, obfuscated_second, '')
            except:
                # if this happens then the field is probably from a super class
                second = obfuscated_second
        else:
            second = obfuscated_second
        first_type_short = first_type.split('.')[-1]
        if second in {'byteValue'}:
            return (first_type_short, obfuscated_first, None)
        return (first_type_short, obfuscated_first, f'TODO: Does {first_type_short}::{second}, may not be implemented')
    return None, field, None


def change_packet_ids(id_map: dict[int, int], direction: str, state: str):
    existing_packet_ids, existing_packet_class_names = get_packets(
        direction, state)

    new_packet_ids = []

    for packet_id in existing_packet_ids:
        new_packet_id = id_map.get(packet_id, packet_id)
        if new_packet_id in new_packet_ids:
            raise Exception('Two packets have the same id')
        new_packet_ids.append(new_packet_id)

    set_packets(new_packet_ids, existing_packet_class_names, direction, state)


def remove_packet_ids(removing_packet_ids: list[int], direction: str, state: str):
    existing_packet_ids, existing_packet_class_names = get_packets(
        direction, state)

    new_packet_ids = []
    new_packet_class_names = []

    for packet_id, packet_class_name in zip(existing_packet_ids, existing_packet_class_names):
        if packet_id in removing_packet_ids:
            try:
                os.remove(
                    f'../azalea-protocol/src/packets/{state}/{packet_class_name}.rs')
            except:
                pass
        else:
            new_packet_ids.append(packet_id)
            new_packet_class_names.append(packet_class_name)

    set_packets(new_packet_ids, new_packet_class_names, direction, state)


def are_packet_instructions_identical(old_packet, new_packet):
    old_packet = old_packet or []
    new_packet = new_packet or []

    if len(old_packet) != len(new_packet):
        return False

    for old_field, new_field in zip(old_packet, new_packet):
        if old_field['operation'] != new_field['operation']:
            return False
        if new_field['operation'] == 'write':
            if burger_type_to_rust_type(old_field.get('type')) != burger_type_to_rust_type(new_field.get('type')):
                return False
        else:
            # comparing is too complicated here since it's possible the type has variables
            # so we just don't
            pass

        if 'instructions' in old_field and 'instructions' in new_field:
            if not are_packet_instructions_identical(old_field['instructions'], new_field['instructions']):
                return False

    return True
