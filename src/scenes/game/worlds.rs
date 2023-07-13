use crate::scenes::game;
use crate::scenes::game::background;
use crate::scenes::game::objects;
use std::path::Path;

pub struct Tile {
    pub bg: background::Background,
    pub floor: Vec<game::Block>,
    pub objects: objects::Objects,
    move_by: f32,
    bg_index: u32,
    last_drawpos: i32,
}

impl Tile {
    fn create(last_pos: i32, add: i32, bg_index: u32, move_by: f32) -> Self {
        let bg = background::Background::init(move_by, 0.0, 1.0, 1.0, bg_index); 

        // Floor
        let mut floor: Vec<game::Block> = vec![];
        let mut last_drawpos = 0;

        for j in (1..=3).step_by(2)  {
            for i in (last_pos..=32*add).step_by(2) {
                let stone = game::Block::create(
                    -1.0+((16.0/256.0)*i as f32), 
                    -1.0+((16.0/208.0)*j as f32), 
                    16.0/208.0, 
                    16.0/256.0,
                    &Path::new("src/scenes/game/assets/images/stone.png"),
                );
                floor.push(stone);
                last_drawpos = i;
            }
        }

        let mut objects = objects::Objects::init();

        Self{bg, floor, objects, move_by, bg_index, last_drawpos}
    }

    unsafe fn draw(&self) {
        self.bg.background_prog.set_active();
        self.bg.draw();

        self.floor[0].program.set_active();
        for stone in self.floor.iter() {
            stone.draw()
        }

        self.objects.draw()
    }
}

pub struct World {
    pub tiles: Vec<Tile>,
}

impl World {
    pub fn init() -> Self {
        let mut tiles: Vec<Tile> = vec![];

        let tile1 = Tile::create(1, 1, 1, 0.0);
        tiles.push(tile1);
        
        let mut tile2 = Tile::create(tiles[0].last_drawpos+2, 2, 2, 2.0);
        tile2.objects.create_question_mark_block(
            -1.0+((16.0/256.0)*33 as f32), 
            -1.0+((16.0/208.0)*11 as f32),
            16.0/208.0, 
            16.0/256.0,
        );
        tile2.objects.create_block(
            -1.0+((16.0/256.0)*41 as f32), 
            -1.0+((16.0/208.0)*11 as f32),
            16.0/208.0, 
            16.0/256.0,
        );
        tile2.objects.create_question_mark_block(
            -1.0+((16.0/256.0)*43 as f32), 
            -1.0+((16.0/208.0)*11 as f32),
            16.0/208.0, 
            16.0/256.0,
        );
        tile2.objects.create_block(
            -1.0+((16.0/256.0)*45 as f32), 
            -1.0+((16.0/208.0)*11 as f32),
            16.0/208.0, 
            16.0/256.0,
        );
        tile2.objects.create_question_mark_block(
            -1.0+((16.0/256.0)*45 as f32), 
            -1.0+((16.0/208.0)*19 as f32),
            16.0/208.0, 
            16.0/256.0,
        );
        tile2.objects.create_question_mark_block(
            -1.0+((16.0/256.0)*47 as f32), 
            -1.0+((16.0/208.0)*11 as f32),
            16.0/208.0, 
            16.0/256.0,
        );
        tile2.objects.create_block(
            -1.0+((16.0/256.0)*49 as f32), 
            -1.0+((16.0/208.0)*11 as f32),
            16.0/208.0, 
            16.0/256.0,
        );
        tile2.objects.create_pipe(
            -1.0+((16.0/256.0)*58 as f32), 
            -1.0+((16.0/208.0)*7 as f32),
            16.0/208.0, 
            32.0/256.0,
        );
        tiles.push(tile2);

        let tile3 = Tile::create(tiles[1].last_drawpos+2, 3, 3, 4.0);
        tiles.push(tile3);

        let tile4 = Tile::create(tiles[2].last_drawpos+2, 4, 1, 6.0);
        tiles.push(tile4);

        Self{tiles}
    }

    pub unsafe fn draw(&self) {
        unsafe {
            gl::ClearColor(0.384, 0.671, 0.831, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        for tile in self.tiles.iter() {
            tile.draw();
        }
    }
}
