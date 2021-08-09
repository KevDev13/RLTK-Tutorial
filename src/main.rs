use rltk::{ Rltk,
            GameState,
            RGB };
use specs::prelude::*;

mod components;
pub use components::*;
mod map;
pub use map::*;
mod player;
pub use player::*;
mod rect;
pub use rect::*;

pub struct State{
    ecs: World
}

impl GameState for State {
    fn tick(&mut self, ctx: &mut Rltk) {
        ctx.cls();  // clear screen

        // handle player input
        player_input(self, ctx);

        // get & draw map
        let map = self.ecs.fetch::<Vec<TileType>>();
        draw_map(&map, ctx);

        // get all position components and rederable components in the ECS
        let positions = self.ecs.read_storage::<Position>();
        let renderables = self.ecs.read_storage::<Renderable>();

        // go through and print all the things
        for (pos, render) in (&positions, &renderables).join() {
            ctx.set(pos.x, pos.y, render.fg, render.bg, render.glyph);
        }
    }
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

    // add ye olde map
    gs.ecs.insert(new_map());

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
