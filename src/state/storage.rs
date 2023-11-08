use cw_storage_plus::Item;

use super::models::Config;

pub const CONFIG: Item<Config> = Item::new("config");
