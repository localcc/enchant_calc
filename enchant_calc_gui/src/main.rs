use std::{
    collections::HashMap,
    sync::{
        mpsc::{self, Sender},
        Arc,
    },
    thread,
    time::Duration,
};

use background_work::{BackgroundThreadMessage, SharedData};
use eframe::{
    egui::{self, Button, RichText},
    epaint::{FontId, Stroke},
};
use egui::{Color32, Label, Sense, TextStyle, Ui, Vec2};
use egui_extras::{Column, Size, StripBuilder, TableBuilder};
use enchant_calc::{
    registry::{self, Item},
    solver::{self},
};
use step_ext::{ImageExt, StepExt};

mod background_work;
mod images;
mod step_ext;

fn main() -> Result<(), eframe::Error> {
    let (background_tx, background_rx) = mpsc::channel::<BackgroundThreadMessage>();

    let shared_data = Arc::new(SharedData::default());

    {
        let shared_data = shared_data.clone();
        thread::Builder::new()
            .name("background".to_string())
            .spawn(move || background_work::background_work(shared_data, background_rx))
            .unwrap_or_else(|e| panic!("Failed to start background thread {e}"));
    }

    eframe::run_native(
        "Enchantment Calculator",
        eframe::NativeOptions {
            follow_system_theme: true,
            ..Default::default()
        },
        Box::new(|cc| {
            let mut fonts = egui::FontDefinitions::default();
            fonts.font_data.iter_mut().for_each(|font| {
                font.1.tweak.scale = 1.15;
            });
            cc.egui_ctx.set_fonts(fonts);

            let mut style = (*cc.egui_ctx.style()).clone();
            style.spacing.button_padding = Vec2::new(10.0, 5.0);

            let step_font_id = FontId {
                size: 15.0,
                ..Default::default()
            };
            style
                .text_styles
                .insert(TextStyle::Name("step".into()), step_font_id);

            cc.egui_ctx.set_style(style);

            Box::new(App::new(cc, shared_data, background_tx))
        }),
    )
}

struct App {
    selected_item: Item,
    multiple_protection_types: bool,
    shared_data: Arc<SharedData>,
    background_tx: Sender<BackgroundThreadMessage>,
    available_enchants: Vec<&'static registry::Enchant<'static>>,
    selected_levels: HashMap<&'static str, u32>,
}

impl App {
    fn new(
        _: &eframe::CreationContext<'_>,
        shared_data: Arc<SharedData>,
        background_tx: Sender<BackgroundThreadMessage>,
    ) -> Self {
        Self {
            selected_item: Item::None,
            multiple_protection_types: false,
            shared_data,
            background_tx,
            available_enchants: Vec::new(),
            selected_levels: HashMap::new(),
        }
    }

    fn item_changed(&mut self) {
        *self.shared_data.result.lock() = None;

        let mut enchants = registry::REGISTRY
            .enchants
            .values()
            .filter(|enchant| enchant.items.contains(&self.selected_item))
            .collect::<Vec<_>>();

        // putting curses to the bottom of the list
        enchants.sort_by(|a, b| {
            if a.name.contains("Curse") {
                return std::cmp::Ordering::Greater;
            }

            if b.name.contains("Curse") {
                return std::cmp::Ordering::Less;
            }

            b.level_max.cmp(&a.level_max)
        });

        let mut groups = Vec::new();
        let mut processed_enchants = Vec::new();

        for enchant in enchants {
            if processed_enchants.contains(&enchant.name) {
                continue;
            }

            let mut group = Vec::from([enchant]);
            group.extend(
                enchant
                    .incompatible
                    .iter()
                    .filter_map(|e| registry::REGISTRY.enchants.get(e))
                    .filter(|e| e.items.contains(&self.selected_item)),
            );

            processed_enchants.extend(group.iter().map(|e| e.name));

            groups.push(group);
        }

        self.available_enchants = groups.iter().flatten().copied().collect::<Vec<_>>()
    }

