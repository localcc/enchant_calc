use lazy_static::lazy_static;
use serde::{de::Error, Deserialize, Deserializer, Serialize};
use std::{collections::HashMap, fmt::Display};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Registry<'a> {
    #[serde(borrow)]
    pub enchants: HashMap<&'a str, Enchant<'a>>,
}

fn string_to_int<'de, D>(deserializer: D) -> Result<u32, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(deserializer)?;

    s.parse::<u32>().map_err(D::Error::custom)
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Enchant<'a> {
    #[serde(skip)]
    pub name: &'a str,
    #[serde(rename = "levelMax")]
    #[serde(deserialize_with = "string_to_int")]
    pub level_max: u32,
    #[serde(deserialize_with = "string_to_int")]
    pub weight: u32,
    #[serde(borrow)]
    pub incompatible: Vec<&'a str>,
    pub items: Vec<Item>,
}

impl<'a> Enchant<'a> {
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
            Item::ShearsBedrock => write!(f, "Shears (Bedrock edition)"),
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
        "Protection": {
            "levelMax": "4",
            "weight": "1",
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

        "Aqua Affinity": {
            "levelMax": "1",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "helmet",
                "turtle_shell"
            ]
        },
        "Bane of Arthropods": {
            "levelMax": "5",
            "weight": "1",
            "incompatible": [
                "Smite",
                "Sharpness"
            ],
            "items": [
                "sword",
                "axe"
            ]
        },
        "Blast Protection": {
            "levelMax": "4",
            "weight": "2",
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
        "Channeling": {
            "levelMax": "1",
            "weight": "4",
            "incompatible": [
                "Riptide",
        "Loyalty"
            ],
            "items": [
                "trident"
            ]
        },
        "Depth Strider": {
            "levelMax": "3",
            "weight": "2",
            "incompatible": [
                "Frost Walker"
            ],
            "items": [
                "boots"
            ]
        },
        "Efficiency": {
            "levelMax": "5",
            "weight": "1",
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
        "Feather Falling": {
            "levelMax": "4",
            "weight": "1",
            "incompatible": [
                ""
            ],
            "items": [
                "boots"
            ]
        },
        "Fire Aspect": {
            "levelMax": "2",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "sword"
            ]
        },
        "Fire Protection": {
            "levelMax": "4",
            "weight": "1",
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
        "Flame": {
            "levelMax": "1",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "bow"
            ]
        },
        "Fortune": {
            "levelMax": "3",
            "weight": "2",
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
        "Frost Walker": {
            "levelMax": "2",
            "weight": "2",
            "incompatible": [
                "Depth Strider"
            ],
            "items": [
                "boots"
            ]
        },
        "Impaling": {
            "levelMax": "5",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "trident"
            ]
        },
        "Infinity": {
            "levelMax": "1",
            "weight": "4",
            "incompatible": [
                "Mending"
            ],
            "items": [
                "bow"
            ]
        },
        "Knockback": {
            "levelMax": "2",
            "weight": "1",
            "incompatible": [
                ""
            ],
            "items": [
                "sword"
            ]
        },
        "Looting": {
            "levelMax": "3",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "sword"
            ]
        },
        "Loyalty": {
            "levelMax": "3",
            "weight": "1",
            "incompatible": [
                "Riptide",
        "Channeling"
            ],
            "items": [
                "trident"
            ]
        },
        "Luck of the Sea": {
            "levelMax": "3",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "fishing_rod"
            ]
        },
        "Lure": {
            "levelMax": "3",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "fishing_rod"
            ]
        },
        "Mending": {
            "levelMax": "1",
            "weight": "2",
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
        "Multishot": {
            "levelMax": "1",
            "weight": "2",
            "incompatible": [
                "Piercing"
            ],
            "items": [
                "crossbow"
            ]
        },
        "Piercing": {
            "levelMax": "4",
            "weight": "1",
            "incompatible": [
                "Multishot"
            ],
            "items": [
                "crossbow"
            ]
        },
        "Power": {
            "levelMax": "5",
            "weight": "1",
            "incompatible": [
                ""
            ],
            "items": [
                "bow"
            ]
        },
        "Projectile Protection": {
            "levelMax": "4",
            "weight": "1",
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
        "Punch": {
            "levelMax": "2",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "bow"
            ]
        },
        "Quick Charge": {
            "levelMax": "3",
            "weight": "1",
            "incompatible": [
                ""
            ],
            "items": [
                "crossbow"
            ]
        },
        "Respiration": {
            "levelMax": "3",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "helmet",
                "turtle_shell"
            ]
        },
        "Riptide": {
            "levelMax": "3",
            "weight": "2",
            "incompatible": [
                "Channeling",
                "Loyalty"
            ],
            "items": [
                "trident"
            ]
        },
        "Sharpness": {
            "levelMax": "5",
            "weight": "1",
            "incompatible": [
                "Bane of Arthropods",
                "Smite"
            ],
            "items": [
                "sword",
        "axe"
            ]
        },
        "Silk Touch": {
            "levelMax": "1",
            "weight": "4",
            "incompatible": [
                "Fortune"
            ],
            "items": [
                "pickaxe",
                "shovel",
                "axe",
                "hoe",
                "shears_bedrock"
            ]
        },
        "Smite": {
            "levelMax": "5",
            "weight": "1",
            "incompatible": [
                "Bane of Arthropods",
                "Sharpness"
            ],
            "items": [
                "sword",
                "axe"
            ]
        },
        "Soul Speed": {
            "levelMax": "3",
            "weight": "4",
            "incompatible": [
                ""
            ],
            "items": [
                "boots"
            ]
        },
        "Sweeping Edge": {
            "levelMax": "3",
            "weight": "2",
            "incompatible": [
                ""
            ],
            "items": [
                "sword"
            ]
        },
        "Swift Sneak": {
            "levelMax": "3",
            "weight": "4",
            "incompatible": [
                ""
            ],
            "items": [
                "leggings"
            ]
        },
        "Thorns": {
            "levelMax": "3",
            "weight": "4",
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
        "Unbreaking": {
            "levelMax": "3",
            "weight": "1",
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
        "Curse of Binding": {
            "levelMax": "1",
            "weight": "4",
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
        "Curse of Vanishing": {
            "levelMax": "1",
            "weight": "4",
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
        }
    }
}
"#;

fn load_registry() -> Registry<'static> {
    let mut registry: Registry<'static> = serde_json::from_str(REGISTRY_JSON).unwrap();

    registry
        .enchants
        .iter_mut()
        .for_each(|(name, enchant)| enchant.name = name);

    registry
}

lazy_static! {
    pub static ref REGISTRY: Registry<'static> = load_registry();
}
