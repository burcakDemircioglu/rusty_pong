use ggez::{
    self, event,
    graphics::{self, DrawMode, Text},
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context, GameResult,
};
use rand::{thread_rng, Rng};

const RACKET_PADDING: f32 = 20.0;
const RACKET_HEIGHT: f32 = 100.0;
const RACKET_WIDTH: f32 = 20.0;
const RACKET_HEIGHT_HALF: f32 = RACKET_HEIGHT * 0.5;
const RACKET_WIDTH_HALF: f32 = RACKET_WIDTH * 0.5;
const BALL_SIZE: f32 = 30.0;
const BALL_STROKE: f32 = 3.5;
const BALL_SIZE_HALF: f32 = BALL_SIZE * 0.5;
const BALL_TOLERANCE: f32 = 0.1;
const PLAYER_SPEED: f32 = 500.0;
const BALL_SPEED: f32 = 150.0;

fn clamp(value: &mut f32, low: f32, high: f32) {
    if *value < low {
        *value = low;
    } else if *value > high {
        *value = high;
    }
}

fn move_racket(pos: &mut na::Point2<f32>, key_code: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    let screen_h = graphics::drawable_size(ctx).1;

    if keyboard::is_key_pressed(ctx, key_code) {
        pos.y += y_dir * PLAYER_SPEED * dt;
    }

    clamp(
        &mut pos.y,
        RACKET_HEIGHT_HALF,
        screen_h - RACKET_HEIGHT_HALF,
    );
}

fn randomize_vec(vec: &mut na::Vector2<f32>, x: f32, y: f32) {
    let mut rng = thread_rng();
    vec.x = match rng.gen_bool(0.5) {
        true => x,
        false => -x,
    };
    vec.y = match rng.gen_bool(0.5) {
        true => y,
        false => -y,
    };
}

struct MainState {
    player_1_pos: na::Point2<f32>,
    player_2_pos: na::Point2<f32>,
    ball_pos: na::Point2<f32>,
    ball_vel: na::Vector2<f32>,
    player_1_score: i32,
    player_2_score: i32,
}

impl MainState {
    pub fn new(context: &mut Context) -> Self {
        let (screen_w, screen_h) = graphics::drawable_size(context);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        let mut ball_vel = na::Vector2::new(0.0, 0.0);
        randomize_vec(&mut ball_vel, BALL_SPEED, BALL_SPEED);

        MainState {
            player_1_pos: na::Point2::new(RACKET_WIDTH_HALF + RACKET_PADDING, screen_h_half),
            player_2_pos: na::Point2::new(
                screen_w - RACKET_WIDTH_HALF - RACKET_PADDING,
                screen_h_half,
            ),
            ball_pos: na::Point2::new(screen_w_half, screen_h_half),
            ball_vel: ball_vel,
            player_1_score: 0,
            player_2_score: 0,
        }
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, ctx: &mut Context) -> GameResult {
        let dt = ggez::timer::delta(ctx).as_secs_f32();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let (screen_w_half, screen_h_half) = (screen_w * 0.5, screen_h * 0.5);

        move_racket(&mut self.player_1_pos, KeyCode::W, -1.0, ctx);
        move_racket(&mut self.player_1_pos, KeyCode::S, 1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Up, -1.0, ctx);
        move_racket(&mut self.player_2_pos, KeyCode::Down, 1.0, ctx);

        self.ball_pos += self.ball_vel * dt;

        if self.ball_pos.x < 0.0 {
            self.ball_pos = na::Point2::new(screen_w_half, screen_h_half);
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_2_score += 1;
        } else if self.ball_pos.x > screen_w {
            self.ball_pos = na::Point2::new(screen_w_half, screen_h_half);
            randomize_vec(&mut self.ball_vel, BALL_SPEED, BALL_SPEED);
            self.player_1_score += 1;
        }
        if self.ball_pos.y < BALL_SIZE_HALF {
            self.ball_pos.y = BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        }
        if self.ball_pos.y > screen_h - BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        let intersects_player_1 = 
            self.ball_pos.x - BALL_SIZE_HALF < self.player_1_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.x - BALL_SIZE_HALF > self.player_1_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.y < self.player_1_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y > self.player_1_pos.y - RACKET_HEIGHT_HALF;

        let intersects_player_2 = 
            self.ball_pos.x + BALL_SIZE_HALF > self.player_2_pos.x - RACKET_WIDTH_HALF
            && self.ball_pos.x + BALL_SIZE_HALF < self.player_2_pos.x + RACKET_WIDTH_HALF
            && self.ball_pos.y < self.player_2_pos.y + RACKET_HEIGHT_HALF
            && self.ball_pos.y > self.player_2_pos.y - RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        } else if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }

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

        let screen_w = graphics::drawable_size(ctx).0;
        let screen_w_half = screen_w * 0.5;
        let score_text = Text::new(format!("{}:{}", self.player_1_score, self.player_2_score));

        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        let mut score_pos = na::Point2::new(screen_w_half, 20.0);
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);

        draw_param.dest = score_pos.into();
        graphics::draw(ctx, &score_text, draw_param)?;

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
