use crate::scenes::game;
use crate::scenes::game::background;
use crate::scenes::game::objects;
use std::path::Path;
use std::rc::Rc;
use std::cell::RefCell;

pub struct TileUnderground {
    pub objects: objects::Objects,
    pub delay: i32,
}

impl TileUnderground {
    fn create(
        mut collisions_objects: &mut Vec<Rc<RefCell<dyn objects::Collisioner>>>,
        mut objects_draw: &mut Vec<Rc<RefCell<dyn objects::Drawer>>>,
    ) -> Self {
        for j in (1..=3).step_by(2)  {
            for i in (1..=32).step_by(2) {
                let block = game::Block::create(
                    -1.0+((16.0/256.0)*i as f32), 
                    -1.0-((16.0/240.0)*(27+j) as f32), 
                    16.0/240.0, 
                    16.0/256.0,
                    false,
                    "src/scenes/game/assets/images/stone_underground.png",
                    "block",
                    "block"
                );
                block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
            }
        }
        for i in (6..=26).rev().step_by(2) {
            let block = game::Block::create(
                -1.0+(16.0/256.0), 
                -1.0-((16.0/240.0)*i as f32), 
                16.0/240.0, 
                16.0/256.0,
                false,
                "src/scenes/game/assets/images/brick_underground.png",
                "block",
                "block"
            );
            block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        }

        // TODO REFACTORIZE CODE
        let pipe = objects::Pipe::create_sidepipe(
            -1.0+((16.0/256.0)*27 as f32),
            -1.0-((16.0/240.0)*25 as f32),
            16.0/240.0, 
            16.0/256.0,
            9,
        );
        pipe.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);

        let objects = objects::Objects::init();
        let delay = 0;

        Self{objects, delay}
    }

    unsafe fn draw(&self) {
        self.objects.draw();
    }
}

pub struct Tile {
    pub bg: background::Background,
    pub objects: objects::Objects,
    move_by: f32,
    bg_index: u32,
    last_drawpos: i32,
}

impl Tile {
    fn create(
        last_pos: i32, 
        add: i32, 
        mut bg_index: u32, 
        mut move_by: f32,
        floor_hole: Vec<i32>, 
        collisions_objects: &mut Vec<Rc<RefCell<dyn objects::Collisioner>>>,
        objects_draw: &mut Vec<Rc<RefCell<dyn objects::Drawer>>>,
    ) -> Self {
        let bg = background::Background::init(move_by, -1.0+((16.0/240.0)*13 as f32), 208.0/240.0, 1.0, bg_index); 

        let mut last_drawpos = 0;

        let mut counter = 0;
        let mut block_index = 1;
        for j in (1..=3).step_by(2)  {
            for i in (last_pos..=32*add).step_by(2) {
                if floor_hole.len() != 0 && floor_hole.len() != counter && floor_hole[counter] == block_index {
                    counter += 1;
                }else{
                    let stone = game::Block::create(
                        -1.0+((16.0/256.0)*i as f32), 
                        -1.0+((16.0/240.0)*j as f32), 
                        16.0/240.0, 
                        16.0/256.0,
                        false,
                        "src/scenes/game/assets/images/stone.png",
                        "block",
                        "block"
                    );
                    let floor_brick_rc: Rc<RefCell<game::Block>> = Rc::new(RefCell::new(stone));

                    let floor_brick_drawer: Rc<RefCell<dyn objects::Drawer>> = floor_brick_rc.clone();
                    objects_draw.push(floor_brick_drawer);

                    let floor_brick_collisioner: Rc<RefCell<dyn objects::Collisioner>> = floor_brick_rc.clone();
                    collisions_objects.push(floor_brick_collisioner);
                }
                last_drawpos = i;
                block_index += 1;
            }
        }

        let objects = objects::Objects::init();

        bg_index += 1;
        if bg_index == 4 {
            bg_index = 1;
        }
        move_by += 2.0;

        Self{bg, objects, move_by, bg_index, last_drawpos}
    }

    unsafe fn draw(&self) {
        self.bg.background_prog.set_active();
        self.bg.draw();

        self.objects.draw()
    }
}

pub struct World {
    pub tiles: Vec<Tile>,
    pub tiles_underground: Vec<TileUnderground>,
    pub bg_color: String,
}

