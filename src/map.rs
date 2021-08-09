use rltk::{ RGB,
            Rltk,
            RandomNumberGenerator };
use std::cmp::{ min, max };

#[derive(PartialEq, Copy, Clone)]
pub enum TileType {
    Wall,
    Floor
}

/// Makes a map with solid boundaires and 400 randomly placed wall. It won't be pretty.
pub fn new_map_test() -> Vec<TileType> {
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

pub fn draw_map(map: &[TileType], ctx: &mut Rltk) {
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

// takes an (x, y) point and changes into a single usize for array purposes
pub fn xy_idx(x: i32, y: i32) -> usize {
    (y as usize * 80) + x as usize
}

pub fn new_map_rooms_and_corridors() -> Vec<TileType> {
    let mut map = vec![TileType::Wall; 80 * 50];

    map
}
