use azalea_registry::Enchantment;

pub fn get_enchant_level(
    _enchantment: Enchantment,
    _player_inventory: &azalea_inventory::Menu,
) -> u32 {
    // TODO

    0
}

pub fn get_sneaking_speed_bonus(player_inventory: &azalea_inventory::Menu) -> f32 {
    get_enchant_level(Enchantment::SwiftSneak, player_inventory) as f32 * 0.15
}
