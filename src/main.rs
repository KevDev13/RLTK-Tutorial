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

        // handle player input
        player_input(self, ctx);

        // get all position components and rederable components in the ECS
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // go through and print all the things
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
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

#[derive(Component, Debug)]
struct Player {

}

fn main() -> rltk::BError {
    use rltk::RltkBuilder;
    let context = RltkBuilder::simple80x50()
        .with_title("Roguelike Tutorial")
        .build()?;

    // game state
    let mut gs = State {
        ecs: World::new()
    };

    // register component types
    gs.ecs.register::<Position>();
    gs.ecs.register::<Renderable>();
    gs.ecs.register::<Player>();

    // create player
    gs.ecs.create_entity()
      .with(Position { x: 40, y: 25 })
      .with(Renderable {
          glyph: rltk::to_cp437('@'),
          fg: RGB::named(rltk::YELLOW),
          bg: RGB::named(rltk::BLACK),
      })
      .with(Player {})
      .build();

    // add in some test things for now
    for i in 1..10 {
        gs.ecs.create_entity()
          .with(Position { x: i * 7, y:20 })
          .with(Renderable {
              glyph: rltk::to_cp437('☺'),
              fg: RGB::named(rltk::RED),
              bg: RGB::named(rltk::BLACK),
          })
          .build();
    }

    // loop for-ev-er
    rltk::main_loop(context, gs)
}

fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();

    for(_player, pos) in (&mut players, &mut positions).join() {
        pos.x = min(79, max(0, pos.x + delta_x));
        pos.y = min(49, max(0, pos.y + delta_y));
    }
}

fn player_input(gs: &mut State, ctx: &mut Rltk) {
    // player movement
    match ctx.key {
        None => {}
        Some(key) => match key {
            VirtualKeyCode::Left => try_move_player(-1, 0, &mut gs.ecs),
            VirtualKeyCode::Right => try_move_player(1, 0, &mut gs.ecs),
            VirtualKeyCode::Up => try_move_player(0, -1, &mut gs.ecs),
            VirtualKeyCode::Down => try_move_player(0, 1, &mut gs.ecs),
            _ => {}
        },
    }
}
