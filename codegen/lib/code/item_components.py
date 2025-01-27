import lib.code.inventory
import lib.code.registry
import lib.code.version
import lib.code.packet
import lib.code.utils
import lib.code.tags
import lib.download
import lib.extract
import lib.utils


ITEM_COMPONENTS_DIR = 'azalea-inventory/src/components.rs'

def generate(version_id: str):
    expected_variants = get_expected_variants(version_id)
    actual_variants = get_actual_variants()

    new_variants = []
    removed_variants = []

    for variant in expected_variants:
        if variant not in actual_variants:
            new_variants.append(variant)
    for variant in actual_variants:
        if variant not in expected_variants:
            removed_variants.append(variant)

    print('New variants:')
    for variant in new_variants:
        print('-', variant)
    print()
    print('Removed variants:')
    for variant in removed_variants:
        print('-', variant)
    print()

    for variant in removed_variants:
        print(f'Removing {variant}...')
        remove_variant(variant)
    for variant in new_variants:
        print(f'Adding {variant}...')
        add_variant(variant)

    lib.code.utils.fmt()

    print('Done!')

def get_expected_variants(version_id: str):
    expected_variants = []
    registries = lib.extract.get_registries_report(version_id)

    registry = registries['minecraft:data_component_type']
    registry_entries = sorted(
        registry['entries'].items(), key=lambda x: x[1]['protocol_id'])
    for variant_name, _variant in registry_entries:
        variant_struct_name = lib.utils.to_camel_case(variant_name.split(':')[-1])
        expected_variants.append(variant_struct_name)

    return expected_variants

def get_actual_variants():
    actual_variants = []
    with open(ITEM_COMPONENTS_DIR, 'r') as f:
        code = f.read().split('\n')

    in_match = False
    for line in code:
        if in_match:
            if line == '    })':
                break
            variant_line_prefix = '        DataComponentKind::'
            if line.startswith(variant_line_prefix):
                variant = line[len(variant_line_prefix):].split(' ', 1)[0]
                actual_variants.append(variant)
        elif line == '    Ok(match kind {':
            in_match = True

    return actual_variants

def remove_variant(variant: str):
    with open(ITEM_COMPONENTS_DIR, 'r') as f:
        code = f.read().split('\n')

    first_line_with_variant = None
    line_after_variant = None

    in_match = False
    for i, line in enumerate(list(code)):
        if in_match:
            if line == '    })':
                line_after_variant = i
                break
            variant_line_prefix = '        DataComponentKind::'
            if line.startswith(variant_line_prefix):
                if first_line_with_variant is not None:
                    line_after_variant = i
                    break
                variant_name = line[len(variant_line_prefix):].split(' ', 1)[0]
                if variant_name == variant:
                    first_line_with_variant = i
        elif line == '    Ok(match kind {':
            in_match = True
    
    if first_line_with_variant is None:
        raise ValueError(f'Variant {variant} not found')
    if line_after_variant is None:
        raise ValueError(f'Couldn\'t find end of variant {variant}')

    code = code[:first_line_with_variant] + code[line_after_variant:]

    # now remove the struct
    line_before_struct = None # this is the #[derive] line
    line_after_struct = None # impl DataComponent for ... {\n...\n}
    for i, line in enumerate(list(code)):
        if line == f'pub struct {variant} {{' or line == f'pub struct {variant};':
            line_before_struct = i - 1
        elif line == f'impl DataComponent for {variant} {{':
            line_after_struct = i + 3
            break
    if line_before_struct is None:
        raise ValueError(f'Couldn\'t find struct {variant}')
    if line_after_struct is None:
        raise ValueError(f'Couldn\'t find impl DataComponent for {variant}')
    
    code = code[:line_before_struct] + code[line_after_struct:]

    with open(ITEM_COMPONENTS_DIR, 'w') as f:
        f.write('\n'.join(code))

def add_variant(variant: str):
    with open(ITEM_COMPONENTS_DIR, 'r') as f:
        code = f.read().split('\n')

    in_match = False
    last_line_in_match = None
    for i, line in enumerate(list(code)):
        if in_match:
            if line == '    })':
                last_line_in_match = i
                break
        elif line == '    Ok(match kind {':
            in_match = True

    if last_line_in_match is None:
        raise ValueError('Couldn\'t find end of match')
    
    code = code[:last_line_in_match] + [
        f'        DataComponentKind::{variant} => Box::new({variant}::azalea_read(buf)?),',
    ] + code[last_line_in_match:]

    # now insert the struct
    code.append('')
    code.append('#[derive(Clone, PartialEq, AzBuf)]')
    code.append(f'pub struct {variant} {{')
    code.append('   pub todo: todo!(), // see DataComponents.java')
    code.append('}')
    code.append(f'impl DataComponent for {variant} {{')
    code.append(f'    const KIND: DataComponentKind = DataComponentKind::{variant};')
    code.append('}')

    with open(ITEM_COMPONENTS_DIR, 'w') as f:
        f.write('\n'.join(code))

    lib.code.utils.fmt()