    fn enchant_selection(&mut self, ui: &mut Ui) {
        let old_spacing = ui.style().spacing.item_spacing;
        let new_spacing = Vec2::new(16.0, old_spacing.y);
        ui.style_mut().spacing.item_spacing = new_spacing;

        ui.vertical(|ui| {
            TableBuilder::new(ui)
                .striped(true)
                .column(Column::initial(150.0).at_least(150.0))
                .column(Column::initial(100.0).at_least(100.0))
                .resizable(false)
                .header(32.0, |mut header| {
                    header.col(|ui| {
                        ui.strong("Enchantment");
                    });

                    header.col(|ui| {
                        ui.strong("Level");
                    });
                })
                .body(|mut body| {
                    for available_enchant in &self.available_enchants {
                        body.row(32.0, |mut row| {
                            row.col(|ui| {
                                ui.horizontal_centered(|ui| {
                                    ui.label(available_enchant.name);
                                });
                            });

                            row.col(|ui| {
                                ui.style_mut().spacing.item_spacing = old_spacing;

                                ui.horizontal_centered(|ui| {
                                    let selected_level =
                                        self.selected_levels.get(available_enchant.name).copied();

                                    for i in 1..=available_enchant.level_max {
                                        let button =
                                            match selected_level.map(|e| e == i).unwrap_or(false) {
                                                true => Button::new(format!("{i}"))
                                                    .fill(Color32::TRANSPARENT)
                                                    .stroke(Stroke::new(2.0, Color32::GRAY)),
                                                false => Button::new(format!("{i}")),
                                            };

                                        if ui.add(button).clicked() {
                                            let selected_level = self
                                                .selected_levels
                                                .entry(available_enchant.name)
                                                .or_insert(0);

                                            if *selected_level == i {
                                                self.selected_levels.remove(available_enchant.name);
                                            } else {
                                                *selected_level = i;

                                                if !self.multiple_protection_types {
                                                    available_enchant.incompatible.iter().for_each(
                                                        |e| {
                                                            self.selected_levels.remove(e);
                                                        },
                                                    );
                                                }
                                            }
                                        }
                                    }
                                });
                            });
                        });
                    }
                });
            ui.checkbox(
                &mut self.multiple_protection_types,
                "Minecraft 1.14.1 (multiple protection types)",
            );
        });
    }

    fn show_steps(&mut self, ui: &mut Ui) {
        let result = self.shared_data.result.lock();

        let Some(ref result) = *result else {
            return;
        };

        let Some(ref path) = result.path else {
            ui.label(RichText::new("No solution found").strong().heading());
            return;
        };

        ui.vertical(|ui| {
            ui.horizontal(|ui| {
                ui.label(RichText::new("Total cost:").strong().heading());
                ui.label(RichText::new(format!("{} levels", path.cost)).heading());
            });

            ui.label(format!("Completed in {}ms", result.time.as_millis()))
        });

        ui.add_space(25.0);

        ui.label(RichText::new("Steps").strong().heading());

        ui.separator();

        for (index, step) in path.steps.iter().enumerate() {
            ui.horizontal_wrapped(|ui| {
                ui.style_mut().spacing.item_spacing.x = 1.0;

                ui.add(
                    Label::new(
                        RichText::new(format!("{}. Combine ", index + 1))
                            .text_style(TextStyle::Name("step".into())),
                    )
                    .wrap(true),
                );

                step.left
                    .get_image(self.selected_item)
                    .show_max_size(ui, Vec2::new(24.0, 24.0));

                ui.add(
                    Label::new(
                        RichText::new(format!(" {} with ", step.left.format(self.selected_item)))
                            .text_style(TextStyle::Name("step".into())),
                    )
                    .wrap(true),
                );

                step.right
                    .get_image(self.selected_item)
                    .show_max_size(ui, Vec2::new(24.0, 24.0));

                ui.add(
                    Label::new(
                        RichText::new(format!(" {}", step.right.format(self.selected_item)))
                            .text_style(TextStyle::Name("step".into())),
                    )
                    .wrap(true),
                );

                ui.add_space(50.0);
            });
        }
    }
}

