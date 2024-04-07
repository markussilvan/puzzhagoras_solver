use eframe::egui;
use serde::{Deserialize, Serialize};
use tracing::{debug, info};

use crate::{
    puzzle::{Dimensions, PuzzleBuilder},
    solver::{PuzzleState, Solver},
};

#[derive(Deserialize, Serialize, PartialEq, Clone)]
enum PieceSet {
    Yellow,
    Green,
    Both,
}

pub struct PuzzhagorasApp {
    width: f32,
    height: f32,
    piece_set: PieceSet,
    solver: Option<Solver>,
    icon: egui::Image<'static>,
    piece_images: Vec<egui::Image<'static>>,
}

impl Default for PuzzhagorasApp {
    fn default() -> Self {
        let images = puzzhagoras_solver_macros::include_piece_images!(
            ;"../assets/yellow_",
            9;
            "../assets/green_",
            16
        );

        Self {
            width: 3.0,
            height: 3.0,
            piece_set: PieceSet::Yellow,
            solver: None,
            icon: egui::Image::new(egui::include_image!("../assets/puzzhagoras_icon.png")),
            piece_images: images,
        }
    }
}

impl PuzzhagorasApp {
    /// Called once before the first frame.
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        tracing::info!("Initializing GUI...");
        Default::default()
    }

    fn settings(&mut self, ui: &mut egui::Ui) {
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.heading("Dimensions");

                let old_width = self.width;
                let old_height = self.height;
                egui::Grid::new("dimensions_grid").show(ui, |ui| {
                    ui.label("Width ");
                    ui.add(egui::Slider::new(&mut self.width, 2.0..=5.0).integer());
                    ui.end_row();

                    ui.label("Height ");
                    ui.add(egui::Slider::new(&mut self.height, 2.0..=5.0).integer());
                    ui.end_row();
                });

                if self.width != old_width || self.height != old_height {
                    self.solver = None;
                }
            });

            ui.add(egui::Separator::spacing(egui::Separator::default(), 50.0));

            let old_piece_set = self.piece_set.clone();
            ui.vertical(|ui| {
                ui.heading("Piece set");
                ui.radio_value(&mut self.piece_set, PieceSet::Yellow, "Yellow");
                ui.radio_value(&mut self.piece_set, PieceSet::Green, "Green");
                ui.radio_value(&mut self.piece_set, PieceSet::Both, "Both");
            });

            if self.piece_set != old_piece_set {
                self.solver = None;
            }
        });

        ui.horizontal(|ui| {
            if ui.button("Start solve").clicked() {
                let dimensions = Dimensions::new(self.width as usize, self.height as usize);

                info!(
                    "Starting with width {} and height {}...",
                    dimensions.width, dimensions.height
                );

                let pieces_data = match self.piece_set {
                    PieceSet::Yellow => include_str!("../yellow-pieces.json"),
                    PieceSet::Green => include_str!("../green-pieces.json"),
                    PieceSet::Both => include_str!("../both-pieces.json"),
                };

                let puzzle = PuzzleBuilder::new()
                    .with_dimensions(dimensions)
                    .with_pieces_from_json(pieces_data)
                    .build();
                self.solver = Some(Solver::new(puzzle));

                let mut i = 0;
                let mut state = PuzzleState::Progressing;
                while state == PuzzleState::Progressing || state == PuzzleState::Backtrack {
                    i += 1;
                    debug!("Step {i}");
                    state = self.solver.as_mut().unwrap().step();
                }

                info!("Final state: {state:?}");
            }
        });
    }

    fn pieces(&self, ui: &mut egui::Ui) {
        let mut piece_id = 0;
        let (num_pieces, piece_offset) = get_piece_set_info(&self.piece_set);

        ui.heading("Pieces");
        egui::Grid::new("pieces_grid")
            .min_col_width(32.0)
            .min_row_height(32.0)
            .show(ui, |ui| {
                'outer: for _ in 0..13 {
                    for _ in 0..2 {
                        ui.add(self.piece_images[piece_id + piece_offset].clone());
                        piece_id += 1;
                        if piece_id >= num_pieces {
                            break 'outer;
                        }
                    }
                    ui.end_row();
                }
            });
    }

    fn puzzle(&self, ui: &mut egui::Ui) {
        if self.solver.is_none() {
            return;
        }

        let (num_pieces, piece_offset) = get_piece_set_info(&self.piece_set);
        let squares = self.solver.as_ref().unwrap().get_board_squares();

        egui::Grid::new("puzzle_grid")
            .min_col_width(64.0)
            .min_row_height(64.0)
            .show(ui, |ui| {
                for x in 0..self.height as usize {
                    for y in 0..self.width as usize {
                        let position = x * self.width as usize + y;
                        if position >= squares.len() || squares[position].is_empty() {
                            ui.add(self.icon.clone());
                        } else {
                            let piece_id = squares[position].piece_id();
                            let image = if piece_id < num_pieces {
                                self.piece_images[piece_id + piece_offset].clone()
                            } else {
                                // this should never happen, but just in case...
                                self.icon.clone()
                            };
                            let piece = self.solver.as_ref().unwrap().get_piece(piece_id);
                            let (angle, uv_rect) =
                                get_piece_image_transformations(piece.rotations, piece.flipped);
                            ui.add(image.rotate(angle, egui::Vec2::splat(0.5)).uv(uv_rect));
                        }
                    }
                    ui.end_row();
                }
            });
    }
}

impl eframe::App for PuzzhagorasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.5);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
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
            self.pieces(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            self.settings(ui);
            ui.separator();

            self.puzzle(ui);
            ui.separator();

            ui.with_layout(egui::Layout::bottom_up(egui::Align::LEFT), |ui| {
                powered_by_egui_and_eframe(ui);
                egui::warn_if_debug_build(ui);
            });
        });
    }
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

/// Get information related to the used piece set
///
/// # Arguments
/// `piece_set` - the piece set to use
///
/// # Returns
///
/// A tuple containing the number of pieces in the set and
/// the offset of the first piece in the global list of pieces.
///
fn get_piece_set_info(piece_set: &PieceSet) -> (usize, usize) {
    match piece_set {
        PieceSet::Yellow => (9, 0),
        PieceSet::Green => (16, 9),
        PieceSet::Both => (25, 0),
    }
}

/// Calculate a rotation angle and optional flipping Rect for a piece's image
///
/// # Arguments
///
/// - `rotations`   - how many times the piece has been rotated
/// - `flip`        - has the piece been flipped related to it's original orientation
///
/// # Returns
///
/// A tuple containing the rotation angle in radians and a UV rectangle.
///
fn get_piece_image_transformations(rotations: usize, flip: bool) -> (f32, egui::Rect) {
    // angles are in radians, so 90 degrees is PI/2 rad
    let mut angle = (rotations % 4) as f32 * (std::f32::consts::PI / 2.0);
    let uv_rect = if flip {
        // flip image horizontally, and rotate
        angle = (rotations % 4) as f32 * (std::f32::consts::PI / 2.0) + std::f32::consts::PI;
        egui::Rect::from_min_max(egui::Pos2::new(0.0, 1.0), egui::Pos2::new(1.0, 0.0))
    } else {
        // just rotate
        egui::Rect::from_min_max(egui::Pos2::new(0.0, 0.0), egui::Pos2::new(1.0, 1.0))
    };

    (angle, uv_rect)
}
