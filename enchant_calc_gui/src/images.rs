use eframe::egui::TextureOptions;
use egui_extras::RetainedImage;
use lazy_static::lazy_static;

macro_rules! load_image {
    ($image_name:expr, $var_name:ident) => {
        lazy_static! {
            pub static ref $var_name: RetainedImage = RetainedImage::from_image_bytes(
                $image_name,
                include_bytes!(concat!("../assets/", $image_name, ".png"))
            )
            .unwrap()
            .with_options(TextureOptions::NEAREST);
        }
    };
}

load_image!("bow", BOW);
load_image!("carrot_on_a_stick", CARROT_ON_A_STICK);
load_image!("compass", COMPASS);
load_image!("crossbow", CROSSBOW);

load_image!("golden_axe", AXE);
load_image!("golden_boots", BOOTS);
load_image!("golden_chestplate", CHESTPLATE);
load_image!("golden_helmet", HELMET);
load_image!("golden_hoe", HOE);
load_image!("golden_leggings", LEGGINGS);
load_image!("golden_pickaxe", PICKAXE);
load_image!("golden_shovel", SHOVEL);
load_image!("golden_sword", SWORD);

load_image!("elytra", ELYTRA);
load_image!("fishing_rod", FISHING_ROD);
load_image!("flint_and_steel", FLINT_AND_STEEL);
load_image!("pumpkin", PUMPKIN);
load_image!("shears", SHEARS);
load_image!("shield", SHIELD);
load_image!("turtle_helmet", TURTLE_SHELL);
load_image!("warped_fungus_on_a_stick", WARPED_FUNGUS_ON_A_STICK);
load_image!("trident", TRIDENT);

load_image!("enchanted_book", ENCHANTED_BOOK);
