use bracket_lib::prelude::*;

enum GameMode {
    Menu,
    Playing,
    End
}
struct State {
    num: isize,
    mode: GameMode,
}
impl State {
    fn new() -> Self {
        State {
            num: 1,
            mode: GameMode::Menu
        }
    }
    fn play(&mut self) {
        self.mode = GameMode::End;
    }

    fn restart(&mut self) {
        self.mode = GameMode::Playing;
    }

    fn main_menu(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "welcome to flayppy dragon.");
        ctx.print_centered(8, "(P) Play Game.");
        ctx.print_centered(9, "(Q) Quit Game.");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }

    fn dead(&mut self, ctx: &mut BTerm) {
        ctx.cls();
        ctx.print_centered(5, "You are dead!");
        ctx.print_centered(8, "(P) Play Again.");
        ctx.print_centered(9, "(Q) Quit Game.");
        if let Some(key) = ctx.key {
            match key {
                VirtualKeyCode::P => self.restart(),
                VirtualKeyCode::Q => ctx.quitting = true,
                _ => {}
            }
        }
    }
}
impl GameState for State {
    fn tick(&mut self, ctx: &mut BTerm) {
        ctx.cls();// 清理屏幕
        self.num += 1;
        ctx.print(1, 1, format!("hello, bracket terminal! {}", self.num));
        match self.mode {
            GameMode::Menu => {self.main_menu(ctx)}
            GameMode::Playing => {self.play(ctx)}
            GameMode::End => {self.dead(ctx)}
        }
    }
}
fn main() -> BError {
    let context = BTermBuilder::simple80x50()
        .with_title("Flappy Dragon")
        // Specify the font file and the dimensions of each glyph in the font sprite sheet
        .with_font("terminal8x8.png", 8, 8)
        // Set the size at which each glyph will be rendered on screen (e.g., 14x14 pixels)
        .with_tile_dimensions(25, 25)
        .build()?;

    main_loop(context, State::new())
}