use ggez::{self, event, graphics, Context, GameResult};

struct MainState {}

impl MainState {
    pub fn new() -> Self {
        MainState {}
    }
}

impl event::EventHandler for MainState {
    fn update(&mut self, _ctx: &mut Context) -> GameResult {
        Ok(())
    }
    fn draw(&mut self, _ctx: &mut Context) -> GameResult {
        graphics::clear(_ctx, graphics::BLACK);
        graphics::present(_ctx)?;
        Ok(())
    }
}

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "BurcakKam");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Pong");

    let mut state = MainState::new();
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}
