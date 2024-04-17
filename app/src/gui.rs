use eframe::egui;
use tracing::info;

use crate::{
    puzzle::{Dimensions, PieceSet, PuzzleBuilder},
    solver::{PuzzleState, Solver},
};

pub struct PuzzhagorasApp {
    width: f32,
    height: f32,
    piece_set: PieceSet,
    solver: Option<Solver>,
    show_progress: bool,
    show_about_dialog: bool,
    state: PuzzleState,
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
            show_progress: false,
            show_about_dialog: false,
            state: PuzzleState::Idle,
            icon: egui::Image::new(egui::include_image!("../assets/puzzhagoras_icon.png")),
            piece_images: images,
        }
    }
}

impl PuzzhagorasApp {
    pub fn new(_cc: &eframe::CreationContext<'_>) -> Self {
        tracing::info!("Initializing GUI...");
        Default::default()
    }

    fn settings(&mut self, ui: &mut egui::Ui, ctx: &egui::Context) {
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

            ui.add(egui::Separator::spacing(egui::Separator::default(), 40.0));

            let old_piece_set = self.piece_set;
            ui.vertical(|ui| {
                ui.heading("Piece set");
                ui.radio_value(&mut self.piece_set, PieceSet::Yellow, "Yellow");
                ui.radio_value(&mut self.piece_set, PieceSet::Green, "Green");
                ui.radio_value(&mut self.piece_set, PieceSet::Both, "Both");
            });

            if self.piece_set != old_piece_set {
                self.solver = None;
            }

            ui.add(egui::Separator::spacing(egui::Separator::default(), 40.0));

            ui.vertical(|ui| {
                ui.heading("Options");
                ui.add(egui::Checkbox::new(
                    &mut self.show_progress,
                    "Show progress *",
                ));
                ui.add_space(23.0);
                ui.add(egui::Label::new("* slows solving"));
            });
        });

        ui.horizontal(|ui| {
            if ui.button("Start solve").clicked() {
                self.start_solve();
            } else {
                self.step(ctx);
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

        let squares = if let Some(s) = self.solver.as_ref() {
            s.get_board_squares()
        } else {
            Vec::new()
        };

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
                            if let Some(s) = self.solver.as_ref() {
                                let piece = s.get_piece(piece_id);
                                let (angle, uv_rect) =
                                    get_piece_image_transformations(piece.rotations, piece.flipped);
                                ui.add(image.rotate(angle, egui::Vec2::splat(0.5)).uv(uv_rect));
                            };
                        }
                    }
                    ui.end_row();
                }
            });
    }

    fn start_solve(&mut self) {
        let dimensions = Dimensions::new(self.width as usize, self.height as usize);

        info!(
            "Starting with width {} and height {}...",
            dimensions.width, dimensions.height
        );

        let pieces_data = include_str!("../pieces.json");

        let puzzle = PuzzleBuilder::new()
            .with_dimensions(dimensions)
            .with_pieces_from_json(pieces_data)
            .with_piece_set(self.piece_set)
            .build();
        self.solver = Some(Solver::new(puzzle));

        self.state = PuzzleState::Progressing;
    }

    fn step(&mut self, ctx: &egui::Context) {
        if self.show_progress {
            if self.state == PuzzleState::Progressing || self.state == PuzzleState::Backtrack {
                if let Some(s) = self.solver.as_mut() {
                    self.state = s.step();
                }
                ctx.request_repaint();
            } else {
                info!("Final state: {:?}", self.state);
            }
        } else {
            while self.state == PuzzleState::Progressing || self.state == PuzzleState::Backtrack {
                if let Some(s) = self.solver.as_mut() {
                    self.state = s.step();
                }
            }
            info!("Final state: {:?}", self.state);
        }
    }

    fn about_dialog(&mut self, ctx: &egui::Context) {
        const VERSION: &str = env!("CARGO_PKG_VERSION");
        let screen_rect_size = ctx.screen_rect().size();
        let center_pos = egui::Pos2::new(screen_rect_size.x / 2.0, screen_rect_size.y / 2.0);
        egui::Window::new("About")
            .collapsible(false)
            .resizable(false)
            .movable(false)
            .default_size(egui::Vec2::new(250.0, 200.0))
            .pivot(egui::Align2([egui::Align::Center, egui::Align::Center]))
            .current_pos(center_pos)
            .show(ctx, |ui| {
                ui.vertical(|ui| {
                    ui.vertical_centered(|ui| {
                        ui.add(
                            self.icon
                                .clone()
                                .fit_to_exact_size(egui::Vec2::new(64.0, 64.0)),
                        );
                        ui.add(egui::Label::new(format!("Puzzhagoras Solver v{VERSION}")));
                        ui.add(egui::Label::new("by Markus Silván, 2024"));
                        egui::warn_if_debug_build(ui);
                        ui.add_space(10.0);
                        if ui.button("Close").clicked() {
                            self.show_about_dialog = false;
                        }
                    });
                });
            });
    }
}

impl eframe::App for PuzzhagorasApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        ctx.set_zoom_factor(1.5);
        egui::TopBottomPanel::top("top_panel").show(ctx, |ui| {
            egui::menu::bar(ui, |ui| {
                let is_web = cfg!(target_arch = "wasm32");
                if !is_web {
                    ui.menu_button("File", |ui| {
                        if ui.button("About").clicked() {
                            self.show_about_dialog = true;
                        }
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
            self.settings(ui, ctx);
            ui.separator();

            self.puzzle(ui);

            if self.show_about_dialog {
                self.about_dialog(ctx);
            }
        });
    }
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
