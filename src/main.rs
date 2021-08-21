use ggez::{
    self, event,
    graphics,
    GameResult,
};

mod game;

fn main() -> GameResult {
    let cb = ggez::ContextBuilder::new("Pong", "BurcakKam");
    let (ctx, event_loop) = &mut cb.build()?;
    graphics::set_window_title(ctx, "Pong");

    let mut state = game::MainState::new(ctx);
    event::run(ctx, event_loop, &mut state)?;
    Ok(())
}
