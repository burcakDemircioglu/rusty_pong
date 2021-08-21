use ggez::{
    self,
    graphics::{self, Text, DrawMode},
    input::keyboard::{self, KeyCode},
    nalgebra as na, Context, GameResult,event
};

pub mod utilities;
pub mod constants;

pub struct MainState {
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
        utilities::randomize_vec(&mut ball_vel, constants::BALL_SPEED, constants::BALL_SPEED);

        MainState {
            player_1_pos: na::Point2::new(constants::RACKET_WIDTH_HALF + constants::RACKET_PADDING, screen_h_half),
            player_2_pos: na::Point2::new(
                screen_w - constants::RACKET_WIDTH_HALF - constants::RACKET_PADDING,
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

        // Bounce the ball from walls
        if self.ball_pos.x < 0.0 {
            self.ball_pos = na::Point2::new(screen_w_half, screen_h_half);
            utilities::randomize_vec(&mut self.ball_vel, constants::BALL_SPEED, constants::BALL_SPEED);
            self.player_2_score += 1;
        } else if self.ball_pos.x > screen_w {
            self.ball_pos = na::Point2::new(screen_w_half, screen_h_half);
            utilities::randomize_vec(&mut self.ball_vel, constants::BALL_SPEED, constants::BALL_SPEED);
            self.player_1_score += 1;
        }
        if self.ball_pos.y < constants::BALL_SIZE_HALF {
            self.ball_pos.y = constants::BALL_SIZE_HALF;
            self.ball_vel.y = self.ball_vel.y.abs();
        }
        if self.ball_pos.y > screen_h - constants::BALL_SIZE_HALF {
            self.ball_pos.y = screen_h - constants::BALL_SIZE_HALF;
            self.ball_vel.y = -self.ball_vel.y.abs();
        }

        // Bounce the ball from rackets
        // A little buggy here.. Need to improve
        let intersects_player_1 = self.ball_pos.x - constants::BALL_SIZE_HALF
            < self.player_1_pos.x + constants::RACKET_WIDTH_HALF
            && self.ball_pos.x - constants::BALL_SIZE_HALF > self.player_1_pos.x - constants::RACKET_WIDTH_HALF
            && self.ball_pos.y < self.player_1_pos.y + constants::RACKET_HEIGHT_HALF
            && self.ball_pos.y > self.player_1_pos.y - constants::RACKET_HEIGHT_HALF;

        let intersects_player_2 = self.ball_pos.x + constants::BALL_SIZE_HALF
            > self.player_2_pos.x - constants::RACKET_WIDTH_HALF
            && self.ball_pos.x + constants::BALL_SIZE_HALF < self.player_2_pos.x + constants::RACKET_WIDTH_HALF
            && self.ball_pos.y < self.player_2_pos.y + constants::RACKET_HEIGHT_HALF
            && self.ball_pos.y > self.player_2_pos.y - constants::RACKET_HEIGHT_HALF;

        if intersects_player_1 {
            self.ball_vel.x = self.ball_vel.x.abs();
        } else if intersects_player_2 {
            self.ball_vel.x = -self.ball_vel.x.abs();
        }

        Ok(())
    }

    fn draw(&mut self, ctx: &mut Context) -> GameResult {
        graphics::clear(ctx, graphics::BLACK);
        let mut draw_param = graphics::DrawParam::default();
        let (screen_w, screen_h) = graphics::drawable_size(ctx);
        let screen_w_half = screen_w * 0.5;

        // Draw middle line
        let (origin, dest) = (
            na::Point2::new(screen_w_half, 0.0),
            na::Point2::new(screen_w_half, screen_h),
        );

        let middleline_mesh =
            graphics::Mesh::new_line(ctx, &[origin, dest], constants::MIDDLE_LINE_WIDTH, graphics::WHITE)?;

        graphics::draw(ctx, &middleline_mesh, draw_param)?;

        // Draw rackets
        let racket_rect = graphics::Rect::new(
            -constants::RACKET_WIDTH_HALF,
            -constants::RACKET_HEIGHT_HALF,
            constants::RACKET_WIDTH,
            constants::RACKET_HEIGHT,
        );
        let racket_mesh = graphics::Mesh::new_rectangle(
            ctx,
            graphics::DrawMode::fill(),
            racket_rect,
            graphics::WHITE,
        )?;

        draw_param.dest = self.player_1_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        draw_param.dest = self.player_2_pos.into();
        graphics::draw(ctx, &racket_mesh, draw_param)?;

        // Draw ball
        let ball_mesh = graphics::Mesh::new_circle::<na::Point2<f32>>(
            ctx,
            DrawMode::stroke(constants::BALL_STROKE),
            self.ball_pos.into(),
            constants::BALL_SIZE_HALF,
            constants::BALL_TOLERANCE,
            graphics::WHITE,
        )?;

        graphics::draw(ctx, &ball_mesh, graphics::DrawParam::default())?;

        // Draw score board
        let mut score_text = Text::new(format!("{}  {}", self.player_1_score, self.player_2_score));
        score_text.set_font(graphics::Font::default(), graphics::Scale::uniform(24.0));

        let (score_text_w, score_text_h) = score_text.dimensions(ctx);
        let mut score_pos = na::Point2::new(screen_w_half, 20.0);
        score_pos -= na::Vector2::new(score_text_w as f32 * 0.5, score_text_h as f32 * 0.5);

        draw_param.dest = score_pos.into();
        graphics::draw(ctx, &score_text, draw_param)?;

        graphics::present(ctx)?;
        Ok(())
    }
}

fn move_racket(pos: &mut na::Point2<f32>, key_code: KeyCode, y_dir: f32, ctx: &mut Context) {
    let dt = ggez::timer::delta(ctx).as_secs_f32();
    let screen_h = graphics::drawable_size(ctx).1;

    if keyboard::is_key_pressed(ctx, key_code) {
        pos.y += y_dir * constants::PLAYER_SPEED * dt;
    }

    utilities::clamp(
        &mut pos.y,
        constants::RACKET_HEIGHT_HALF,
        screen_h - constants::RACKET_HEIGHT_HALF,
    );
}