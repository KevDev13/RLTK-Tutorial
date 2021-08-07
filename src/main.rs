use rltk::{ Rltk,
            GameState,
            RGB,
            VirtualKeyCode};
use specs::prelude::*;
use std::cmp::{ max, min };
use specs_derive::Component;

struct State{
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();  // clear screen
        ctx.print(1, 1, "hello RLTK!");
    }
}

#[derive(Component)]
struct Position {
    x: i32,
    y: i32,
}

#[derive(Component)]
struct Renderable {
    glyph: rltk::FontCharType,
    fg: RGB,
    bg: RGB,
}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    let mut gs = State {
        ecs: World::new()
    };
    rltk::main_loop(context, gs)
}
