extern crate sdl2;

use glm;
use std::rc::Rc;
use std::cell::RefCell;

mod worlds;
mod background;
mod objects;

use crate::render;
use std::path::Path;
use std::ffi::{CString, c_void};

use self::objects::Collisioner;

pub struct Spirit {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
    object_type: String,
    state: usize,
    is_falling: bool,
    pub is_dead: bool,
    is_moving: i32,
    is_turn: bool,
    is_crouch: bool,
    is_underground: bool,
    delay: i32,
    move_acc_x: i32,
    move_acc_y: f32,
    obj: render::Object,
    textures: Vec<render::Texture>,
    flip: bool,
    program: render::Program,
}

impl Spirit {
    pub fn create(x: f32, y: f32, h: f32, w: f32, path: &Path) -> Self {
        let points: Vec<f32> = vec![
            w, h, 0.0, 1.0, 0.0,
            w, -h, 0.0, 1.0, 1.0,
            -w, -h, 0.0, 0.0, 1.0,
            -w, h, 0.0, 0.0, 0.0
        ];

        const INDCIES: [i32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];

        let obj = render::Object::create_square_with_points(points, INDCIES);

        let mut textures: Vec<render::Texture> = vec![];
        textures.push(render::Texture::create_new_texture_from_file(path));
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
            obj.set_vertex_attrib_pointers()
        }
        
        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("game/assets/shaders/spirit.vert")).unwrap(),
        ).unwrap();

        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("game/assets/shaders/spirit.frag")).unwrap(),
        ).unwrap();
        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        let state = 0;
        let is_falling = true;
        let is_dead = false;
        let is_moving = 0;
        let is_turn = false;
        let is_crouch = false;
        let is_underground = false;
        let delay = 0;
        let move_acc_x = 0;
        let move_acc_y = 0.0;
        let flip = false;

        Self{x, y, h, w, object_type: "spirit".to_string(), state, is_falling, is_dead, is_moving, is_turn, is_crouch, is_underground, delay, move_acc_x, move_acc_y, obj, textures, flip, program}
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

impl objects::Collisioner for Spirit {
    fn get_collision_rect(&self) -> objects::Collision_rect {
        objects::Collision_rect{
            x: self.x, 
            y: self.y, 
            w: self.w, 
            h: self.h,
        }
    }
    fn get_type(&self) -> &String {
        &self.object_type
    }
    fn handle_collision(&mut self, collision_object: &String, collision_rect: objects::Collision_rect, collisions: Vec<char>) {
        for collision in collisions.iter() {
            match collision {
                &'b' => {
                    if collision_object == "block" || collision_object == "?block" || collision_object == "block_up" || collision_object == "stone" {
                        self.y = collision_rect.y+collision_rect.h+self.h-0.01;
                        if self.move_acc_y < 0 as f32 {
                            self.move_acc_y = 0.0;
                        }
                        self.is_falling = false;
                    }
                },
                &'t' => {
                    if collision_object == "block" || collision_object == "?block" || collision_object == "block_up" || collision_object == "stone" {
                        self.y = collision_rect.y-collision_rect.h-self.h+0.01;

                        self.move_acc_y *= -1 as f32;
                    }
                }
                &'l' => {
                    if collision_object == "block" || collision_object == "?block" || collision_object == "block_up" || collision_object == "stone" {
                        self.x = collision_rect.x+collision_rect.w+self.w+0.01;
                    }
                }
                &'r' => {
                    if collision_object == "block" || collision_object == "?block" || collision_object == "block_up" || collision_object == "stone" {
                        self.x = collision_rect.x-collision_rect.w-self.w-0.01;
                    }
                }
                _ => (),
            }
        }
    }
    fn get_collision(&mut self){}
    fn set_default_behavior(&mut self) {
        self.is_falling = true;
    }
    fn run_default_behavior(&mut self, deltatime: u32) {
        if self.is_falling {
            self.move_acc_y -= 0.10;
        }
        self.y += (deltatime as f32)*0.001*self.move_acc_y;
    }
}

