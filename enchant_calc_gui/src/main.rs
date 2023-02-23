#[cfg(not(target_arch = "wasm32"))]
fn main() -> Result<(), eframe::Error> {
    tracing_subscriber::fmt::init();

    eframe::run_native(
        "Enchantment Calculator",
        eframe::NativeOptions {
            follow_system_theme: true,
            ..Default::default()
        },
        Box::new(|cc| Box::new(enchant_calc_gui::App::new(cc, true))),
    )
}

#[cfg(target_arch = "wasm32")] // make compiler not complain
fn main() {}
