use eframe::egui;
use serde::{Deserialize, Serialize};

use crate::{
    puzzle::{Dimensions, PuzzleBuilder},
    solver::{PuzzleState, Solver},
};

#[derive(Deserialize, Serialize, PartialEq)]
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
        let images = vec![
            // yellow pieces
            egui::Image::new(egui::include_image!("../assets/yellow_01.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_02.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_03.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_04.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_05.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_06.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_07.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_08.png")),
            egui::Image::new(egui::include_image!("../assets/yellow_09.png")),
            // green pieces
            egui::Image::new(egui::include_image!("../assets/green_00.png")),
            egui::Image::new(egui::include_image!("../assets/green_01.png")),
            egui::Image::new(egui::include_image!("../assets/green_02.png")),
            egui::Image::new(egui::include_image!("../assets/green_03.png")),
            egui::Image::new(egui::include_image!("../assets/green_04.png")),
            egui::Image::new(egui::include_image!("../assets/green_05.png")),
            egui::Image::new(egui::include_image!("../assets/green_06.png")),
            egui::Image::new(egui::include_image!("../assets/green_07.png")),
            egui::Image::new(egui::include_image!("../assets/green_08.png")),
            egui::Image::new(egui::include_image!("../assets/green_09.png")),
            egui::Image::new(egui::include_image!("../assets/green_10.png")),
            egui::Image::new(egui::include_image!("../assets/green_11.png")),
            egui::Image::new(egui::include_image!("../assets/green_12.png")),
            egui::Image::new(egui::include_image!("../assets/green_13.png")),
            egui::Image::new(egui::include_image!("../assets/green_14.png")),
            egui::Image::new(egui::include_image!("../assets/green_15.png")),
        ];
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

                egui::Grid::new("dimensions_grid").show(ui, |ui| {
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
            if ui.button("Start solve").clicked() {
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
                while state == PuzzleState::Progressing || state == PuzzleState::Backtrack {
                    i += 1;
                    println!("Step {i}");
                    state = self.solver.as_mut().unwrap().step();
                }

                println!("Final state: {state:?}");
            }
        });
    }

    fn pieces(&self, ui: &mut egui::Ui) {
        let mut piece_id = 0;
        let num_pieces = match self.piece_set {
            PieceSet::Yellow => 9,
            PieceSet::Green => 16,
            PieceSet::Both => 25,
        };
        ui.heading("Pieces");
        egui::Grid::new("puzzle_grid")
            .min_col_width(32.0)
            .min_row_height(32.0)
            .show(ui, |ui| {
                'outer: for _ in 0..13 {
                    for _ in 0..2 {
                        ui.add(self.piece_images[piece_id].clone());
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
                            let image = self.piece_images[piece_id].clone();
                            let piece = self.solver.as_ref().unwrap().get_piece(piece_id);
                            // 90 degress is approx 1.57 radians
                            let angle = (piece.rotations % 4) as f32 * 1.57;
                            ui.add(image.rotate(angle, egui::Vec2::splat(0.5)));
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
            self.pieces(ui);
        });

        egui::CentralPanel::default().show(ctx, |ui| {
            // The central panel the region left after adding TopPanel's and SidePanel's
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
