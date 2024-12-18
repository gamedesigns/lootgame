use bevy::prelude::*;
use crate::components::player_components::Player;
use crate::components::loot_box_components::LootBox;
use crate::components::item_components::Item;

pub fn ui_system(
    mut commands: Commands,
    player_query: Query<&Player>,
    loot_box_query: Query<(Entity, &LootBox)>,
    item_query: Query<(Entity, &Item)>,
) {
    let player = player_query.single();
    let loot_boxes: Vec<(Entity, &LootBox)> = loot_box_query.iter().collect();
    let items: Vec<(Entity, &Item)> = item_query.iter().collect();

    // Clear the screen
    println!("\x1B[2J\x1B[1;1H");

    // Display player stats
    println!("Player Stats:");
    println!("Level: {}", player.level);
    println!("Money: {}", player.money);
    println!("Score: {}", player.score);

    // Display loot boxes
    println!("\nLoot Boxes:");
    for (i, (loot_box_entity, loot_box)) in loot_boxes.iter().enumerate() {
        println!("{}: {} Loot Box", i + 1, loot_box.rarity);
        for item_entity in &loot_box.items {
            if let Ok(item) = commands.get_entity(*item_entity) {
                println!("  - {}: {} (Value: {}, Score Bonus: {})", item.category, item.rarity, item.value, item.score_bonus);
            }
        }
    }

    // Display equipped items
    println!("\nEquipped Items:");
    for (item_entity, item) in items.iter() {
        println!("{}: {} (Value: {}, Score Bonus: {})", item.category, item.rarity, item.value, item.score_bonus);
    }

    // Handle user input
    println!("\nChoose a loot box to open (1-3) or type 'exit' to quit:");
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let choice: usize = input.trim().parse().unwrap_or(0);

    if choice > 0 && choice <= loot_boxes.len() {
        let (loot_box_entity, _) = &loot_boxes[choice - 1];
        commands.entity(*loot_box_entity).despawn();
    } else if input.trim() == "exit" {
        std::process::exit(0);
    }
}
