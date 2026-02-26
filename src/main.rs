mod structs;
use crate::structs::{Piece, Type};

mod init_board;
use crate::init_board::init_board;

mod move_pieces;
use crate::move_pieces::{mv_king, mv_queen, mv_bishop, mv_knight, mv_rook, mv_pawn};

use ggez::event;
use ggez::graphics::{Canvas, Color, DrawParam, Image};
use ggez::input::mouse::MouseButton;
use ggez::mint::Point2;
use ggez::{Context, ContextBuilder};

const TILE_SIZE: f32 = 105.5;

struct GameState {
    board: [[Piece; 8]; 8],
    board_tex: Image,
    sprites: [Option<Image>; 13],
    selected: Option<[usize; 2]>,
}

impl GameState {
    fn new(ctx: &mut Context) -> Self {
        let board = init_board();
        let board_tex = Image::from_path(ctx, "/board.png")
            .unwrap_or_else(|_| panic!("Impossible de charger board.png"));

        let mut sprites: [Option<Image>; 13] = Default::default();

        let load = |ctx: &mut Context, path: &str| {
            Image::from_path(ctx, path).ok()
        };

        sprites[Type::Wpawn as usize] = load(ctx, "/white_pawn.png");
        sprites[Type::Wrook as usize] = load(ctx, "/white_rook.png");
        sprites[Type::Wknight as usize] = load(ctx, "/white_knight.png");
        sprites[Type::Wbishop as usize] = load(ctx, "/white_bishop.png");
        sprites[Type::Wqueen as usize] = load(ctx, "/white_queen.png");
        sprites[Type::Wking as usize] = load(ctx, "/white_king.png");

        sprites[Type::Bpawn as usize] = load(ctx, "/black_pawn.png");
        sprites[Type::Brook as usize] = load(ctx, "/black_rook.png");
        sprites[Type::Bknight as usize] = load(ctx, "/black_knight.png");
        sprites[Type::Bbishop as usize] = load(ctx, "/black_bishop.png");
        sprites[Type::Bqueen as usize] = load(ctx, "/black_queen.png");
        sprites[Type::Bking as usize] = load(ctx, "/black_king.png");

        Self {
            board,
            board_tex,
            sprites,
            selected: None,
        }
    }

    fn screen_to_board(x: f32, y: f32) -> Option<[usize; 2]> {
        if x < 0.0 || y < 0.0 {
            return None;
        }
        let col = (x / TILE_SIZE) as usize;
        let row = (y / TILE_SIZE) as usize;
        if row < 8 && col < 8 {
            Some([row, col])
        } else {
            None
        }
    }
}

impl event::EventHandler<Context> for GameState {
    fn update(&mut self, _ctx: &mut Context) -> Result<(), Context> {
        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> Result<(), Context> {
        let mut canvas = Canvas::from_frame(ctx, Color::from_rgb(0, 0, 0));

        canvas.draw(&self.board_tex, DrawParam::default());

        for i in 0..8 {
            for j in 0..8 {
                let piece = self.board[i][j];
                if piece.type_p != Type::None {
                    if let Some(img) = &self.sprites[piece.type_p as usize] {
                        let dest = Point2 {
                            x: j as f32 * TILE_SIZE,
                            y: i as f32 * TILE_SIZE,
                        };
                        canvas.draw(img, DrawParam::default().dest(dest));
                    }
                }
            }
        }
        let _ = canvas.finish(ctx);

        Ok(())
    }

    fn mouse_button_down_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), Context> {
        if button == MouseButton::Left {
            if let Some(cords) = GameState::screen_to_board(x, y) {
                if self.board[cords[0]][cords[1]].type_p != Type::None {
                    self.selected = Some(cords);
                }
            }
        }
        Ok(())
    }

    fn mouse_button_up_event(
        &mut self,
        _ctx: &mut Context,
        button: MouseButton,
        x: f32,
        y: f32,
    ) -> Result<(), Context> {
        if button == MouseButton::Left {
            if let (Some(from), Some(to)) = (self.selected, GameState::screen_to_board(x, y)) {
                let piece = self.board[from[0]][from[1]];
                let _ = mv_pawn(piece, &mut self.board, to);
                let _ = mv_rook(piece, &mut self.board, to);
                let _ = mv_knight(piece, &mut self.board, to);
                let _ = mv_bishop(piece, &mut self.board, to);
                let _ = mv_queen(piece, &mut self.board, to);
                let _ = mv_king(piece, &mut self.board, to);
            }
            self.selected = None;
        }
        Ok(())
    }
}

pub fn main() {
    let (mut ctx, event_loop) = ContextBuilder::new("echecs", "moi")
        .add_resource_path("./sprites")
        .window_setup(ggez::conf::WindowSetup::default().title("Rust Chess"))
        .window_mode(
            ggez::conf::WindowMode::default()
                .dimensions(8.0 * TILE_SIZE, 8.0 * TILE_SIZE),
        )
        .build()
        .expect("Impossible de créer le contexte ggez");

    let state = GameState::new(&mut ctx);
    event::run(ctx, event_loop, state);
}
