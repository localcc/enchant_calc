use egui_extras::RetainedImage;
use enchant_calc::{registry, solver::ResolvedStepItem};
use itertools::Itertools;

use crate::images;

pub trait StepExt {
    fn format(&self, item: registry::Item) -> String;
}

impl StepExt for ResolvedStepItem {
    fn format(&self, item: registry::Item) -> String {
        match self {
            ResolvedStepItem::Item => item.to_string(),
            ResolvedStepItem::Enchant(e) => match e.level > 1 {
                true => format!("{} {}", e.enchant.name, e.level),
                false => e.enchant.name.to_string(),
            },
        }
    }
}

impl StepExt for Vec<ResolvedStepItem> {
    fn format(&self, item: registry::Item) -> String {
        let mut result = self.first().unwrap().format(item);

        if self.len() > 1 {
            result += " (w/ ";
        }
        result += &self.iter().skip(1).map(|e| e.format(item)).join(" + ");
        if self.len() > 1 {
            result += ")";
        }

        result
    }
}

pub trait ImageExt {
    fn get_image(&self, item: registry::Item) -> &RetainedImage;
}

impl ImageExt for ResolvedStepItem {
    fn get_image(&self, item: registry::Item) -> &RetainedImage {
        match self {
            ResolvedStepItem::Item => match item {
                registry::Item::None => &images::ENCHANTED_BOOK,
                registry::Item::Helmet => &images::HELMET,
                registry::Item::Chestplate => &images::CHESTPLATE,
                registry::Item::Leggings => &images::LEGGINGS,
                registry::Item::Boots => &images::BOOTS,
                registry::Item::TurtleShell => &images::TURTLE_SHELL,
                registry::Item::Elytra => &images::ELYTRA,
                registry::Item::Sword => &images::SWORD,
                registry::Item::Axe => &images::AXE,
                registry::Item::Trident => &images::TRIDENT,
                registry::Item::Pickaxe => &images::PICKAXE,
                registry::Item::Shovel => &images::SHOVEL,
                registry::Item::Hoe => &images::HOE,
                registry::Item::Bow => &images::BOW,
                registry::Item::Shield => &images::SHIELD,
                registry::Item::Crossbow => &images::CROSSBOW,
                registry::Item::FishingRod => &images::FISHING_ROD,
                registry::Item::Shears => &images::SHEARS,
                registry::Item::ShearsBedrock => &images::SHEARS,
                registry::Item::FlintAndSteel => &images::FLINT_AND_STEEL,
                registry::Item::Compass => &images::COMPASS,
                registry::Item::CarrotOnAStick => &images::CARROT_ON_A_STICK,
                registry::Item::Pumpkin => &images::PUMPKIN,
                registry::Item::WarpedFungusOnAStick => &images::WARPED_FUNGUS_ON_A_STICK,
            },
            ResolvedStepItem::Enchant(_) => &images::ENCHANTED_BOOK,
        }
    }
}

impl ImageExt for Vec<ResolvedStepItem> {
    fn get_image(&self, item: registry::Item) -> &RetainedImage {
        self.first().unwrap().get_image(item)
    }
}
