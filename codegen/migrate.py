import lib.code.item_components
import lib.code.inventory
import lib.code.language
import lib.code.registry
import lib.code.version
import lib.code.blocks
import lib.code.packet
import lib.code.shapes
import lib.code.entity
import lib.code.utils
import lib.download
import lib.extract
import sys

lib.download.clear_version_cache()

if len(sys.argv) == 1:
    print('\033[91mYou must provide a version to migrate to.\033[m')
    version_manifest = lib.download.get_version_manifest()
    newest_version = version_manifest['latest']['snapshot']
    print(f'Hint: newest version is \033[1m{newest_version}\033[m')
    exit()


old_version_id = lib.code.version.get_version_id()
old_mappings = lib.download.get_mappings_for_version(old_version_id)
old_burger_data = lib.extract.get_burger_data_for_version(old_version_id)

new_version_id = sys.argv[1]
new_mappings = lib.download.get_mappings_for_version(new_version_id)
new_burger_data = lib.extract.get_burger_data_for_version(new_version_id)

new_packets_report = lib.extract.get_packets_report(new_version_id)
lib.code.packet.set_packets(new_packets_report)

lib.code.version.set_protocol_version(
    new_burger_data[0]['version']['protocol'])
lib.code.version.set_version_name(new_version_id)

print('Updated protocol!')

print('Generating blocks and shapes...')
# TODO: pixlyzer is broken so we use old data
new_shape_datas = lib.extract.get_pixlyzer_data(
    '1.20.3-pre4', 'shapes')
new_pixlyzer_block_datas = lib.extract.get_pixlyzer_data(
    '1.20.3-pre4', 'blocks')
new_block_states_report = lib.extract.get_block_states_report(new_version_id)
new_registries = lib.extract.get_registries_report(new_version_id)
new_ordered_blocks = lib.code.blocks.get_ordered_blocks(new_registries)
lib.code.blocks.generate_blocks(
    new_block_states_report, new_pixlyzer_block_datas, new_ordered_blocks)
lib.code.shapes.generate_block_shapes(
    new_pixlyzer_block_datas, new_shape_datas['shapes'], new_shape_datas['aabbs'], new_block_states_report)

print('Getting en_us.json...')
language = lib.extract.get_en_us_lang(new_version_id)
lib.code.language.write_language(language)

print('Generating registries...')
import genregistries
genregistries.generate(new_version_id)

print('Generating entity metadata...')
burger_entities_data = new_burger_data[0]['entities']
lib.code.entity.generate_entity_metadata(burger_entities_data, new_mappings)

print('Generating item components...')
lib.code.item_components.generate(new_version_id)

print('Finishing touches, setting version in README and formatting code...')
lib.code.version.set_version_id(new_version_id)

lib.code.utils.fmt()

print('Done!')
print('Make sure to `cargo check` and look for the generated `TODO`s to make sure everything is correct!')
