use crate::scenes::game;
use std::path::Path;

pub struct Tile {
    pub floor: Vec<game::Block>,
}

impl Tile {
    fn create() -> Self {
        // Floor
        let mut floor: Vec<game::Block> = vec![];

        for j in (1..=3).step_by(2)  {
            for i in (1..=32).step_by(2) {
                let stone = game::Block::create(
                    -1.0+((16.0/256.0)*i as f32), 
                    -1.0+((16.0/208.0)*j as f32), 
                    16.0/208.0, 
                    16.0/256.0,
                    &Path::new("src/scenes/game/assets/images/stone.png"),
                );
                floor.push(stone);
            }
        }

        Self{floor}
    }

    unsafe fn draw(&self) {
        for stone in self.floor.iter() {
            stone.draw()
        }
    }
}

pub struct World {
    pub tiles: Vec<Tile>,
}

impl World {
    pub fn init() -> Self {
        let mut tiles: Vec<Tile> = vec![];

        let tile = Tile::create();
        tiles.push(tile);

        Self{tiles}
    }

    pub unsafe fn draw(&self) {
        for tile in self.tiles.iter() {
            tile.draw();
        }
    }
}
