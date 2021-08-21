use ggez::{
    self, event,
    graphics::{self, DrawMode, FillOptions},
    nalgebra as na, Context, GameResult,
};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
}

impl MainState {
    pub fn new(context: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(context);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        let racket_rect = graphics::Rect::new(
            -RACKET_WIDTH_HALF,
            -RACKET_HEIGHT_HALF,
            RACKET_WIDTH,
            RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;

        let mut draw_param_1 = graphics::DrawParam::default();
        draw_param_1.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param_1)?;

        let mut draw_param_2 = graphics::DrawParam::default();
        draw_param_2.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param_2)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "BurcakKam");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Pong");

    let mut state = MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}
