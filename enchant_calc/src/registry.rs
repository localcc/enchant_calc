use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry<'a> {
    #[serde(borrow)]
    pub enchants: HashMap<&'a str, Enchant>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enchant {
    pub name: String,
    #[serde(rename = "levelMax")]
    pub level_max: u32,
    pub weight: u32,
    pub incompatible: Vec<String>,
    pub items: Vec<Item>,
}

impl Enchant {
    pub fn levels_required(&self, enchant_level: u32) -> u32 {
        enchant_level * self.weight
    }
}

#[derive(Default, Debug, Copy, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Item {
    #[default]
    None,
    Helmet,
    Chestplate,
    Leggings,
    Boots,
    TurtleShell,
    Elytra,
    Sword,
    Axe,
    Trident,
    Pickaxe,
    Shovel,
    Hoe,
    Bow,
    Shield,
    Crossbow,
    FishingRod,
    Shears,
    ShearsBedrock,
    FlintAndSteel,
    Compass,
    CarrotOnAStick,
    Pumpkin,
    WarpedFungusOnAStick,
}

impl Display for Item {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Item::None => write!(f, "None"),
            Item::Helmet => write!(f, "Helmet"),
            Item::Chestplate => write!(f, "Chestplate"),
            Item::Leggings => write!(f, "Leggings"),
            Item::Boots => write!(f, "Boots"),
            Item::TurtleShell => write!(f, "Turtle Shell"),
            Item::Elytra => write!(f, "Elytra"),
            Item::Sword => write!(f, "Sword"),
            Item::Axe => write!(f, "Axe"),
            Item::Trident => write!(f, "Trident"),
            Item::Pickaxe => write!(f, "Pickaxe"),
            Item::Shovel => write!(f, "Shovel"),
            Item::Hoe => write!(f, "Hoe"),
            Item::Bow => write!(f, "Bow"),
            Item::Shield => write!(f, "Shield"),
            Item::Crossbow => write!(f, "Crossbow"),
            Item::FishingRod => write!(f, "Fishing Rod"),
            Item::Shears => write!(f, "Shears"),
            Item::ShearsBedrock => write!(f, "Shears (Bedrock Edition)"),
            Item::FlintAndSteel => write!(f, "Flint And Steel"),
            Item::Compass => write!(f, "Compass"),
            Item::CarrotOnAStick => write!(f, "Carrot On A Stick"),
            Item::Pumpkin => write!(f, "Pumpkin"),
            Item::WarpedFungusOnAStick => write!(f, "Warped Fungus On A Stick"),
        }
    }
}

