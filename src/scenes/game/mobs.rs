use crate::render;
use crate::scenes::game;
use crate::scenes::game::objects;
use std::path::Path;
use std::ffi::{CString, c_void};


pub struct Troopa {
    pub delay: usize,
    pub state: usize,
    pub obj: game::Block,
    pub to_move: bool,
    pub is_squash: bool,
    pub to_move_squash: bool,
    pub program: render::Program,
}

impl Troopa {
    pub fn create(x: f32, y: f32) -> Self {
        let mut obj = game::Block::create(0.0, 0.0, 24.0/240.0, 16.0/256.0, false, &Path::new("src/scenes/game/assets/images/troopa1.png"), "troopa");

        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("assets/shaders/troopa.vert")).unwrap(),
        ).unwrap();
        
        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/troopa.frag")).unwrap(),
        ).unwrap();
        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        obj.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/troopa2.png")));
        obj.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/troopa_squash.png")));
        
        obj.x = x;
        obj.y = y;

        obj.move_acc_x = -1.0;

        let state = 0;
        let delay = 0;
        let to_move = false;
        let is_squash = false;
        let to_move_squash = false;

        Self{delay, state, obj, to_move, is_squash, to_move_squash, program}
    }

    pub fn squash(&mut self) {
        let x = self.obj.x;
        let y = self.obj.y;
        self.obj = game::Block::create(0.0, 0.0, 16.0/240.0, 16.0/256.0, false, &Path::new("src/scenes/game/assets/images/troopa_squash.png"), "goomba");
        self.obj.x = x;
        self.obj.y = y;

        self.is_squash = true;
    } 

    pub unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.obj.textures[self.state].texture);
        gl::BindVertexArray(self.obj.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}

pub struct Goomba {
    pub delay: usize,
    pub state: usize,
    pub obj: game::Block,
    pub to_move: bool,
    pub is_squash: bool,
    pub program: render::Program,
}

impl Goomba {
    pub fn create(x: f32, y: f32) -> Self {
        let mut obj = game::Block::create(0.0, 0.0, 16.0/240.0, 16.0/256.0, false, &Path::new("src/scenes/game/assets/images/goomba1.png"), "goomba");

        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("assets/shaders/goomba.vert")).unwrap(),
        ).unwrap();
        
        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/goomba.frag")).unwrap(),
        ).unwrap();
        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        obj.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/goomba2.png")));
        obj.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/goomba_squash.png")));
        
        obj.x = x;
        obj.y = y;

        obj.move_acc_x = -1.0;

        let state = 0;
        let delay = 0;
        let to_move = false;
        let is_squash = false;

        Self{delay, state, obj, to_move, is_squash, program}
    }

    pub fn squash(&mut self) {
        let x = self.obj.x;
        let y = self.obj.y;
        self.obj = game::Block::create(0.0, 0.0, 8.0/240.0, 16.0/256.0, false, &Path::new("src/scenes/game/assets/images/goomba_squash.png"), "goomba");
        self.obj.x = x;
        self.obj.y = y;

        self.state = 0;

        self.is_squash = true;
    } 

    pub unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.obj.textures[self.state].texture);
        gl::BindVertexArray(self.obj.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}