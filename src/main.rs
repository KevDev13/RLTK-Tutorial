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

#[derive(PartialEq, Copy, Clone)]
enum TileType {
    Wall,
    Floor
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

//try to move the player, assuming they're not trying to move off the screen
fn try_move_player(delta_x: i32, delta_y: i32, ecs: &mut World) {
    let mut positions = ecs.write_storage::<Position>();
    let mut players = ecs.write_storage::<Player>();
    let map = ecs.fetch::<Vec<TileType>>(); // get the map

    for(_player, pos) in (&mut players, &mut positions).join() {
        let destination_idx = xy_idx(pos.x + delta_x, pos.y + delta_y);
        // if no collision
        if map[destination_idx] != TileType::Wall {
            pos.x = min(79, max(0, pos.x + delta_x));
            pos.y = min(49, max(0, pos.y + delta_y));
        }
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

// takes an (x, y) point and changes into a single usize for array purposes
fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

fn new_map() -> Vec<TileType> {
    use TileType::*; // save myself some typing here...
    let mut map = vec![Floor; 80 * 50];

    // make boundaries the walls
    for x in 0..80 {
        map[xy_idx(x, 0)] = Wall;
        map[xy_idx(x, 49)] = Wall;
    }

    for y in 0..50 {
        map[xy_idx(0, y)] = Wall;
        map[xy_idx(79, y)] = Wall;
    }

    // Now randomly place a ton of walls

    // obtain RNG:
    let mut rng = rltk::RandomNumberGenerator::new();

    for _i in 0..400 {
        let x = rng.roll_dice(1, 79);
        let y = rng.roll_dice(1, 49);
        let idx = xy_idx(x, y);
        if idx != xy_idx(40, 25) {
            map[idx] = Wall;
        }
    }

    map
}

fn draw_map(map: &[TileType], ctx: &mut Rltk) {
    let mut x = 0;
    let mut y = 0;

    for tile in map.iter() {
        // render tile
        match tile {
            TileType::Floor => {
                ctx.set(x, y, RGB::from_f32(0.5, 0.5, 0.5), RGB::from_f32(0., 0., 0.), rltk::to_cp437('.'));
            }
            TileType::Wall => {
                ctx.set(x, y, RGB::from_f32(0., 1., 0.), RGB::from_f32(0., 0., 0.), rltk::to_cp437('#'));
            }
        }
        x += 1;
        if x > 79 {
            x = 0;
            y += 1;
        }
    }
}