impl eframe::App for App {
    fn update(&mut self, ctx: &eframe::egui::Context, _: &mut eframe::Frame) {
        egui::CentralPanel::default().show(ctx, |ui| {
            StripBuilder::new(ui)
                .size(Size::exact(44.0))
                .size(Size::remainder())
                .vertical(|mut strip| {
                    strip.strip(|builder| {
                        builder
                            .size(Size::remainder())
                            .size(Size::exact(88.0))
                            .horizontal(|mut strip| {
                                strip.cell(|ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        egui::ComboBox::from_id_source("item_picker")
                                            .selected_text(format!("{}", self.selected_item))
                                            .wrap(true)
                                            .show_ui(ui, |ui| {
                                                for variant in [
                                                    Item::Helmet,
                                                    Item::Chestplate,
                                                    Item::Leggings,
                                                    Item::Boots,
                                                    Item::TurtleShell,
                                                    Item::Elytra,
                                                    Item::Sword,
                                                    Item::Axe,
                                                    Item::Trident,
                                                    Item::Pickaxe,
                                                    Item::Shovel,
                                                    Item::Hoe,
                                                    Item::Bow,
                                                    Item::Shield,
                                                    Item::Crossbow,
                                                    Item::FishingRod,
                                                    Item::Shears,
                                                    Item::ShearsBedrock,
                                                    Item::FlintAndSteel,
                                                    Item::Compass,
                                                    Item::CarrotOnAStick,
                                                    Item::Pumpkin,
                                                    Item::WarpedFungusOnAStick,
                                                ] {
                                                    if ui
                                                        .selectable_value(
                                                            &mut self.selected_item,
                                                            variant,
                                                            format!("{variant}"),
                                                        )
                                                        .changed()
                                                    {
                                                        self.item_changed();
                                                    }
                                                }
                                            });
                                    });
                                });
                                strip.cell(|ui| {
                                    ui.vertical_centered_justified(|ui| {
                                        let button = match self
                                            .shared_data
                                            .working
                                            .load(std::sync::atomic::Ordering::Relaxed)
                                        {
                                            true => Button::new("Calculate").sense(Sense::hover()),
                                            false => Button::new("Calculate"),
                                        };

                                        if ui.add(button).clicked() {
                                            let enchants = self
                                                .selected_levels
                                                .iter()
                                                .map(|(name, level)| {
                                                    solver::Enchant::new(
                                                        registry::REGISTRY.enchants[name].clone(),
                                                        *level,
                                                    )
                                                })
                                                .collect::<Vec<_>>();

                                            if self
                                                .background_tx
                                                .send(BackgroundThreadMessage::Calculate(enchants))
                                                .is_ok()
                                            {
                                                self.shared_data.working.store(
                                                    true,
                                                    std::sync::atomic::Ordering::Relaxed,
                                                );
                                            }
                                        }
                                    });
                                });
                            });
                    });

                    strip.strip(|builder| {
                        builder
                            .size(Size::relative(0.2).at_least(350.0))
                            .size(Size::remainder())
                            .horizontal(|mut strip| {
                                strip.cell(|ui| self.enchant_selection(ui));

                                strip.cell(|ui| {
                                    ui.vertical(|ui| {
                                        let working = self
                                            .shared_data
                                            .working
                                            .load(std::sync::atomic::Ordering::Acquire);

                                        if working {
                                            ui.label(
                                                RichText::new(format!(
                                                    "Calculating step {} ({} paths tried)",
                                                    self.shared_data
                                                        .current_step
                                                        .load(std::sync::atomic::Ordering::Relaxed),
                                                    self.shared_data
                                                        .total_tries
                                                        .load(std::sync::atomic::Ordering::Relaxed)
                                                ))
                                                .strong()
                                                .heading(),
                                            );
                                        } else {
                                            self.show_steps(ui);
                                        }
                                    });
                                });
                            });
                    });
                });
        });
        ctx.request_repaint_after(Duration::from_secs(1));
    }
}