impl World {
    pub fn init(
        mut collisions_objects: &mut Vec<Rc<RefCell<dyn objects::Collisioner>>>,
        mut objects_draw: &mut Vec<Rc<RefCell<dyn objects::Drawer>>>,
    ) -> Self {
        let mut tiles: Vec<Tile> = vec![];
        
        let mut tiles_underground: Vec<TileUnderground> = vec![];
        let mut tile_underground1 = TileUnderground::create(&mut collisions_objects, &mut objects_draw);
        for j in (1..=3).step_by(2)  {
            for i in (9..=21).step_by(2) {
                let block = game::Block::create(
                    -1.0+((16.0/256.0)*i as f32), 
                    -1.0-((16.0/240.0)*(23+j) as f32), 
                    16.0/240.0, 
                    16.0/256.0,
                    false,
                    "src/scenes/game/assets/images/brick_underground.png",
                    "block",
                    "block"
                );
                block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
            }
        }
        for i in (9..=21).step_by(2) {
            let block = game::Block::create(
                -1.0+((16.0/256.0)*i as f32), 
                -1.0-((16.0/240.0)*6 as f32), 
                16.0/240.0, 
                16.0/256.0,
                false,
                "src/scenes/game/assets/images/brick_underground.png",
                "block_up",
                "block_up"
            );
            block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        }
        for i in (9..=21).step_by(2) {
            tile_underground1.objects.create_coin(
                -1.0+((16.0/256.0)*i as f32), 
                -1.0-((16.0/240.0)*22 as f32), 
            );
        }
        for i in (9..=21).step_by(2) {
            tile_underground1.objects.create_coin(
                -1.0+((16.0/256.0)*i as f32), 
                -1.0-((16.0/240.0)*18 as f32), 
            );
        }
        for i in (11..=19).step_by(2) {
            tile_underground1.objects.create_coin(
                -1.0+((16.0/256.0)*i as f32), 
                -1.0-((16.0/240.0)*14 as f32), 
            );
        }
        tiles_underground.push(tile_underground1);

        let floor_hole: Vec<i32> = vec![]; 

        let tile1 = Tile::create(1, 1, 1, 0.0, floor_hole, &mut collisions_objects, &mut objects_draw);
        tiles.push(tile1);
        
        let floor_hole: Vec<i32> = vec![]; 

        let mut tile2 = Tile::create(tiles[0].last_drawpos+2, 2, tiles[0].bg_index, tiles[0].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+((16.0/256.0)*33 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+((16.0/256.0)*41 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+((16.0/256.0)*43 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "mushroom".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+((16.0/256.0)*45 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+((16.0/256.0)*45 as f32), 
            -1.0+((16.0/240.0)*19 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+((16.0/256.0)*47 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+((16.0/256.0)*49 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let pipe = objects::Pipe::create_pipe(
            -1.0+((16.0/256.0)*58 as f32), 
            -1.0+((16.0/240.0)*7 as f32),
            16.0/240.0, 
            32.0/256.0,
            1,
            true,
            false,
        );
        pipe.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        tiles.push(tile2);

        let floor_hole: Vec<i32> = vec![]; 

        let mut tile3 = Tile::create(tiles[1].last_drawpos+2, 3, tiles[1].bg_index, tiles[1].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let pipe = objects::Pipe::create_pipe(
            -1.0+((16.0/256.0)*78 as f32), 
            -1.0+((16.0/240.0)*9 as f32),
            16.0/240.0, 
            32.0/256.0,
            2,
            true,
            false,
        );
        pipe.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let pipe = objects::Pipe::create_pipe(
            -1.0+((16.0/256.0)*94 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            32.0/256.0,
            3,
            true,
            false,
        );
        pipe.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        tiles.push(tile3);

        let floor_hole: Vec<i32> = vec![]; 

        let mut tile4 = Tile::create(tiles[2].last_drawpos+2, 4, tiles[2].bg_index, tiles[2].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let pipe = objects::Pipe::create_pipe(
            -1.0+((16.0/256.0)*116 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            32.0/256.0,
            3,
            true,
            true,
        );
        pipe.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        tiles.push(tile4);

        let floor_hole: Vec<i32> = vec![5, 6, 21, 22]; 

        let mut tile5 = Tile::create(tiles[3].last_drawpos+2, 5, tiles[3].bg_index, tiles[3].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+((16.0/256.0)*155 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+((16.0/256.0)*157 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "mushroom".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+((16.0/256.0)*159 as f32), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        tiles.push(tile5);

        let floor_hole: Vec<i32> = vec![7, 8, 9, 23, 24, 25];

        let mut tile6 = Tile::create(tiles[4].last_drawpos+2, 6, tiles[4].bg_index, tiles[4].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        for i in (0..=7*2).step_by(2) {
            let block = game::Block::create(
                -1.0+((16.0/256.0)*((161+i) as f32)), 
                -1.0+((16.0/240.0)*19 as f32),
                16.0/240.0, 
                16.0/256.0,
                false,
                "src/scenes/game/assets/images/brick.png",
                "block_up",
                "block_up"
            );
            block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        }
        for i in (0..=2*2).step_by(2) {
            let block = game::Block::create(
                -1.0+((16.0/256.0)*((183+i) as f32)), 
                -1.0+((16.0/240.0)*19 as f32),
                16.0/240.0, 
                16.0/256.0,
                false,
                "src/scenes/game/assets/images/brick.png",
                "block_up",
                "block_up"
            );
            block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        }
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+((16.0/256.0)*189 as f32), 
            -1.0+((16.0/240.0)*19 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let mut block = game::Block::create(
            -1.0+((16.0/256.0)*(189 as f32)), 
            -1.0+((16.0/240.0)*11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.collision_num = 10;
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        tiles.push(tile6);

        let floor_hole: Vec<i32> = vec![];

        let mut tile7 = Tile::create(tiles[5].last_drawpos+2, 7, tiles[5].bg_index, tiles[5].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(201 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let mut block = game::Block::create(
            -1.0+(16.0/256.0)*(203 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.collision_name = "star".to_string();
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+(16.0/256.0)*(213 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "mushroom".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+(16.0/256.0)*(219 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+(16.0/256.0)*(219 as f32), 
            -1.0+(16.0/240.0)*(19 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        tiles.push(tile7);

        let floor_hole: Vec<i32> = vec![];

        let mut tile8 = Tile::create(tiles[6].last_drawpos+2, 8, tiles[6].bg_index, tiles[6].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+(16.0/256.0)*(225 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(237 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        for i in (0..=2*2).step_by(2) {
            let block = game::Block::create(
                -1.0+(16.0/256.0)*((243+i) as f32), 
                -1.0+(16.0/240.0)*(19 as f32),
                16.0/240.0, 
                16.0/256.0,
                false,
                "src/scenes/game/assets/images/brick.png",
                "block_up",
                "block_up"
            );
            block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        }
        tiles.push(tile8);

        let floor_hole: Vec<i32> = vec![];

        let mut tile9 = Tile::create(tiles[7].last_drawpos+2, 9, tiles[7].bg_index, tiles[7].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(257 as f32), 
            -1.0+(16.0/240.0)*(19 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+(16.0/256.0)*(259 as f32), 
            -1.0+(16.0/240.0)*(19 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+(16.0/256.0)*(261 as f32), 
            -1.0+(16.0/240.0)*(19 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(263 as f32), 
            -1.0+(16.0/240.0)*(19 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(259 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(261 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        for i in (0..=(3*2)).step_by(2) {
            for j in (0..=3*2-i).step_by(2) {
                let stone = game::Block::create(
                    -1.0+(16.0/256.0)*((275-i) as f32), 
                    -1.0+(16.0/240.0)*((5+j) as f32),
                    16.0/240.0, 
                    16.0/256.0,
                    false, 
                    "src/scenes/game/assets/images/stone_up.png",
                    "stone",
                    "stone",
                );
                stone.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
            }
        }
        for i in (0..=(3*2)).step_by(2) {
            for j in (0..=3*2-i).step_by(2) {
                let stone = game::Block::create(
                    -1.0+(16.0/256.0)*((281+i) as f32), 
                    -1.0+(16.0/240.0)*((5+j) as f32),
                    16.0/240.0, 
                    16.0/256.0,
                    false, 
                    "src/scenes/game/assets/images/stone_up.png",
                    "stone",
                    "stone",
                );
                stone.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
            }
        }
        tiles.push(tile9);

        let floor_hole: Vec<i32> = vec![10, 11, 26, 27];

        let mut tile10 = Tile::create(tiles[8].last_drawpos+2, 10, tiles[8].bg_index, tiles[8].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        for i in (0..=3*2).step_by(2) {
            let stone = game::Block::create(
                -1.0+(16.0/256.0)*(305 as f32), 
                -1.0+(16.0/240.0)*((5+i) as f32),
                16.0/240.0, 
                16.0/256.0,
                false, 
                "src/scenes/game/assets/images/stone_up.png",
                "stone",
                "stone",
            );
            stone.attach_to_main_loop(&mut collisions_objects, &mut objects_draw)
        }
        for i in (0..=(3*2)).step_by(2) {
            for j in (0..=3*2-i).step_by(2) {
                let stone = game::Block::create(
                    -1.0+(16.0/256.0)*((303-i) as f32), 
                    -1.0+(16.0/240.0)*((5+j) as f32),
                    16.0/240.0, 
                    16.0/256.0,
                    false, 
                "src/scenes/game/assets/images/stone_up.png",
                "stone",
                "stone",
                );
                stone.attach_to_main_loop(&mut collisions_objects, &mut objects_draw)
            }
        }

        for i in (0..=(3*2)).step_by(2) {
            for j in (0..=3*2-i).step_by(2) {
                let stone = game::Block::create(
                    -1.0+(16.0/256.0)*((311+i) as f32), 
                    -1.0+(16.0/240.0)*((5+j) as f32),
                    16.0/240.0, 
                    16.0/256.0,
                    false, 
                "src/scenes/game/assets/images/stone_up.png",
                "stone",
                "stone",
                );
                stone.attach_to_main_loop(&mut collisions_objects, &mut objects_draw)
            }
        }
        tiles.push(tile10);

        let floor_hole: Vec<i32> = vec![];

        let mut tile11 = Tile::create(tiles[9].last_drawpos+2, 11, tiles[9].bg_index, tiles[9].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let pipe = objects::Pipe::create_pipe(
            -1.0+(16.0/256.0)*(328 as f32), 
            -1.0+(16.0/240.0)*(7 as f32),
            16.0/240.0, 
            32.0/256.0,
            1,
            true,
            false,
        );
        pipe.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(337 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(339 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let question_mark_block = objects::QuestionMarkBlock::create(
            -1.0+(16.0/256.0)*(341 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            true,
            "coin".to_string(),
        );
        question_mark_block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        let block = game::Block::create(
            -1.0+(16.0/256.0)*(343 as f32), 
            -1.0+(16.0/240.0)*(11 as f32),
            16.0/240.0, 
            16.0/256.0,
            false,
            "src/scenes/game/assets/images/brick.png",
            "block_up",
            "block_up"
        );
        block.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        tiles.push(tile11);

        let floor_hole: Vec<i32> = vec![];

        let mut tile12 = Tile::create(tiles[10].last_drawpos+2, 12, tiles[10].bg_index, tiles[10].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        let pipe = objects::Pipe::create_pipe(
            -1.0+(16.0/256.0)*(360 as f32), 
            -1.0+(16.0/240.0)*(7 as f32),
            16.0/240.0, 
            32.0/256.0,
            1,
            true,
            false,
        );
        pipe.attach_to_main_loop(&mut collisions_objects, &mut objects_draw);
        for i in (0..=7*2).step_by(2) {
            let stone = game::Block::create(
                -1.0+(16.0/256.0)*(379 as f32), 
                -1.0+(16.0/240.0)*((5+i) as f32),
                16.0/240.0, 
                16.0/256.0,
                false, 
                "src/scenes/game/assets/images/stone_up.png",
                "stone",
                "stone",
            );
            stone.attach_to_main_loop(&mut collisions_objects, &mut objects_draw)
        }
        for i in (0..=(8*2)).step_by(2) {
            for j in (0..=7*2-i).step_by(2) {
                let stone = game::Block::create(
                    -1.0+(16.0/256.0)*((377-i) as f32), 
                    -1.0+(16.0/240.0)*((5+j) as f32),
                    16.0/240.0, 
                    16.0/256.0,
                    false, 
                "src/scenes/game/assets/images/stone_up.png",
                "stone",
                "stone",
                );
                stone.attach_to_main_loop(&mut collisions_objects, &mut objects_draw)
            }
        }
        tiles.push(tile12);

        let floor_hole: Vec<i32> = vec![];

        let mut tile13 = Tile::create(tiles[11].last_drawpos+2, 13, tiles[11].bg_index, tiles[11].move_by, floor_hole, &mut collisions_objects, &mut objects_draw);
        tile13.objects.create_flag(
            -1.0+(16.0/256.0)*(397 as f32), 
            -1.0+(16.0/240.0)*(5 as f32),
        );
        tile13.objects.create_castle(
            -1.0+(16.0/256.0)*(409 as f32), 
            -1.0+(16.0/240.0)*(9 as f32),
            "small",
        );
        tiles.push(tile13);

        let bg_color = "blue".to_string();

        Self{tiles, tiles_underground, bg_color}
    }

    pub unsafe fn draw(&self) {
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            if self.bg_color == "blue" {
                gl::ClearColor(0.384, 0.671, 0.831, 1.0);
            }
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        for tile in self.tiles.iter() {
            tile.draw();
        }

        for tile in self.tiles_underground.iter() {
            tile.draw();
        }
    }
}
