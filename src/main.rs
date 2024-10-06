use std::{fs::File, io::Write};

use anyhow::{Context, Result};
use clap::Parser;
use rusqlite::{Connection};
use terra_items::{Item, ItemSlot, Prefix};
use terra_plr::{
    inventory::{
        AccessoryRow, ArmorRow, InventorySlot, Loadout, Loadouts, MiscRow, MiscRowWithVisibility,
        SingleItemSlot,
    },
    Player,
    SUPPORTED_VERSIONS
};
use terra_types::PositiveI32;

// open sqlite database from tshock

fn open_db(path: &str) -> Result<Connection> {
    let conn = Connection::open(path).context("unable to open db")?;
    Ok(conn)
}

fn parse_item(item: String) -> Result<ItemSlot> {
    let mut item_parts = item.split(",");
    let item_id: i32 = item_parts.next().context("no item id")?.parse()?;
    let count: i32 = item_parts
        .next()
        .context("cant load item count")?
        .parse::<i32>()
        .context("item count failed to parse")?;
    let prefix: u8 = item_parts
        .next()
        .context("can't load item prefix")?
        .parse()?;

    let item_slot = ItemSlot {
        item: Item::from_id(item_id, 1).context("item id is invalid")?,
        prefix: Prefix::from_id(prefix),
        count: PositiveI32::new(count).context("item count is negative")?,
    };

    Ok(item_slot)
}

fn parse_inventory(inventory: String) -> Result<Vec<Option<ItemSlot>>> {
    let mut item_slots: Vec<Option<ItemSlot>> = Vec::new();
    for item in inventory.split("~") {
        let item_slot = parse_item(item.to_string()).ok();
        item_slots.push(item_slot);
    }
    Ok(item_slots)
}



fn get_loadout(inventory: &[Option<ItemSlot>]) -> Loadout {
    Loadout {
        helmet: ArmorRow {
            armor: inventory.get(0).and_then(|item| {
                item.and_then(|item| Some(SingleItemSlot::from(item)))
            }),
            vanity_armor: inventory.get(10).and_then(|item| {
                item.and_then(|item| Some(SingleItemSlot::from(item)))
            }),
            dye: None,
        },
        breastplate: ArmorRow {
            armor: inventory.get(1).and_then(|item| {
                item.and_then(|item| Some(SingleItemSlot::from(item)))
            }),
            vanity_armor: inventory.get(11).and_then(|item| {
                item.and_then(|item| Some(SingleItemSlot::from(item)))
            }),
            dye: None,
        },
        pants: ArmorRow {
            armor: inventory.get(2).and_then(|item| {
                item.and_then(|item| Some(SingleItemSlot::from(item)))
            }),
            vanity_armor: inventory.get(12).and_then(|item| {
                item.and_then(|item| Some(SingleItemSlot::from(item)))
            }),
            dye: None,
        },
        accessories: [
            AccessoryRow {
                accessory: inventory.get(3).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                vanity_accessory: inventory.get(13).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                dye: None,
                is_accessory_shown: true,
            },
            AccessoryRow {
                accessory: inventory.get(4).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                vanity_accessory: inventory.get(14).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                dye: None,
                is_accessory_shown: true,
            },
            AccessoryRow {
                accessory: inventory.get(5).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                vanity_accessory: inventory.get(15).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                dye: None,
                is_accessory_shown: true,
            },
            AccessoryRow {
                accessory: inventory.get(6).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                vanity_accessory: inventory.get(16).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                dye: None,
                is_accessory_shown: true,
            },
            AccessoryRow {
                accessory: inventory.get(7).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                vanity_accessory: inventory.get(17).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                dye: None,
                is_accessory_shown: true,
            },
            AccessoryRow {
                accessory: inventory.get(8).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                vanity_accessory: inventory.get(18).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                dye: None,
                is_accessory_shown: true,
            },
            AccessoryRow {
                accessory: inventory.get(9).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                vanity_accessory: inventory.get(19).and_then(|item| {
                    item.and_then(|item| Some(SingleItemSlot::from(item)))
                }),
                dye: None,
                is_accessory_shown: true,
            }
        ]
    }
}