const REGISTRY_JSON: &str = r#"
{
    "enchants": {
      "Projectile Protection": {
        "name": "Projectile Protection",
        "levelMax": 4,
        "weight": 1,
        "incompatible": [
          "Protection",
          "Blast Protection",
          "Fire Protection"
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "turtle_shell"
        ]
      },
      "Fortune": {
        "name": "Fortune",
        "levelMax": 3,
        "weight": 2,
        "incompatible": [
          "Silk Touch"
        ],
        "items": [
          "pickaxe",
          "shovel",
          "axe",
          "hoe"
        ]
      },
      "Curse of Binding": {
        "name": "Curse of Binding",
        "levelMax": 1,
        "weight": 4,
        "incompatible": [
          ""
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "elytra",
          "pumpkin",
          "turtle_shell"
        ]
      },
      "Knockback": {
        "name": "Knockback",
        "levelMax": 2,
        "weight": 1,
        "incompatible": [
          ""
        ],
        "items": [
          "sword"
        ]
      },
      "Blast Protection": {
        "name": "Blast Protection",
        "levelMax": 4,
        "weight": 2,
        "incompatible": [
          "Fire Protection",
          "Protection",
          "Projectile Protection"
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "turtle_shell"
        ]
      },
      "Sharpness": {
        "name": "Sharpness",
        "levelMax": 5,
        "weight": 1,
        "incompatible": [
          "Bane of Arthropods",
          "Smite"
        ],
        "items": [
          "sword",
          "axe"
        ]
      },
      "Bane of Arthropods": {
        "name": "Bane of Arthropods",
        "levelMax": 5,
        "weight": 1,
        "incompatible": [
          "Smite",
          "Sharpness"
        ],
        "items": [
          "sword",
          "axe"
        ]
      },
      "Sweeping Edge": {
        "name": "Sweeping Edge",
        "levelMax": 3,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "sword"
        ]
      },
      "Power": {
        "name": "Power",
        "levelMax": 5,
        "weight": 1,
        "incompatible": [
          ""
        ],
        "items": [
          "bow"
        ]
      },
      "Looting": {
        "name": "Looting",
        "levelMax": 3,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "sword"
        ]
      },
      "Aqua Affinity": {
        "name": "Aqua Affinity",
        "levelMax": 1,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "helmet",
          "turtle_shell"
        ]
      },
      "Feather Falling": {
        "name": "Feather Falling",
        "levelMax": 4,
        "weight": 1,
        "incompatible": [
          ""
        ],
        "items": [
          "boots"
        ]
      },
      "Fire Aspect": {
        "name": "Fire Aspect",
        "levelMax": 2,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "sword"
        ]
      },
      "Fire Protection": {
        "name": "Fire Protection",
        "levelMax": 4,
        "weight": 1,
        "incompatible": [
          "Blast Protection",
          "Protection",
          "Projectile Protection"
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "turtle_shell"
        ]
      },
      "Infinity": {
        "name": "Infinity",
        "levelMax": 1,
        "weight": 4,
        "incompatible": [
          "Mending"
        ],
        "items": [
          "bow"
        ]
      },
      "Efficiency": {
        "name": "Efficiency",
        "levelMax": 5,
        "weight": 1,
        "incompatible": [
          ""
        ],
        "items": [
          "pickaxe",
          "shovel",
          "axe",
          "hoe",
          "shears"
        ]
      },
      "Channeling": {
        "name": "Channeling",
        "levelMax": 1,
        "weight": 4,
        "incompatible": [
          "Riptide",
          "Loyalty"
        ],
        "items": [
          "trident"
        ]
      },
      "Luck of the Sea": {
        "name": "Luck of the Sea",
        "levelMax": 3,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "fishing_rod"
        ]
      },
      "Lure": {
        "name": "Lure",
        "levelMax": 3,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "fishing_rod"
        ]
      },
      "Flame": {
        "name": "Flame",
        "levelMax": 1,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "bow"
        ]
      },
      "Impaling": {
        "name": "Impaling",
        "levelMax": 5,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "trident"
        ]
      },
      "Frost Walker": {
        "name": "Frost Walker",
        "levelMax": 2,
        "weight": 2,
        "incompatible": [
          "Depth Strider"
        ],
        "items": [
          "boots"
        ]
      },
      "Loyalty": {
        "name": "Loyalty",
        "levelMax": 3,
        "weight": 1,
        "incompatible": [
          "Riptide",
          "Channeling"
        ],
        "items": [
          "trident"
        ]
      },
      "Piercing": {
        "name": "Piercing",
        "levelMax": 4,
        "weight": 1,
        "incompatible": [
          "Multishot"
        ],
        "items": [
          "crossbow"
        ]
      },
      "Quick Charge": {
        "name": "Quick Charge",
        "levelMax": 3,
        "weight": 1,
        "incompatible": [
          ""
        ],
        "items": [
          "crossbow"
        ]
      },
      "Unbreaking": {
        "name": "Unbreaking",
        "levelMax": 3,
        "weight": 1,
        "incompatible": [
          ""
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "pickaxe",
          "shovel",
          "axe",
          "sword",
          "hoe",
          "fishing_rod",
          "bow",
          "shears",
          "flint_and_steel",
          "compass",
          "carrot_on_a_stick",
          "warped_fungus_on_a_stick",
          "shield",
          "elytra",
          "trident",
          "turtle_shell",
          "crossbow"
        ]
      },
      "Curse of Vanishing": {
        "name": "Curse of Vanishing",
        "levelMax": 1,
        "weight": 4,
        "incompatible": [
          ""
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "pickaxe",
          "shovel",
          "axe",
          "sword",
          "hoe",
          "fishing_rod",
          "bow",
          "shears",
          "flint_and_steel",
          "compass",
          "carrot_on_a_stick",
          "warped_fungus_on_a_stick",
          "shield",
          "elytra",
          "pumpkin",
          "helmet",
          "trident",
          "turtle_shell",
          "crossbow"
        ]
      },
      "Multishot": {
        "name": "Multishot",
        "levelMax": 1,
        "weight": 2,
        "incompatible": [
          "Piercing"
        ],
        "items": [
          "crossbow"
        ]
      },
      "Thorns": {
        "name": "Thorns",
        "levelMax": 3,
        "weight": 4,
        "incompatible": [
          ""
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "turtle_shell"
        ]
      },
      "Depth Strider": {
        "name": "Depth Strider",
        "levelMax": 3,
        "weight": 2,
        "incompatible": [
          "Frost Walker"
        ],
        "items": [
          "boots"
        ]
      },
      "Protection": {
        "name": "Protection",
        "levelMax": 4,
        "weight": 1,
        "incompatible": [
          "Blast Protection",
          "Fire Protection",
          "Projectile Protection"
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "turtle_shell"
        ]
      },
      "Punch": {
        "name": "Punch",
        "levelMax": 2,
        "weight": 2,
        "incompatible": [
          ""
        ],
        "items": [
          "bow"
        ]
      },
      "Riptide": {
        "name": "Riptide",
        "levelMax": 3,
        "weight": 2,
        "incompatible": [
          "Channeling",
          "Loyalty"
        ],
        "items": [
          "trident"
        ]
      },
      "Smite": {
        "name": "Smite",
        "levelMax": 5,
        "weight": 1,
        "incompatible": [
          "Bane of Arthropods",
          "Sharpness"
        ],
        "items": [
          "sword",
          "axe"
        ]
      },
      "Mending": {
        "name": "Mending",
        "levelMax": 1,
        "weight": 2,
        "incompatible": [
          "Infinity"
        ],
        "items": [
          "helmet",
          "chestplate",
          "leggings",
          "boots",
          "pickaxe",
          "shovel",
          "axe",
          "sword",
          "hoe",
          "fishing_rod",
          "bow",
          "shears",
          "flint_and_steel",
          "compass",
          "carrot_on_a_stick",
          "warped_fungus_on_a_stick",
          "shield",
          "elytra",
          "trident",
          "turtle_shell",
          "crossbow"
        ]
      },
      "Silk Touch": {
        "name": "Silk Touch",
        "levelMax": 1,
        "weight": 4,
        "incompatible": [
          "Fortune"
        ],
        "items": [
          "pickaxe",
          "turtle_shell",
          "shears_bedrock"
        ]
      },
      "Soul Speed": {
        "name": "Soul Speed",
        "levelMax": 3,
        "weight": 4,
        "incompatible": [
          ""
        ],
        "items": [
          "boots"
        ]
      }
    }
  }
"#;

fn load_registry() -> Registry<'static> {
    serde_json::from_str(REGISTRY_JSON).unwrap()
}

lazy_static! {
    pub static ref REGISTRY: Registry<'static> = load_registry();
}
