use bevy::prelude::*;
use crate::components::loot_box_components::{LootBox, LootBoxRarity};
use crate::components::player_components::Player;
use crate::components::item_components::{Item, ItemRarity, ItemCategory};
use rand::Rng;

pub fn generate_loot_boxes_system(mut commands: Commands, player_query: Query<&Player>) {
    let player = player_query.single();
    let mut rng = rand::thread_rng();

    for _ in 0..3 {
        let rarity = match player.level {
            1..=5 => LootBoxRarity::Common,
            6..=10 => LootBoxRarity::Uncommon,
            11..=15 => LootBoxRarity::Rare,
            16..=20 => LootBoxRarity::Epic,
            _ => LootBoxRarity::Legendary,
        };

        let items = vec![
            commands.spawn().insert(Item {
                rarity: match rng.gen_range(0..5) {
                    0 => ItemRarity::Common,
                    1 => ItemRarity::Uncommon,
                    2 => ItemRarity::Rare,
                    3 => ItemRarity::Epic,
                    _ => ItemRarity::Legendary,
                },
                value: rng.gen_range(10..100),
                score_bonus: rng.gen_range(1..10),
                category: match rng.gen_range(0..6) {
                    0 => ItemCategory::BodyArmor,
                    1 => ItemCategory::Helmet,
                    2 => ItemCategory::Weapon,
                    3 => ItemCategory::Shield,
                    4 => ItemCategory::Potion,
                    _ => ItemCategory::Equipment,
                },
            }).id(),
        ];

        commands.spawn().insert(LootBox {
            rarity,
            items,
        });
    }
}
