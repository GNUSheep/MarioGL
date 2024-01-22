extern crate sdl2;

use glm;

mod worlds;
mod background;
mod objects;
mod mobs;
mod spirit;

use crate::render;
use std::path::Path;
use std::ffi::{CString, c_void};

pub struct Block {
    pub x: f32,
    pub y: f32,
    h: f32,
    w: f32,
    move_acc_y: f32,
    move_acc_x: f32,
    collision_event: bool,
    collision_name: String,
    collision_num: u32,
    state: usize,
    obj: render::Object,
    textures: Vec<render::Texture>,
    program: render::Program,
}

impl Block {
    pub fn create(x: f32, y: f32, h: f32, w: f32, collision_event: bool, path: &Path, collision_name: &str) -> Self {
        let points: Vec<f32> = vec![
            x+w, y+h, 0.0, 1.0, 0.0,
            x+w, y-h, 0.0, 1.0, 1.0,
            x-w, y-h, 0.0, 0.0, 1.0,
            x-w, y+h, 0.0, 0.0, 0.0
        ];

        const INDCIES: [i32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        let obj = render::Object::create_square_with_points(points, INDCIES);
        let texture = render::Texture::create_new_texture_from_file(&Path::new(path));

        let textures: Vec<render::Texture> = vec![texture];

        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("game/assets/shaders/block.vert")).unwrap(),
        ).unwrap();

        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("game/assets/shaders/block.frag")).unwrap(),
        ).unwrap();
        
        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        unsafe {
            obj.set_vertex_attrib_pointer(0, 
                3, 
                gl::FLOAT, 
                gl::FALSE, 
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint, 
                std::ptr::null()
            );
            
            obj.set_vertex_attrib_pointer(1, 
                2, 
                gl::FLOAT, 
                gl::FALSE, 
                (5 * std::mem::size_of::<f32>()) as gl::types::GLint, 
                (3 * std::mem::size_of::<f32>()) as *const c_void, 
            );
        }
        
        let collision_name = collision_name.to_string();
        let mut collision_num = 0;
        if collision_event {
            collision_num = 1;
        }
        let state = 0;
        let move_acc_y = 0.0;
        let move_acc_x = 0.0;

        Self{x, y, w, h, move_acc_y, move_acc_x, collision_event, collision_name, collision_num, state, obj, textures, program} 
    }

    pub fn check_hitbox(&self, obj: &Block) -> &str {
        let right: bool = self.x+self.w >= obj.x-obj.w;
        let left: bool = obj.x+obj.w >= self.x-self.w;
        let top: bool = self.y+self.h >= obj.y-obj.h;
        let bottom: bool = obj.y+obj.h >= self.y-self.h;
        if bottom && left && right && top {
            if obj.y < self.y {return "bottom"}
            if obj.y > self.y && (obj.x-obj.w < self.x && self.x < obj.x+obj.w) {return "top"}
            if obj.x < self.x {return "left"}
            if obj.x > self.x {return "right"};
        }
        return "nil"
    }

    pub fn check_hitbox_spirit(&self, obj: &spirit::Mario) -> &str {
        let right: bool = self.x+self.w >= obj.x-obj.w;
        let left: bool = obj.x+obj.w >= self.x-self.w;
        let top: bool = self.y+self.h >= obj.y-obj.h;
        let bottom: bool = obj.y+obj.h >= self.y-self.h;
        if bottom && left && right && top {
            if obj.y < self.y {return "bottom"}
            if obj.y > self.y && (obj.x-obj.w < self.x && self.x < obj.x+obj.w) {return "top"}
            if obj.x < self.x {return "left"}
            if obj.x > self.x {return "right"};
        }
        return "nil"
    }

    pub fn check_hitbox_question_mark_block(&self, obj: &objects::QuestionMarkBlock) -> &str {
        let right: bool = self.x+self.w >= obj.x-obj.w;
        let left: bool = obj.x+obj.w >= self.x-self.w;
        let top: bool = self.y+self.h >= obj.y-obj.h;
        let bottom: bool = obj.y+obj.h >= self.y-self.h;
        if bottom && left && right && top {
            if obj.y < self.y {return "bottom"}
            if obj.y > self.y && (obj.x-obj.w < self.x && self.x < obj.x+obj.w) {return "top"}
            if obj.x < self.x {return "left"}
            if obj.x > self.x {return "right"};
        }
        return "nil"
    }

    pub fn check_hitbox_pipe(&self, obj: &render::Object) -> &str {
        let right: bool = self.x+self.w >= obj.x-obj.w;
        let left: bool = obj.x+obj.w >= self.x-self.w;
        let top: bool = self.y+self.h >= obj.y-obj.h;
        let bottom: bool = obj.y+obj.h >= self.y-self.h;
        if bottom && left && right && top {
            if obj.y < self.y {return "bottom"}
            if obj.y > self.y && (obj.x-obj.w < self.x && self.x < obj.x+obj.w) {return "top"}
            if obj.x < self.x {return "left"}
            if obj.x > self.x {return "right"};
        }
        return "nil"
    }

    pub fn handle(&mut self, objects: &mut Vec<Block>) {
        if self.collision_event && self.collision_num != 0 {
            if self.collision_name == "block".to_string() {
                let mut block = Block::create(self.x, self.y+2.0*self.h, self.h, 8.0/256.0, true, &Path::new("src/scenes/game/assets/images/coin1.png"), "coin");

                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin2.png")));
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin3.png")));
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin4.png")));
                block.move_acc_y = 2.5;

                let vert_shader = render::Shader::vertex_from_src(
                    &CString::new(include_str!("game/assets/shaders/coin.vert")).unwrap(),
                ).unwrap();
        
                let frag_shader = render::Shader::fragment_from_src(
                    &CString::new(include_str!("game/assets/shaders/coin.frag")).unwrap(),
                ).unwrap();
        
                let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();
    
                block.program = program;

                block.x = 0.0;
                block.y = 0.0;

                objects.push(block);
            }
            else if self.collision_name == "star".to_string() {
                let mut block = Block::create(0.0, 0.0, self.h, self.w, true, &Path::new("src/scenes/game/assets/images/star1.png"), "star");
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/star2.png")));
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/star3.png")));
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/star4.png")));

                let vert_shader = render::Shader::vertex_from_src(
                    &CString::new(include_str!("game/assets/shaders/star.vert")).unwrap(),
                ).unwrap();
        
                let frag_shader = render::Shader::fragment_from_src(
                    &CString::new(include_str!("game/assets/shaders/star.frag")).unwrap(),
                ).unwrap();
        
                let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();
    
                block.program = program;

                block.x = self.x;
                block.y = self.y+2.0*self.h;
                block.move_acc_y = 3.0;
                block.move_acc_x = 1.0;

                objects.push(block);
            }

            self.collision_num -= 1;
        }

        if self.collision_num == 0 {
            self.textures[0] = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/brick-still.png"));
            self.collision_event = false;
        }

    }

    pub unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.textures[self.state].texture);
        gl::BindVertexArray(self.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub struct Game {
    world: worlds::World,
    pub spirit: spirit::Mario,
    objects_still: Vec<Block>,
    objects_inmove: Vec<Block>,
    goombas: Vec<mobs::Goomba>,
    troopas: Vec<mobs::Troopa>,
    delay: i32,
    screen_move_x: f32,
    screen_move_y: f32,
    is_over: bool,
    pub is_endlvl: bool,
    hud: render::Texts,
    hud_coin_icon: Block,
    score: u32,
    coins: u32,
    world_number: u32,
    world_level: u32,
    pub time: u32,
}

impl Game {    
    pub fn init() -> Self {      
        let world = worlds::World::init();
        
        let screen_move_x = 0.0;
        let screen_move_y = 0.0;
        let is_over = false;
        let is_endlvl = false;
        let objects_still: Vec<Block> = vec![];
        let objects_inmove: Vec<Block> = vec![];
        let mut goombas: Vec<mobs::Goomba> = vec![];
        let mut troopas: Vec<mobs::Troopa> = vec![];
        let delay = 0;

        let score = 0;
        let coins = 0;
        let world_number = 1;
        let world_level = 1;
        let time = 400;

        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*45 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*81 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*103 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*107 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*161 as f32), 
            -1.0+((16.0/240.0)*21 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*165 as f32), 
            -1.0+((16.0/240.0)*21 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*195 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*199 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        troopas.push(mobs::Troopa::create(
            -1.0+((16.0/256.0)*215 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*229 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*233 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*249 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*253 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*257 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*261 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*349 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));
        goombas.push(mobs::Goomba::create(
            -1.0+((16.0/256.0)*351 as f32), 
            -1.0+((16.0/240.0)*5 as f32),
        ));

        let mut hud = render::Texts::init();
        let mut hud_coin_icon = Block::create(-1.0+(8.0/256.0)*23.0, 1.0-(8.0/240.0)*7.0, 8.0/240.0, 8.0/256.0, false, &Path::new("src/scenes/game/assets/images/coin_icon1.png"), "coin_icon");
        hud_coin_icon.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin_icon2.png")));
        hud_coin_icon.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin_icon3.png")));

        let mut spirit = spirit::Mario::create(0.0, 0.0, 16.0/240.0, 16.0/256.0, &Path::new("src/scenes/game/assets/images/mario.png"));
        spirit.x = -0.3;
        spirit.y = -1.0+((16.0/240.0)*5 as f32);
        Self{world, spirit, objects_still, objects_inmove, goombas, troopas, delay, screen_move_x, screen_move_y, is_over, is_endlvl, hud, hud_coin_icon, score, coins, world_number, world_level, time}
    }

    pub fn jump(&mut self) {
        self.spirit.is_falling = true;
        if self.spirit.move_acc_y == 0.0 {
            self.spirit.move_acc_y = 3.0;
        }
    }

    pub fn crouch(&mut self) {
        self.spirit.is_crouch = true;
    }

    pub fn go_into_pipe(&mut self, exit: bool) {
        if exit {
            self.spirit.y = -1.0+(16.0/240.0)*(9 as f32);
            self.spirit.x = -1.0+(16.0/256.0)*(328 as f32); 
            self.spirit.is_underground = false;
            self.screen_move_y = 0.0;
            self.screen_move_x = -2.0*10.0;
            self.world.bg_color = "blue".to_string(); 
        }else {
            self.spirit.y = -2.0;
            self.spirit.x = -1.0+(16.0/256.0)*5 as f32;
            self.spirit.is_underground = true;
            self.screen_move_y = 2.07;
            self.screen_move_x = 0.0;
            self.world.bg_color = "black".to_string(); 
        }
    }

    pub fn endLevel(&mut self, deltatime: u32) {
        self.spirit.is_falling = false;
        if self.spirit.y >= -1.0+(16.0/240.0)*(7 as f32) {
            self.spirit.y -= (deltatime as f32)*0.0008;
        }else{
            self.spirit.y = -1.0+(16.0/240.0)*(5 as f32);
            if self.spirit.x <= -1.0+(16.0/256.0)*(409 as f32) {
                self.spirit.x += (deltatime as f32)*0.001;
                self.spirit.delay += 1;
                if self.spirit.delay == 5 {
                    self.spirit.delay = 0;
                    self.spirit.state += 1;
                    if self.spirit.state == 4 {
                        self.spirit.state = 0;
                    }
                }
                self.screen_move_x -= (deltatime as f32)*0.001; 
            }else{
                self.over();
            }
        }

        unsafe {
            let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
            let move_vel = gl::GetUniformLocation(self.spirit.program.program, cname.as_ptr());
            self.spirit.program.set_active();
            gl::Uniform2f(move_vel, self.spirit.x, self.spirit.y);

            let view = glm::mat4(1.0, 0.0, 0.0, self.screen_move_x,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0);

            for tile in self.world.tiles.iter() {
            let cname = std::ffi::CString::new("view").expect("CString::new failed");
            let view_loc = gl::GetUniformLocation(tile.bg.background_prog.program, cname.as_ptr());
            tile.bg.background_prog.set_active();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

            let cname = std::ffi::CString::new("view").expect("CString::new failed");
            let view_loc = gl::GetUniformLocation(tile.floor[0].program.program, cname.as_ptr());
            tile.floor[0].program.set_active();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
            }

            let cname = std::ffi::CString::new("view").expect("CString::new failed");
            let view_loc = gl::GetUniformLocation(self.spirit.program.program, cname.as_ptr());
            self.spirit.program.set_active();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
        }
    }

    pub fn move_x(&mut self, dir: &str) {
        if dir == "left" {
            self.spirit.move_vel_x = 1;
        }else{
            self.spirit.move_vel_x = -1;
        }
    }

    fn dead(&mut self) {
        let texture_dead = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario_dead.png"));
        self.spirit.textures.push(texture_dead);
        self.spirit.move_acc_y = 5.0;
        self.spirit.is_dead = true;
    }

    fn over(&mut self) {
        self.is_over = true;
        unsafe {
            gl::ClearColor(0.0, 0.0, 0.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
    }

    pub fn handle(&mut self, deltatime: u32) {
        // floor collision system
        self.spirit.is_falling = true;

        let mut go_into_pipe = false;
        for tile in self.world.tiles.iter_mut() {
            for brick in tile.floor.iter() {
                if self.spirit.check_hitbox(brick) == "bottom" {
                    self.spirit.y = brick.y+brick.h+self.spirit.h;
                    if self.spirit.move_acc_y < 0 as f32 {
                        self.spirit.move_acc_y = 0.0;
                    }
                    self.spirit.is_falling = false;
                }
                else if self.spirit.check_hitbox(brick) == "top" {
                    self.spirit.y = brick.y-brick.h-self.spirit.w;
                }
                else if self.spirit.check_hitbox(brick) == "left" {
                    self.spirit.x = brick.x+brick.w+self.spirit.w+0.01;
                }
                else if self.spirit.check_hitbox(brick) == "right" {
                    self.spirit.x = brick.x-brick.w-self.spirit.w-0.01;
                }
            }

            for stone in tile.objects.stones.iter() {
                if self.spirit.check_hitbox(stone) == "bottom" {
                    self.spirit.y = stone.y+stone.h+self.spirit.h;
                    if self.spirit.move_acc_y < 0 as f32 {
                        self.spirit.move_acc_y = 0.0;
                    }
                    self.spirit.is_falling = false;
                }else if self.spirit.check_hitbox(stone) == "top" {
                    self.spirit.y = stone.y-stone.h-self.spirit.w;
                }else if self.spirit.check_hitbox(stone) == "left" {
                    self.spirit.x = stone.x+stone.w+self.spirit.w+0.01;
                }else if self.spirit.check_hitbox(stone) == "right" {
                    self.spirit.x = stone.x-stone.w-self.spirit.w-0.01;
                }
            }

            for block in tile.objects.blocks.iter_mut() {
                if self.spirit.check_hitbox(block) == "bottom" {
                    self.spirit.y = block.y+block.h+self.spirit.h;
                    if self.spirit.move_acc_y < 0 as f32 {
                        self.spirit.move_acc_y = 0.0;
                    }
                    self.spirit.is_falling = false;
                }else if self.spirit.check_hitbox(block) == "top" {
                    self.spirit.y = block.y-block.h-self.spirit.w;

                    if block.collision_name == "star" {
                        block.handle(&mut self.objects_inmove);
                    }else {
                        block.handle(&mut self.objects_still);
                    }
                    self.spirit.move_acc_y = -1.0
                }else if self.spirit.check_hitbox(block) == "left" {
                    self.spirit.x = block.x+block.w+self.spirit.w+0.01;
                }else if self.spirit.check_hitbox(block) == "right" {
                    self.spirit.x = block.x-block.w-self.spirit.w-0.01;
                }
            }
        }

        // still objects animations and collision
        let mut index = 0; 
        let mut indexes_to_remove: Vec<usize> = vec![];
        if self.delay >= 5 {
            for obj in self.objects_still.iter_mut() {
                if obj.collision_name == "coin".to_string() {
                    obj.state += 1;
                    if obj.state == 4 {
                        obj.state = 0;
                    }
                }
            }
            for obj in self.objects_inmove.iter_mut() {
                if obj.collision_name == "star".to_string() {
                    obj.state += 1;
                    if obj.state == 4 {
                        obj.state = 0;
                    }
                }
            }
            self.delay = 0;
        }
        for obj in self.objects_still.iter_mut() {
            if obj.collision_name == "coin".to_string() {
                if obj.move_acc_y < 0.0 {
                    indexes_to_remove.push(index);
                    self.coins += 1;
                    self.score += 200;
                }
                obj.move_acc_y -= 0.15;
                obj.y += (deltatime as f32)*0.0017*obj.move_acc_y;  
            }
            index += 1;
        }

        indexes_to_remove.sort();
        indexes_to_remove.reverse();

        for index in indexes_to_remove {
            self.objects_still.remove(index);
        }
        index = 0;

        let mut index = 0; 
        let mut indexes_to_remove: Vec<usize> = vec![];

        indexes_to_remove.sort();
        indexes_to_remove.reverse();

        for index in indexes_to_remove {
            self.objects_inmove.remove(index);
        }
        index = 0;
        
        if self.world.tiles_underground[0].delay >= 10 {
            for obj in self.world.tiles_underground[0].objects.coins.iter_mut() {
                obj.state += 1;
                if obj.state == 3 {
                    obj.state = 0;
                }
            }
            self.world.tiles_underground[0].delay = 0;

            self.hud_coin_icon.state += 1;
            if self.hud_coin_icon.state == 3 {
                self.hud_coin_icon.state = 0;
            }
        }
        
        self.world.tiles_underground[0].delay += 1;
        self.delay += 1;

        if self.spirit.is_falling {
            self.spirit.move_acc_y -= 0.15;
        }

        self.spirit.y += (deltatime as f32)*0.001*self.spirit.move_acc_y;
        
        //left screen side collision
        if self.spirit.x-self.spirit.w <= -1.0-(self.screen_move_x) {
            self.spirit.x = -1.0-(self.screen_move_x)+self.spirit.w;
        }

        //if self.spirit.y-self.spirit.h <= -1.0 && !self.spirit.is_dead {
        //    self.dead();
        //}else if self.spirit.y-self.spirit.h <= -1.0 && self.spirit.is_dead {
        //    self.over();
        //}

        // moving
        if self.spirit.move_vel_x != 0 {
            if self.spirit.move_vel_x == 1 {
                self.spirit.flip = true;
            }else {
                self.spirit.flip = false;
            }

            if self.spirit.is_moving != 0 && self.spirit.is_moving != self.spirit.move_vel_x {
                self.spirit.state = 4;
                self.spirit.is_turn = true;
            }
            self.spirit.is_moving = self.spirit.move_vel_x;

            self.spirit.x -= self.spirit.move_vel_x  as f32 * ((deltatime as f32)*0.001);
            self.spirit.delay += 1;
        }else{
            self.spirit.is_moving = 0;
            self.spirit.state = 0;
        }
        self.spirit.move_vel_x = 0;

        // animation
        if self.spirit.delay == 5  {
            self.spirit.state += 1;
            self.spirit.delay = 0;
            if self.spirit.is_turn {
                self.spirit.state = 1;
                self.spirit.is_turn = false;
            }
        }

        if self.spirit.state == 4 && !self.spirit.is_turn {
            self.spirit.state = 1;
        }

        if self.spirit.x >= -0.2-(self.screen_move_x) && !self.spirit.is_underground {
            self.screen_move_x -= (deltatime as f32)*0.001; 
        }

        if self.spirit.is_dead {
            self.spirit.state = self.spirit.textures.len()-1;
            self.over();
        }


        unsafe {
            let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
            let move_vel = gl::GetUniformLocation(self.spirit.program.program, cname.as_ptr());
            self.spirit.program.set_active();
            gl::Uniform2f(move_vel, self.spirit.x, self.spirit.y);

            let cname = std::ffi::CString::new("flipTex").expect("CString::new failed");
            let flip = gl::GetUniformLocation(self.spirit.program.program, cname.as_ptr());
            self.spirit.program.set_active();
            gl::Uniform1i(flip, self.spirit.flip as i32);
        }

        let view = glm::mat4(1.0, 0.0, 0.0, self.screen_move_x,
                                 0.0, 1.0, 0.0, self.screen_move_y,
                                 0.0, 0.0, 1.0, 0.0,
                                 0.0, 0.0, 0.0, 1.0);
        
        unsafe {
            for tile in self.world.tiles.iter() {
                let cname = std::ffi::CString::new("view").expect("CString::new failed");
                let view_loc = gl::GetUniformLocation(tile.bg.background_prog.program, cname.as_ptr());
                tile.bg.background_prog.set_active();
                gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
                
                let cname = std::ffi::CString::new("view").expect("CString::new failed");
                let view_loc = gl::GetUniformLocation(tile.floor[0].program.program, cname.as_ptr());
                tile.floor[0].program.set_active();
                gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

                for goomba in self.goombas.iter() {
                    let cname = std::ffi::CString::new("view").expect("CString::new failed");
                    let view_loc = gl::GetUniformLocation(goomba.program.program, cname.as_ptr());
                    goomba.program.set_active();
                    gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

                    let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
                    let move_vel = gl::GetUniformLocation(goomba.program.program, cname.as_ptr());
                    goomba.program.set_active();
                    gl::Uniform2f(move_vel, goomba.obj.x, goomba.obj.y);
                }

                for troopa in self.troopas.iter() {
                    let cname = std::ffi::CString::new("view").expect("CString::new failed");
                    let view_loc = gl::GetUniformLocation(troopa.program.program, cname.as_ptr());
                    troopa.program.set_active();
                    gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

                    let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
                    let move_vel = gl::GetUniformLocation(troopa.program.program, cname.as_ptr());
                    troopa.program.set_active();
                    gl::Uniform2f(move_vel, troopa.obj.x, troopa.obj.y);
                }
            }

            for obj in self.objects_still.iter() {
                let cname = std::ffi::CString::new("view").expect("CString::new failed");
                let view_loc = gl::GetUniformLocation(obj.program.program, cname.as_ptr());
                obj.program.set_active();
                gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

                let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
                let move_vel = gl::GetUniformLocation(obj.program.program, cname.as_ptr());
                obj.program.set_active();
                gl::Uniform2f(move_vel, obj.x, obj.y);
            }

            for obj in self.objects_inmove.iter() {
                let cname = std::ffi::CString::new("view").expect("CString::new failed");
                let view_loc = gl::GetUniformLocation(obj.program.program, cname.as_ptr());
                obj.program.set_active();
                gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

                let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
                let move_vel = gl::GetUniformLocation(obj.program.program, cname.as_ptr());
                obj.program.set_active();
                gl::Uniform2f(move_vel, obj.x, obj.y);
            }

            let cname = std::ffi::CString::new("view").expect("CString::new failed");
            let view_loc = gl::GetUniformLocation(self.spirit.program.program, cname.as_ptr());
            self.spirit.program.set_active();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
        }

    }

    pub unsafe fn draw(&mut self) {
        if !self.is_over {
            self.world.draw();
            for obj in self.objects_inmove.iter() {
                obj.program.set_active();
                obj.draw();
            }
            for obj in self.objects_still.iter() {
                obj.program.set_active();
                obj.draw();
            }

            for obj in self.goombas.iter() {
                obj.program.set_active();
                obj.draw();
            }

            for obj in self.troopas.iter() {
                obj.program.set_active();
                obj.draw();
            }

            self.hud.add_text("mario".to_string(), -1.0+(8.0/256.0)*5.0, 1.0-(8.0/240.0)*5.0);

            let mut score_string = self.score.to_string();
            for i in 1..=6-self.score.to_string().len() {
                score_string = "0".to_string() + &score_string;
            }
            self.hud.add_text(score_string, -1.0+(8.0/256.0)*5.0, 1.0-(8.0/240.0)*7.0);
    
            let mut coins_string = self.coins.to_string();
            for i in 1..=2-self.coins.to_string().len() {
                coins_string = "0".to_string() + &coins_string;
            }
            self.hud.add_text("+".to_string() + &coins_string, -1.0+(8.0/256.0)*25.0, 1.0-(8.0/240.0)*7.0);
    
            self.hud.add_text("world".to_string(), -1.0+(8.0/256.0)*40.0, 1.0-(8.0/240.0)*5.0);
    
            self.hud.add_text(self.world_number.to_string() + "-" + &self.world_level.to_string(), -1.0+(8.0/256.0)*42.0, 1.0-(8.0/240.0)*7.0);
    
            self.hud.add_text("time".to_string(), -1.0+(8.0/256.0)*55.0, 1.0-(8.0/240.0)*5.0);
            self.hud.add_text(self.time.to_string(), -1.0+(8.0/256.0)*57.0, 1.0-(8.0/240.0)*7.0);

            self.hud_coin_icon.draw();

            self.spirit.draw();
        }
    }
}