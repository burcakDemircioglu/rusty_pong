use ggez::{
    self, event,
    graphics::{self, DrawMode},
    nalgebra as na, Context, GameResult,
};

const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_STROKE: f32 = 3.5;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_TOLERANCE: f32 = 0.1;

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
}

impl MainState {
    pub fn new(context: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(context);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF, screen_h_half),
            player_2_pos: na::Point2::new(screen_w - RACKET_WIDTH_HALF, screen_h_half),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);

        // Draw rackets
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

        let mut draw_param = graphics::DrawParam::default();
        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        // Draw ball
        let ball_mesh = graphics::Mesh::new_circle::<na::Point2<f32>>(
            ctx,
            DrawMode::stroke(BALL_STROKE),
            self.ball_pos.into(),
            BALL_SIZE_HALF,
            BALL_TOLERANCE,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;

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
