extern crate sdl2;

use glm;

mod worlds;
mod background;
mod objects;

use crate::render;
use std::path::Path;
use std::ffi::{CString, c_void};

pub struct Spirit {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
    state: usize,
    is_falling: bool,
    pub is_dead: bool,
    is_moving: i32,
    is_turn: bool,
    delay: i32,
    move_vel_x: i32,
    move_acc_y: f32,
    obj: render::Object,
    textures: Vec<render::Texture>,
    flip: bool,
    program: render::Program,
}

impl Spirit {
    pub fn create(x: f32, y: f32, h: f32, w: f32, path: &Path) -> Self {
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

        let mut textures: Vec<render::Texture> = vec![];

        let texture0 = render::Texture::create_new_texture_from_file(path);
        textures.push(texture0);
        let texture1 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario_move1.png"));
        textures.push(texture1);
        let texture2 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario_move2.png"));
        textures.push(texture2);
        let texture3 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario_move3.png"));
        textures.push(texture3);

        let texture_turn = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario_turn.png"));
        textures.push(texture_turn);

        let texture_jump = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario_jump.png"));
        textures.push(texture_jump);

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
        
        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("game/assets/shaders/spirit.vert")).unwrap(),
        ).unwrap();

        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("game/assets/shaders/spirit.frag")).unwrap(),
        ).unwrap();

        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        let move_vel_x = 0;
        let state = 0;
        let is_falling = true;
        let is_dead = false;
        let is_moving = 0;
        let is_turn = false;
        let delay = 0;
        let move_acc_y = 0.0;
        let flip = false;
         
        Self{x, y, h, w, state, is_falling, is_dead, is_moving, is_turn, delay, move_vel_x, move_acc_y, obj, textures, flip, program}
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

    pub unsafe fn draw(&self) {
        self.program.set_active();
        if self.is_falling && !self.is_dead {
            gl::BindTexture(gl::TEXTURE_2D, self.textures[5].texture);
        }else {
            gl::BindTexture(gl::TEXTURE_2D, self.textures[self.state].texture);
        }
        gl::BindVertexArray(self.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub struct Block {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
    move_acc_y: f32,
    collision_event: bool,
    name: String,
    state: usize,
    obj: render::Object,
    textures: Vec<render::Texture>,
    program: render::Program,
}

impl Block {
    pub fn create(x: f32, y: f32, h: f32, w: f32, collision_event: bool, path: &Path, name: &str) -> Self {
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
        
        let name = "block".to_string();
        let state = 0;
        let move_acc_y = 0.0;

        Self{x, y, w, h, move_acc_y, collision_event, name, state, obj, textures, program} 
    }

    pub fn handle(&mut self, objects: &mut Vec<Block>) {
        if self.collision_event {
            let mut block = Block::create(self.x, self.y+2.0*self.h, self.h, 8.0/256.0, true, &Path::new("src/scenes/game/assets/images/coin1.png"), "coin");

            block.name = "coin".to_string();
            block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin2.png")));
            block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin3.png")));
            block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin4.png")));
            block.move_acc_y = 1.0;

            objects.push(block);
        }
        self.textures[0] = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/brick-still.png"));
        self.collision_event = false;
    }

    pub unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.textures[self.state].texture);
        gl::BindVertexArray(self.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub struct Game {
    world: worlds::World,
    pub spirit: Spirit,
    object_still: Vec<Block>,
    delay: i32,
    screen_move: f32,
    is_over: bool,
}

impl Game {    
    pub fn init() -> Self {      
        let world = worlds::World::init();
        
        let screen_move = 0.0;
        let is_over = false;
        let object_still: Vec<Block> = vec![];
        let delay = 0;

        let spirit = Spirit::create(0.0, 0.0, 16.0/208.0, 16.0/256.0, &Path::new("src/scenes/game/assets/images/mario.png"));
        Self{world, spirit, object_still, delay, screen_move, is_over}
    }

    pub fn jump(&mut self) {
        self.spirit.is_falling = true;
        if self.spirit.move_acc_y == 0.0 {
            self.spirit.move_acc_y = 3.0;
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

            for block in tile.objects.blocks.iter_mut() {
                if self.spirit.check_hitbox(block) == "bottom" {
                    self.spirit.y = block.y+block.h+self.spirit.h;
                    if self.spirit.move_acc_y < 0 as f32 {
                        self.spirit.move_acc_y = 0.0;
                    }
                    self.spirit.is_falling = false;
                }
                else if self.spirit.check_hitbox(block) == "top" {
                    self.spirit.y = block.y-block.h-self.spirit.w;

                    block.handle(&mut self.object_still);
                }
                else if self.spirit.check_hitbox(block) == "left" {
                    self.spirit.x = block.x+block.w+self.spirit.w+0.01;
                }
                else if self.spirit.check_hitbox(block) == "right" {
                    self.spirit.x = block.x-block.w-self.spirit.w-0.01;
                }
            }
        }


        // still objects animations and collision
        let mut index = 0; 
        let mut indexes_to_remove: Vec<usize> = vec![];
        for obj in self.object_still.iter_mut() {   
            if obj.name == "coin".to_string() {
                if self.delay >= 8 {
                    obj.state += 1;
                    if obj.state == 4 {
                        obj.state = 0;
                    }
                    self.delay = 0;
                }
            }
            index += 1;
        }

        for index in indexes_to_remove {
            self.object_still.remove(index);
        }

        index = 0;
        self.delay += 1;
        
        if self.spirit.is_falling {
            self.spirit.move_acc_y -= 0.15;
        }

        self.spirit.y += (deltatime as f32)*0.001*self.spirit.move_acc_y;
        
        //left screen side collision
        if self.spirit.x-self.spirit.w <= -1.0-(self.screen_move) {
            self.spirit.x = -1.0-(self.screen_move)+self.spirit.w;
        }

        if self.spirit.y-self.spirit.h <= -1.0 && !self.spirit.is_dead {
            self.dead();
        }else if self.spirit.y-self.spirit.h <= -1.0 && self.spirit.is_dead {
            self.over();
        }

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

        if self.spirit.x >= 0.5-(self.screen_move) {
            self.screen_move -= (deltatime as f32)*0.001; 
        }

        if self.spirit.is_dead {
            self.spirit.state = self.spirit.textures.len()-1;
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

        let view = glm::mat4(1.0, 0.0, 0.0, self.screen_move,
                                 0.0, 1.0, 0.0, 0.0,
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
                
            }

            let cname = std::ffi::CString::new("view").expect("CString::new failed");
            let view_loc = gl::GetUniformLocation(self.spirit.program.program, cname.as_ptr());
            self.spirit.program.set_active();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
        }

    }

    pub unsafe fn draw(&self) {
        if !self.is_over {
            self.world.draw();
            for obj in self.object_still.iter() {
                obj.draw();
            }
            self.spirit.draw();

        }
    }
}