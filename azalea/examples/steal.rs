//! Steal all the diamonds from all the nearby chests.

use std::sync::Arc;

use azalea::{BlockPos, prelude::*};
use azalea_inventory::ItemStack;
use azalea_inventory::operations::QuickMoveClick;
use parking_lot::Mutex;

#[tokio::main]
async fn main() {
    let account = Account::offline("bot");
    // or let bot = Account::microsoft("email").await.unwrap();

    ClientBuilder::new()
        .set_handler(handle)
        .start(account, "localhost")
        .await
        .unwrap();
}

#[derive(Default, Clone, Component)]
struct State {
    pub checked_chests: Arc<Mutex<Vec<BlockPos>>>,
}

async fn handle(bot: Client, event: Event, state: State) -> anyhow::Result<()> {
    if let Event::Chat(m) = event {
        if m.sender() == Some(bot.profile.name.clone()) {
            return Ok(());
        };
        if m.content() != "go" {
            return Ok(());
        }
        {
            state.checked_chests.lock().clear();
        }

        let chest_block = bot
            .world()
            .read()
            .find_block(bot.position(), &azalea::registry::Block::Chest.into());
        // TODO: update this when find_blocks is implemented
        let Some(chest_block) = chest_block else {
            bot.chat("No chest found");
            return Ok(());
        };
        // bot.goto(BlockPosGoal(chest_block));
        let Some(chest) = bot.open_container_at(chest_block).await else {
            println!("Couldn't open chest");
            return Ok(());
        };

        println!("Getting contents");
        for (index, slot) in chest
            .contents()
            .expect("we just opened the chest")
            .iter()
            .enumerate()
        {
            println!("Checking slot {index}: {slot:?}");
            if let ItemStack::Present(item) = slot {
                if item.kind == azalea::registry::Item::Diamond {
                    println!("clicking slot ^");
                    chest.click(QuickMoveClick::Left { slot: index as u16 });
                }
            }
        }

        println!("Done");
    }

    Ok(())
}