fn get_player(mut player: Player, conn: &Connection, name: &str) -> Result<Player> {
    let mut stmt = conn.prepare(
        "
        select
            u.Username,
            Health, 
            MaxHealth, 
            Mana,
            MaxMana,
            Inventory,
            hair
            hairDye,
            hairColor,
            pantsColor,
            shirtColor,
            underShirtColor,
            shoeColor,
            skinColor,
            eyeColor,
            questsCompleted,
            unlockedBiomeTorches,
            ateArtisanBread,
            usedAegisCrystal,
            usedAegisFruit,
            usedArcaneCrystal,
            usedGalaxyPearl,
            usedGummyWorm,
            usedAmbrosia,
            unlockedSuperCart,
            enabledSuperCart
        from Users u
        left join tsCharacter c on c.Account = u.ID
        where u.Username = ?1
    ",
    )?;



    let player = stmt.query_row(&[&name], |row| {
        // map the row to a terra_plr Player struct
        player.name = row.get(0)?;
        player.life = row.get(1)?;
        player.max_life = row.get(2)?;
        player.mana = row.get(3)?;
        player.max_mana = row.get(4)?;

        let inventory = parse_inventory(row.get(5)?)
            .context("failed to parse inventory")
            .expect("failed to parse inventory");

        // inventory
        //     .clone()
        //     .into_iter()
        //     .enumerate()
        //     .for_each(|(i, item)| {
        //         println!("item: {i}: {item:?}");
        //     });

        // player.inventory is a 2d array of InventorySlot objects, 10x5
        // which we can get from the inventory vec we just parsed

        let inventory_arrays: Vec<[Option<InventorySlot>; 10]> = inventory
            .chunks(10)
            .map(|chunk| {
                let mut row = [None; 10];
                for (i, item) in chunk.iter().enumerate() {
                    let inventory_item: Option<InventorySlot> = match item {
                        Some(item) => Some(InventorySlot::from(item.clone())),
                        None => None,
                    };
                    row[i] = inventory_item;
                }
                row
            })
            .collect();

        player.inventory = [
            inventory_arrays[0],
            inventory_arrays[1],
            inventory_arrays[2],
            inventory_arrays[3],
            inventory_arrays[4],
        ];

        let piggy_bank_items = inventory
            .get(99..149)
            .context("no piggy bank")
            .expect("no piggy bank");
        let piggy_bank_arrays: Vec<[Option<ItemSlot>; 10]> = piggy_bank_items
            .chunks(10)
            .map(|chunk| {
                let mut row = [None; 10];
                for (i, item) in chunk.iter().enumerate() {
                    row[i] = *item;
                }
                row
            })
            .collect();

        player.piggy_bank = [
            piggy_bank_arrays[0],
            piggy_bank_arrays[1],
            piggy_bank_arrays[2],
            piggy_bank_arrays[3],
        ];

        let forge_items = inventory
            .get(199..249)
            .context("no forge")
            .expect("no forge");
        let forge_arrays: Vec<[Option<ItemSlot>; 10]> = forge_items
            .chunks(10)
            .map(|chunk| {
                let mut row = [None; 10];
                for (i, item) in chunk.iter().enumerate() {
                    row[i] = *item;
                }
                row
            })
            .collect();

        player.defenders_forge = [
            forge_arrays[0],
            forge_arrays[1],
            forge_arrays[2],
            forge_arrays[3],
        ];

        let safe_items = inventory.get(139..189).context("no safe").expect("no safe");
        let forge_arrays: Vec<[Option<ItemSlot>; 10]> = safe_items
            .chunks(10)
            .map(|chunk| {
                let mut row = [None; 10];
                for (i, item) in chunk.iter().enumerate() {
                    row[i] = *item;
                }
                row
            })
            .collect();

        player.safe = [
            forge_arrays[0],
            forge_arrays[1],
            forge_arrays[2],
            forge_arrays[3],
        ];

        player.loadouts = Loadouts {
            loadouts: [
                get_loadout(inventory.get(59..79).expect("no loadout 1")),
                get_loadout(inventory.get(290..310).expect("no loadout 2")),
                get_loadout(inventory.get(320..340).expect("no loadout 3"))
            ],
            selected_loadout_index: 0,
        };
        player.pet = MiscRowWithVisibility {
            item: inventory
                .get(89)
                .and_then(|item| item.and_then(|item| Some(SingleItemSlot::from(item)))),
            dye: None,
            is_shown: true,
        };

        player.light_pet = MiscRowWithVisibility {
            item: inventory
                .get(90)
                .and_then(|item| item.and_then(|item| Some(SingleItemSlot::from(item)))),
            dye: None,
            is_shown: true,
        };

        player.minecart = MiscRow {
            item: inventory
                .get(91)
                .and_then(|item| item.and_then(|item| Some(SingleItemSlot::from(item)))),
            dye: None,
        };

        player.mount = MiscRow {
            item: inventory
                .get(92)
                .and_then(|item| item.and_then(|item| Some(SingleItemSlot::from(item)))),
            dye: None,
        };

        player.hook = MiscRow {
            item: inventory
                .get(93)
                .and_then(|item| item.and_then(|item| Some(SingleItemSlot::from(item)))),
            dye: None,
        };

        let ammo_items = inventory.get(54..58).context("no ammo").expect("no ammo");
        let ammo_array = [
            ammo_items.get(0).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
            ammo_items.get(1).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
            ammo_items.get(2).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
            ammo_items.get(3).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
        ];

        player.ammo = ammo_array;


        let coin_items = inventory.get(49..53).context("no coins").expect("no coins");
        let coin_array = [
            coin_items.get(0).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
            coin_items.get(1).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
            coin_items.get(2).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
            coin_items.get(3).and_then(|item| item.and_then(|item| Some(InventorySlot::from(item)))),
        ];

        player.coins = coin_array;

        // player.hair_style = row.get(10)?.parse()?;
        // player.hair_dye = row.get(11)?;
        // player.hair_color = row.get(12)?;
        // player.pants_color = row.get(13)?;
        // player.shirt_color = row.get(14)?;
        // player.under_shirt_color = row.get(15)?;
        // player.shoe_color = row.get(16)?;
        // player.skin_color = row.get(18)?;
        // player.eye_color = row.get(19)?;
        player.finished_angler_quests_count = row.get(20)?;
        player.is_using_biome_torches = match row.get(23) {
            Ok(1) => Some(true),
            _ => Some(false),
        };
        player.is_artisan_bread_eaten = match row.get(25) {
            Ok(1) => true,
            _ => false,
        };
        player.is_aegis_crystal_used = match row.get(26) {
            Ok(1) => true,
            _ => false,
        };
        player.is_aegis_fruit_used = match row.get(27) {
            Ok(1) => true,
            _ => false,
        };
        player.is_arcane_crystal_used = match row.get(28) {
            Ok(1) => true,
            _ => false,
        };
        player.is_galaxy_pearl_used = match row.get(29) {
            Ok(1) => true,
            _ => false,
        };
        player.is_gummy_worm_used = match row.get(30) {
            Ok(1) => true,
            _ => false,
        };
        player.is_ambrosia_used = match row.get(31) {
            Ok(1) => true,
            _ => false,
        };
        player.is_super_cart_enabled = match row.get(32) {
            Ok(1) => Some(true),
            _ => Some(false),
        };
        player.is_super_cart_enabled = match row.get(33) {
            Ok(1) => Some(true),
            _ => Some(false),
        };
        Ok(player)
    })?;
    // dbg!(&player);
    Ok(player)
}

#[derive(Parser, Debug)]
#[command(version, about, long_about=None)]
struct MyArgs {
    #[clap(short, long)]
    name: String,
    #[clap(short, long)]
    database: String,
    #[clap(short, long)]
    verbose: bool,
}

fn main() -> Result<()> {
    let args = MyArgs::parse();

    println!("args: {args:?}");

    let conn = open_db(&args.database).unwrap();
    let mut template = File::open(format!("Template.plr"))?;
    let player = Player::read_player(&mut template)?;
    let player = get_player(player, &conn, &args.name).unwrap();
    // dbg!(&player);


    let mut outfile = File::create(format!("{name}.plr", name=args.name))?;
    player.write_player_unencrypted(&mut outfile, *SUPPORTED_VERSIONS.end())?;
    outfile.flush()?;


    let mut written_player = File::open(format!("{name}.plr", name=args.name))?;
    let player = Player::read_player_unencrypted(&mut written_player)?;
    dbg!(&player); // this should be the same as we thought we just made

    Ok(())
}