impl objects::Drawer for Spirit {
    unsafe fn get_program(&self) -> &render::Program {
        &self.program
    }

    unsafe fn set_uniforms(&self, view_x: f32, view_y: f32) {
        let view = glm::mat4(1.0, 0.0, 0.0, view_x,
            0.0, 1.0, 0.0, view_y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0);
        let cname = std::ffi::CString::new("view").expect("CString::new failed");
        let view_loc = gl::GetUniformLocation(self.program.program, cname.as_ptr());
        self.program.set_active();
        gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

        let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
        let move_vel = gl::GetUniformLocation(self.program.program, cname.as_ptr());
        self.program.set_active();
        gl::Uniform2f(move_vel, self.x, self.y);

        let cname = std::ffi::CString::new("flipTex").expect("CString::new failed");
        let flip = gl::GetUniformLocation(self.program.program, cname.as_ptr());
        self.program.set_active();
        gl::Uniform1i(flip, self.flip as i32);
    }

    unsafe fn draw(&self) {
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
    pub x: f32,
    pub y: f32,
    h: f32,
    w: f32,
    object_type: String,
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
    pub fn create(x: f32, y: f32, h: f32, w: f32, collision_event: bool, path: &str, collision_name: &str, object_type: &str) -> Self {
        let points: Vec<f32> = vec![
            w,  h, 0.0, 1.0, 0.0,
            w, -h, 0.0, 1.0, 1.0,
            -w, -h, 0.0, 0.0, 1.0,
            -w, h, 0.0, 0.0, 0.0
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
            obj.set_vertex_attrib_pointers();
        }
        
        let collision_name = collision_name.to_string();
        let mut collision_num = 0;
        if collision_event {
            collision_num = 1;
        }
        let state = 0;
        let move_acc_y = 0.0;
        let move_acc_x = 0.0;

        Self{x, y, w, h, object_type: object_type.to_string(), move_acc_y, move_acc_x, collision_event, collision_name, collision_num, state, obj, textures, program} 
    }

    pub fn attach_to_main_loop(
        self,
        mut collisions_objects: &mut Vec<Rc<RefCell<dyn objects::Collisioner>>>,
        mut objects_draw: &mut Vec<Rc<RefCell<dyn objects::Drawer>>>,
    ){
        let block_rc: Rc<RefCell<Block>> = Rc::new(RefCell::new(self));

        let block_drawer: Rc<RefCell<dyn objects::Drawer>> = block_rc.clone();
        objects_draw.push(block_drawer);

        let block_collisioner: Rc<RefCell<dyn Collisioner>> = block_rc.clone();
        collisions_objects.push(block_collisioner);
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

    pub fn check_hitbox_spirit(&self, obj: &Spirit) -> &str {
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
                let mut block = Block::create(self.x, self.y+2.0*self.h, self.h, 8.0/256.0, true, "src/scenes/game/assets/images/coin1.png", "coin", "coin");

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
                let mut block = Block::create(0.0, 0.0, self.h, self.w, true, "src/scenes/game/assets/images/star1.png", "star", "star");
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

impl objects::Collisioner for Block {
    fn get_collision_rect(&self) -> objects::Collision_rect {
        objects::Collision_rect{
            x: self.x, 
            y: self.y, 
            w: self.w, 
            h: self.h,
        }
    }
    fn get_type(&self) -> &String {
        &self.object_type
    }
    fn handle_collision(&mut self, collision_object: &String, collision_rect: objects::Collision_rect, collisions: Vec<char>){}
    fn get_collision(&mut self){}
    fn set_default_behavior(&mut self){}
    fn run_default_behavior(&mut self, deltatime: u32){}
}

impl objects::Drawer for Block {
    unsafe fn get_program(&self) -> &render::Program {
        &self.program
    }

    unsafe fn set_uniforms(&self, view_x: f32, view_y: f32) {
        let view = glm::mat4(1.0, 0.0, 0.0, view_x,
            0.0, 1.0, 0.0, view_y,
            0.0, 0.0, 1.0, 0.0,
            0.0, 0.0, 0.0, 1.0);
        let cname = std::ffi::CString::new("view").expect("CString::new failed");
        let view_loc = gl::GetUniformLocation(self.program.program, cname.as_ptr());
        self.program.set_active();
        gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);

        let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
        let move_vel = gl::GetUniformLocation(self.program.program, cname.as_ptr());
        self.program.set_active();
        gl::Uniform2f(move_vel, self.x, self.y);
    }

    unsafe fn draw(&self) {
        self.program.set_active();
        gl::BindTexture(gl::TEXTURE_2D, self.textures[self.state].texture);
        gl::BindVertexArray(self.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub struct Game {
    world: worlds::World,
    pub spirit: Rc<RefCell<Spirit>>,
    objects_still: Vec<Block>,
    objects_inmove: Vec<Block>,
    goombas: Vec<objects::Goomba>,
    troopas: Vec<objects::Troopa>,
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
    pub collisions_objects: Vec<Rc<RefCell<dyn objects::Collisioner>>>,
    pub objects_draw: Vec<Rc<RefCell<dyn objects::Drawer>>>,
}

impl Game {    
    pub fn init() -> Self {
        let mut collisions_objects: Vec<Rc<RefCell<dyn objects::Collisioner>>> = vec![];
        let mut objects_draw: Vec<Rc<RefCell<dyn objects::Drawer>>> = vec![];

        let world = worlds::World::init(&mut collisions_objects, &mut objects_draw);
        
        let screen_move_x = 0.0;
        let screen_move_y = 0.0;
        let is_over = false;
        let is_endlvl = false;
        let objects_still: Vec<Block> = vec![];
        let objects_inmove: Vec<Block> = vec![];
        let mut goombas: Vec<objects::Goomba> = vec![];
        let mut troopas: Vec<objects::Troopa> = vec![];
        let delay = 0;

        let score = 0;
        let coins = 0;
        let world_number = 1;
        let world_level = 1;
        let time = 400;

        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*45 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*81 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*103 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*107 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*161 as f32), 
        //    -1.0+((16.0/240.0)*21 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*165 as f32), 
        //    -1.0+((16.0/240.0)*21 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*195 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*199 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //troopas.push(objects::Troopa::create(
        //    -1.0+((16.0/256.0)*215 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*229 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*233 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*249 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*253 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*257 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*261 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*349 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
        //goombas.push(objects::Goomba::create(
        //    -1.0+((16.0/256.0)*351 as f32), 
        //    -1.0+((16.0/240.0)*5 as f32),
        //));
//
        let mut hud = render::Texts::init();
        let mut hud_coin_icon = Block::create(-1.0+(8.0/256.0)*23.0, 1.0-(8.0/240.0)*7.0, 8.0/240.0, 8.0/256.0, false, "src/scenes/game/assets/images/coin_icon1.png", "coin_icon", "nill");
        hud_coin_icon.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin_icon2.png")));
        hud_coin_icon.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin_icon3.png")));

        let mut spirit_obj = Spirit::create(-0.3, -1.0+((16.0/240.0)*7 as f32), 16.0/240.0, 16.0/256.0, &Path::new("src/scenes/game/assets/images/mario.png"));
        let spirit: Rc<RefCell<Spirit>> = Rc::new(RefCell::new(spirit_obj));

        let spirit_drawer: Rc<RefCell<dyn objects::Drawer>> = spirit.clone();
        objects_draw.push(spirit_drawer);

        let spirit_collisioner: Rc<RefCell<dyn objects::Collisioner>> = spirit.clone();
        collisions_objects.push(spirit_collisioner);

        Self{world, spirit, objects_still, objects_inmove, goombas, troopas, delay, screen_move_x, screen_move_y, is_over, is_endlvl, hud, hud_coin_icon, score, coins, world_number, world_level, time, collisions_objects, objects_draw}
    }

    pub fn jump(&mut self) {
        self.spirit.borrow_mut().is_falling = true;
        if self.spirit.borrow_mut().move_acc_y == 0.0 {
            self.spirit.borrow_mut().move_acc_y = 3.0;
        }
    }

    pub fn crouch(&mut self) {
        self.spirit.borrow_mut().is_crouch = true;
    }

    pub fn go_into_pipe(&mut self, exit: bool) {
        if exit {
            self.spirit.borrow_mut().y = -1.0+(16.0/240.0)*(9 as f32);
            self.spirit.borrow_mut().x = -1.0+(16.0/256.0)*(328 as f32); 
            self.spirit.borrow_mut().is_underground = false;
            self.screen_move_y = 0.0;
            self.screen_move_x = -2.0*10.0;
            self.world.bg_color = "blue".to_string(); 
        }else {
            self.spirit.borrow_mut().y = -2.0;
            self.spirit.borrow_mut().x = -1.0+(16.0/256.0)*5 as f32;
            self.spirit.borrow_mut().is_underground = true;
            self.screen_move_y = 2.07;
            self.screen_move_x = 0.0;
            self.world.bg_color = "black".to_string(); 
        }
    }

    pub fn endLevel(&mut self, deltatime: u32) {
        self.spirit.borrow_mut().is_falling = false;
        if self.spirit.borrow_mut().y >= -1.0+(16.0/240.0)*(7 as f32) {
            self.spirit.borrow_mut().y -= (deltatime as f32)*0.0008;
        }else{
            self.spirit.borrow_mut().y = -1.0+(16.0/240.0)*(5 as f32);
            if self.spirit.borrow_mut().x <= -1.0+(16.0/256.0)*(409 as f32) {
                self.spirit.borrow_mut().x += (deltatime as f32)*0.001;
                self.spirit.borrow_mut().delay += 1;
                if self.spirit.borrow_mut().delay == 5 {
                    self.spirit.borrow_mut().delay = 0;
                    self.spirit.borrow_mut().state += 1;
                    if self.spirit.borrow_mut().state == 4 {
                        self.spirit.borrow_mut().state = 0;
                    }
                }
                self.screen_move_x -= (deltatime as f32)*0.001; 
            }else{
                self.over();
            }
        }

        unsafe {
            let cname = std::ffi::CString::new("movePos").expect("CString::new failed");
            let move_vel = gl::GetUniformLocation(self.spirit.borrow_mut().program.program, cname.as_ptr());
            self.spirit.borrow_mut().program.set_active();
            gl::Uniform2f(move_vel, self.spirit.borrow_mut().x, self.spirit.borrow_mut().y);

            let view = glm::mat4(1.0, 0.0, 0.0, self.screen_move_x,
                0.0, 1.0, 0.0, 0.0,
                0.0, 0.0, 1.0, 0.0,
                0.0, 0.0, 0.0, 1.0);

            for tile in self.world.tiles.iter() {
            let cname = std::ffi::CString::new("view").expect("CString::new failed");
            let view_loc = gl::GetUniformLocation(tile.bg.background_prog.program, cname.as_ptr());
            tile.bg.background_prog.set_active();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
            }

            let cname = std::ffi::CString::new("view").expect("CString::new failed");
            let view_loc = gl::GetUniformLocation(self.spirit.borrow_mut().program.program, cname.as_ptr());
            self.spirit.borrow_mut().program.set_active();
            gl::UniformMatrix4fv(view_loc, 1, gl::FALSE, &view[0][0]);
        }
    }

    pub fn move_x(&mut self, dir: &str) {
        if dir == "left" {
            self.spirit.borrow_mut().move_acc_x = 1;
        }else{
            self.spirit.borrow_mut().move_acc_x = -1;
        }
    }

    fn dead(&mut self) {
        let texture_dead = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario_dead.png"));
        self.spirit.borrow_mut().textures.push(texture_dead);
        self.spirit.borrow_mut().move_acc_y = 5.0;
        self.spirit.borrow_mut().is_dead = true;
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
        for obj_first in &self.collisions_objects {
            obj_first.try_borrow_mut().unwrap().set_default_behavior();
        }

        for obj_sec in &self.collisions_objects {
            let collisions: Vec<char>;
            let obj_sec_ref = obj_sec.borrow();
            if self.spirit.try_borrow_mut().is_ok() == false {
                continue;
            }
            collisions = self.spirit.borrow_mut().get_collision_rect().check_hitbox(obj_sec_ref.get_collision_rect());
            self.spirit.borrow_mut().handle_collision(obj_sec_ref.get_type(), obj_sec_ref.get_collision_rect(), collisions);
        }

        for obj_first in &self.collisions_objects {
            obj_first.try_borrow_mut().unwrap().run_default_behavior(deltatime);
        }

        let mut go_into_pipe = false;
        for tile in self.world.tiles.iter_mut() {
            let mut sprt = self.spirit.borrow_mut();

            for flag in tile.objects.flag.iter() {
                for obj in flag.objects.iter() {
                    if  sprt.check_hitbox_pipe(obj) == "bottom" || 
                    sprt.check_hitbox_pipe(obj) == "top" ||
                    sprt.check_hitbox_pipe(obj) == "left" ||
                    sprt.check_hitbox_pipe(obj) == "right"  
                    { 
                        self.is_endlvl = true;
                        sprt.delay = 0;
                    }
                }
            }
        }
        
        if go_into_pipe {
            self.spirit.borrow_mut().is_crouch = false;
            self.go_into_pipe(false);
        }
        go_into_pipe = false;
        self.spirit.borrow_mut().is_crouch = false;

        let mut indexes_to_remove: Vec<usize> = vec![];
        let mut index = 0;
        for tile in self.world.tiles_underground.iter() {
        let mut sprt = self.spirit.borrow_mut();            
            for coin in tile.objects.coins.iter() {
                if  sprt.check_hitbox(coin) == "bottom" ||
                    sprt.check_hitbox(coin) == "top" ||
                    sprt.check_hitbox(coin) == "left" ||
                    sprt.check_hitbox(coin) == "right"  
                {   
                    indexes_to_remove.push(index);
                    self.coins += 1;
                    self.score += 200;
                }
                index += 1;
            }
        }

        indexes_to_remove.sort();
        indexes_to_remove.reverse();

        for index in indexes_to_remove {
            self.world.tiles_underground[0].objects.coins.remove(index);
        }

        if go_into_pipe {
            self.go_into_pipe(true);
        }
        self.spirit.borrow_mut().is_crouch = false;
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
        
        let mut indexes_to_remove: Vec<usize> = vec![];
        let mut index = 0;
        for goomba in self.goombas.iter_mut() {
            let obj_falling = true;
            // TODO: goombas collision with eachother

            for troopa in self.troopas.iter() {
                if goomba.obj.check_hitbox(&troopa.obj) == "left" || goomba.obj.check_hitbox(&troopa.obj) == "right" {
                    goomba.squash();
                    goomba.delay = 0;
                }
            }

            if self.spirit.borrow_mut().check_hitbox(&goomba.obj) == "bottom" && !self.spirit.borrow_mut().is_dead {
                goomba.squash();
                self.score += 100;
                self.spirit.borrow_mut().move_acc_y = 1.5;
                goomba.delay = 0;
                break;
            }

            if self.spirit.borrow_mut().check_hitbox(&goomba.obj) == "top" ||
                self.spirit.borrow_mut().check_hitbox(&goomba.obj) == "left" ||
                self.spirit.borrow_mut().check_hitbox(&goomba.obj) == "right" {
                 self.spirit.borrow_mut().is_dead = true;
            }
            if obj_falling {
                goomba.obj.move_acc_y -= 0.15;
            }
            goomba.obj.y += (deltatime as f32)*0.001*goomba.obj.move_acc_y; 

            if self.spirit.borrow_mut().x+1.5 >= goomba.obj.x {
                goomba.to_move = true;
            }
            if goomba.to_move && !goomba.is_squash {
                goomba.obj.x += (deltatime as f32)*0.0005*goomba.obj.move_acc_x;
            }

            if goomba.delay >= 10 && !goomba.is_squash {
                goomba.state += 1;
                if goomba.state == 2 {
                    goomba.state = 0;
                }
                goomba.delay = 0;
            }
            goomba.delay += 1;

            if goomba.is_squash {
                goomba.state = 0;
                if goomba.delay >= 10 {
                    indexes_to_remove.push(index);
                }
            }
            index += 1;
        }

        indexes_to_remove.sort();
        indexes_to_remove.reverse();

        for index in indexes_to_remove {
            self.goombas.remove(index);
        }
        index = 0;

        for troopa in self.troopas.iter_mut() {
            let mut obj_falling = true;

            if self.spirit.borrow_mut().check_hitbox(&troopa.obj) == "bottom" && !self.spirit.borrow_mut().is_dead && !troopa.is_squash {
                troopa.squash();
                self.score += 100;
                self.spirit.borrow_mut().move_acc_y = 1.5;
                troopa.delay = 0;
                troopa.state = 0;
            }
            
            if self.spirit.borrow_mut().check_hitbox(&troopa.obj) == "bottom" && troopa.is_squash && troopa.obj.check_hitbox_spirit(&self.spirit.borrow_mut()) == "right" && !troopa.to_move_squash {
                self.spirit.borrow_mut().move_acc_y = 1.5;
                troopa.obj.move_acc_x = -1 as f32;
                troopa.to_move_squash = true;
            }else if self.spirit.borrow_mut().check_hitbox(&troopa.obj) == "bottom" && troopa.is_squash && !troopa.to_move_squash {
                self.spirit.borrow_mut().move_acc_y = 1.5;
                troopa.obj.move_acc_x = 1 as f32;
                troopa.to_move_squash = true;
            }
            
            if (self.spirit.borrow_mut().check_hitbox(&troopa.obj) == "top" || self.spirit.borrow_mut().check_hitbox(&troopa.obj) == "left" || self.spirit.borrow_mut().check_hitbox(&troopa.obj) == "right") && (!troopa.is_squash || troopa.to_move_squash) {
                 self.spirit.borrow_mut().is_dead = true;
            }

            if obj_falling {
                troopa.obj.move_acc_y -= 0.15;
            }
            troopa.obj.y += (deltatime as f32)*0.001*troopa.obj.move_acc_y; 

            if self.spirit.borrow_mut().x+1.5 >= troopa.obj.x {
                troopa.to_move = true;
            }
            if troopa.to_move && !troopa.is_squash {
                troopa.obj.x -= (deltatime as f32)*0.0005;
            }
            if troopa.to_move_squash {
                troopa.obj.x += (deltatime as f32)*0.002*troopa.obj.move_acc_x;
            }


            if troopa.delay >= 10 && !troopa.is_squash {
                troopa.state += 1;
                if troopa.state == 2 {
                    troopa.state = 0;
                }
                troopa.delay = 0;
            }
            troopa.delay += 1;
        }

        let mut index = 0; 
        let mut indexes_to_remove: Vec<usize> = vec![];
        for obj in self.objects_inmove.iter_mut() {
            if obj.collision_name == "mushroom".to_string() {
                let mut obj_falling = true;
                obj.x += (deltatime as f32)*0.0008*obj.move_acc_x;
                
                if self.spirit.borrow_mut().check_hitbox(obj) == "bottom" ||
                    self.spirit.borrow_mut().check_hitbox(obj) == "top" ||
                    self.spirit.borrow_mut().check_hitbox(obj) == "left" ||
                    self.spirit.borrow_mut().check_hitbox(obj) == "right" {
                        indexes_to_remove.push(index);
                }
                
                if obj_falling {
                    obj.move_acc_y -= 0.15;
                }
                obj.y += (deltatime as f32)*0.001*obj.move_acc_y; 
            } else if obj.collision_name == "star".to_string() {
                let mut obj_falling = true;
                obj.x += (deltatime as f32)*0.0008*obj.move_acc_x;
                
                if self.spirit.borrow_mut().check_hitbox(obj) == "bottom" ||
                    self.spirit.borrow_mut().check_hitbox(obj) == "top" ||
                    self.spirit.borrow_mut().check_hitbox(obj) == "left" ||
                    self.spirit.borrow_mut().check_hitbox(obj) == "right" {
                        indexes_to_remove.push(index);
                }

                if obj_falling {
                    obj.move_acc_y -= 0.15;
                }
                obj.y += (deltatime as f32)*0.001*obj.move_acc_y; 
            }
            index += 1;
        }

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

        if self.spirit.borrow_mut().is_falling {
            self.spirit.borrow_mut().move_acc_y -= 0.15;
        }
        if self.spirit.try_borrow_mut().is_ok() {
            let mut sprt = self.spirit.borrow_mut();
            sprt.y += (deltatime as f32)*0.001*sprt.move_acc_y;
        }
        
        //left screen side collision
        {
            let mut sprt = self.spirit.borrow_mut();
            if sprt.x-sprt.w <= -1.0-(self.screen_move_x) {
                sprt.x = -1.0-(self.screen_move_x)+sprt.w;
            }
        }

        //if self.spirit.borrow_mut().y-self.spirit.borrow_mut().h <= -1.0 && !self.spirit.borrow_mut().is_dead {
        //    self.dead();
        //}else if self.spirit.borrow_mut().y-self.spirit.borrow_mut().h <= -1.0 && self.spirit.borrow_mut().is_dead {
        //    self.over();
        //}

        // moving
        if self.spirit.borrow().move_acc_x != 0 {
            let mut sprt = self.spirit.borrow_mut();
            if sprt.move_acc_x == 1 {
                sprt.flip = true;
            }else {
                sprt.flip = false;
            }

            if sprt.is_moving != 0 && sprt.is_moving != sprt.move_acc_x {
                sprt.state = 4;
                sprt.is_turn = true;
            }
            sprt.is_moving = sprt.move_acc_x;

            sprt.x -= sprt.move_acc_x  as f32 * ((deltatime as f32)*0.001);
            sprt.delay += 1;
        }else{
            self.spirit.borrow_mut().is_moving = 0;
            self.spirit.borrow_mut().state = 0;
        }
        self.spirit.borrow_mut().move_acc_x = 0;

        // animation
        if self.spirit.borrow_mut().delay == 5  {
            self.spirit.borrow_mut().state += 1;
            self.spirit.borrow_mut().delay = 0;
            if self.spirit.borrow_mut().is_turn {
                self.spirit.borrow_mut().state = 1;
                self.spirit.borrow_mut().is_turn = false;
            }
        }

        if self.spirit.borrow_mut().state == 4 && !self.spirit.borrow_mut().is_turn {
            self.spirit.borrow_mut().state = 1;
        }

        if self.spirit.borrow_mut().x >= -0.2-(self.screen_move_x) && !self.spirit.borrow_mut().is_underground {
            self.screen_move_x -= (deltatime as f32)*0.001; 
        }

        if self.spirit.borrow_mut().is_dead {
            self.spirit.borrow_mut().state = self.spirit.borrow_mut().textures.len()-1;
            self.over();
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
        }
    }

    pub unsafe fn draw(&mut self) {
        if !self.is_over {
            self.world.draw();
            for obj_ref in self.objects_draw.iter() {
                let obj = obj_ref.as_ref().borrow();
                obj.set_uniforms(self.screen_move_x, self.screen_move_y);
                obj.draw();
            }

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

            self.spirit.borrow_mut().draw();
        }
    }
}