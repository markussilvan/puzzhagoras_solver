use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::{
    puzzle::{Dimensions, Puzzle, PuzzleBuilder},
    solver::{PuzzleState, Solver},
};

#[derive(Deserialize, Serialize, PartialEq)]
enum PieceSet {
    Yellow,
    Green,
    Both,
}

/// We derive Deserialize/Serialize so we can persist app state on shutdown.
#[derive(Deserialize, Serialize)]
#[serde(default)] // if we add new fields, give them default values when deserializing old state
pub struct PuzzhagorasApp {
    width: f32,
    height: f32,
    piece_set: PieceSet,
    #[serde(skip)]
    solver: Option<Solver>,
}

impl Default for PuzzhagorasApp {
    fn default() -> Self {
        Self {
            width: 3.0,
            height: 3.0,
            piece_set: PieceSet::Yellow,
            solver: None,
        }
    }
}

impl PuzzhagorasApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        // This is also where you can customize the look and feel of egui using
        // `cc.egui_ctx.set_visuals` and `cc.egui_ctx.set_fonts`.

        // Load previous app state (if any).
        // Note that you must enable the `persistence` feature for this to work.
        //if let Some(storage) = cc.storage {
        //    return eframe::get_value(storage, eframe::APP_KEY).unwrap_or_default();
        //}

        Default::default()
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Dimensions");

                egui::Grid::new("some_unique_id").show(ui, |ui| {
                    ui.label("Width ");
                    ui.add(egui::Slider::new(&mut self.width, 2.0..=5.0).integer());
                    ui.end_row();

                    ui.label("Height ");
                    ui.add(egui::Slider::new(&mut self.height, 2.0..=5.0).integer());
                    ui.end_row();
                });
            });

            ui.add(egui::Separator::spacing(egui::Separator::default(), 50.0));

            ui.vertical(|ui| {
                ui.heading("Piece set");
                ui.radio_value(&mut self.piece_set, PieceSet::Yellow, "Yellow");
                ui.radio_value(&mut self.piece_set, PieceSet::Green, "Green");
                ui.radio_value(&mut self.piece_set, PieceSet::Both, "Both");
            });
        });

        ui.horizontal(|ui| {
            if ui.button("Reset puzzle").clicked() {
                self.solver = None;
            }
            if ui.button("Solve puzzle").clicked() {
                let dimensions = Dimensions::new(self.width as usize, self.height as usize);

                println!(
                    "Starting with width {} and height {}...",
                    dimensions.width, dimensions.height
                );

                let pieces_file = match self.piece_set {
                    PieceSet::Yellow => "yellow-pieces.json",
                    PieceSet::Green => "green-pieces.json",
                    PieceSet::Both => "both-pieces.json",
                };

                let puzzle = PuzzleBuilder::new()
                    .with_dimensions(dimensions)
                    .with_pieces_from_file(pieces_file.to_string())
                    .build();
                self.solver = Some(Solver::new(puzzle));

                let mut i = 0;
                let mut state = PuzzleState::Progressing;
                while state == PuzzleState::Progressing {
                    i += 1;
                    println!("Step {i}");
                    state = self.solver.as_mut().unwrap().step();
                }

                //puzzle.write_pieces_to_file("pieces.json".to_string());

                println!("Board:");
                //println!("{}", self.solver.as_ref().unwrap().puzzle);
                println!("Final state: {state:?}");
            }
        });
    }
}

impl eframe::App for PuzzhagorasApp {
    /// Called by the frame work to save state before shutdown.
    //fn save(&mut self, storage: &mut dyn eframe::Storage) {
    //    eframe::set_value(storage, eframe::APP_KEY, self);
    //}

    /// Called each time the UI needs repainting, which may be many times per second.
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.5);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            // The top panel is often a good place for a menu bar:

            egui::menu::bar(ui, |ui| {
                // NOTE: no File->Quit on web pages!
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("Quit").clicked() {
                            ctx.send_viewport_cmd(egui::ViewportCommand::Close);
                        }
                    });
                    ui.add_space(16.0);
                }

                egui::widgets::global_dark_light_mode_buttons(ui);
            });
        });

        egui::SidePanel::right("right_panel").show(ctx, |ui| {
            ui.heading("Pieces");
            ui.horizontal(|ui| {
                ui.vertical(|ui| {
                    //TODO: draw all pieces here
                    for _ in 0..13 {
                        ui.add(
                            egui::Image::new(egui::include_image!(
                                "../assets/puzzhagoras_icon.png"
                            ))
                            .rounding(5.0),
                        );
                    }
                });
                ui.vertical(|ui| {
                    //TODO: draw all pieces here
                    for _ in 0..13 {
                        ui.add(
                            egui::Image::new(egui::include_image!(
                                "../assets/puzzhagoras_icon.png"
                            ))
                            .rounding(5.0),
                        );
                    }
                });
            });
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
            self.settings(ui);
            ui.separator();

            puzzle(ui);
            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
}

fn puzzle(ui: &mut egui::Ui) {
    ui.heading("Puzzle");
    ui.vertical(|ui| {
        //TODO: size? how to scale it? use a painter?
        for _ in 0..5 {
            ui.horizontal(|ui| {
                for _ in 0..5 {
                    ui.add(
                        egui::Image::new(egui::include_image!("../assets/puzzhagoras_icon.png"))
                            .rounding(5.0),
                    );
                }
            });
        }
    });
}

fn powered_by_egui_and_eframe(ui: &mut egui::Ui) {
    ui.horizontal(|ui| {
        ui.spacing_mut().item_spacing.x = 0.0;
        ui.label("Powered by ");
        ui.hyperlink_to("egui", "https://github.com/emilk/egui");
        ui.label(" and ");
        ui.hyperlink_to(
            "eframe",
            "https://github.com/emilk/egui/tree/master/crates/eframe",
        );
        ui.label(".");
    });
}
