extern crate sdl2;

use glm;

mod worlds;
mod background;
mod objects;

use crate::render;
use std::path::Path;
use std::ffi::{CString, c_void};

struct Spirit {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
    state: usize,
    is_falling: bool,
    delay: i32,
    move_vel_x: i32,
    move_vel_y: i32,
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
        let texture1 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario-move1.png"));
        textures.push(texture1);
        let texture2 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario-move2.png"));
        textures.push(texture2);
        let texture3 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario-move3.png"));
        textures.push(texture3);

        let texture_jump = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/mario-jump.png"));
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
        let move_vel_y = -1;
        let state = 0;
        let is_falling = true;
        let delay = 0;
        let move_acc_y = 0.0;
        let flip = false;
         
        Self{x, y, h, w, state, is_falling, delay, move_vel_x, move_vel_y, move_acc_y, obj, textures, flip, program}
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
        if self.is_falling {
            gl::BindTexture(gl::TEXTURE_2D, self.textures[4].texture);
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
    obj: render::Object,
    texture: render::Texture,
    program: render::Program,
}

impl Block {
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

        let texture = render::Texture::create_new_texture_from_file(&Path::new(path));

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
        
        Self{x, y, w, h, obj, texture, program} 
    }

    pub unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.texture.texture);
        gl::BindVertexArray(self.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub struct Game {
    world: worlds::World,
    spirit: Spirit,
    screen_move: f32,
}

impl Game {    
    pub fn init() -> Self {      
        let world = worlds::World::init();
        
        let screen_move = 0.0;

        let spirit = Spirit::create(0.0, 0.0, 16.0/208.0, 16.0/256.0, &Path::new("src/scenes/game/assets/images/mario-still.png"));
        Self{world, spirit, screen_move}
    }

    pub fn jump(&mut self) {
        self.spirit.is_falling = true;
        if self.spirit.move_acc_y == 0.0 {
            self.spirit.move_acc_y = 2.0;
        }
    }

    pub fn move_x(&mut self, dir: &str) {
        if dir == "left" {
            self.spirit.move_vel_x = 1;
        }else{
            self.spirit.move_vel_x = -1;
        }
    }

    pub fn handle(&mut self, deltatime: u32) {
        // brick collison falling
        self.spirit.is_falling = true;

        for tile in self.world.tiles.iter() {
            for brick in tile.floor.iter() {
                if self.spirit.check_hitbox(brick) == "bottom" {
                    self.spirit.y = brick.y+brick.h+self.spirit.h;
                    if self.spirit.move_acc_y < 0 as f32 {
                        self.spirit.move_acc_y = 0.0;
                    }
                    self.spirit.is_falling = false;
                }
            }
        } 
        
        //for brick in self.bricks_up.iter() {
        //    if self.spirit.check_hitbox(brick) == "bottom" {
        //        self.spirit.y = brick.y+brick.h+self.spirit.h;
        //        if self.spirit.move_acc_y < 0 as f32 {
        //            self.spirit.move_acc_y = 0.0;
        //        }
        //        self.spirit.is_falling = false;
        //    }
        //    else if self.spirit.check_hitbox(brick) == "top" {
        //        self.spirit.y = brick.y-brick.h-self.spirit.w;
        //    }
        //    else if self.spirit.check_hitbox(brick) == "left" {
        //        self.spirit.x = brick.x+brick.w+self.spirit.w+0.01;
        //    }
        //    else if self.spirit.check_hitbox(brick) == "right" {
        //        self.spirit.x = brick.x-brick.w-self.spirit.w-0.01;
        //    }
        //}

        if self.spirit.is_falling {
            self.spirit.move_acc_y -= 0.15;
        }

        if self.spirit.move_acc_y > 0.0 {
            self.spirit.move_vel_y = 1;
        }else {
            self.spirit.move_vel_y = -1;
        }

        self.spirit.y += (deltatime as f32)*0.001*self.spirit.move_acc_y;
        
        //left screen side collison
        if self.spirit.x-self.spirit.w <= -1.0-(self.screen_move) {
            self.spirit.x = -1.0-(self.screen_move)+self.spirit.w;
        }

        // moving

        if self.spirit.move_vel_x != 0 {
            if self.spirit.move_vel_x == 1 {
                self.spirit.flip = true;
            }else {
                self.spirit.flip = false;
            }
            self.spirit.x -= self.spirit.move_vel_x  as f32 * ((deltatime as f32)*0.001);
            self.spirit.delay += 1;
        }else{
            self.spirit.state = 0;
        }
        self.spirit.move_vel_x = 0;

        // animation
        if self.spirit.delay == 5 {
            self.spirit.state += 1;
            self.spirit.delay = 0;
        }

        if self.spirit.state == 4 {
            self.spirit.state = 1;
        }

        if self.spirit.x >= 0.5-(self.screen_move) {
            self.screen_move -= (deltatime as f32)*0.001; 
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
        self.world.draw();

        //for brick in self.bricks_up.iter() {
        //    unsafe {
         //       brick.draw();
          //  }
        //}
        
        self.spirit.draw();
    }
}