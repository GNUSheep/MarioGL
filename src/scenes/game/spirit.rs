use crate::render;
use std::path::Path;
use std::ffi::{CString, c_void};
use crate::scenes::{game, game::objects};

pub struct Mario {
    pub x: f32,
    pub y: f32,
    pub h: f32,
    pub w: f32,
    pub state: usize,
    pub is_falling: bool,
    pub is_dead: bool,
    pub is_moving: i32,
    pub is_turn: bool,
    pub is_crouch: bool,
    pub is_underground: bool,
    pub delay: i32,
    pub move_vel_x: i32,
    pub move_acc_y: f32,
    pub obj: render::Object,
    pub textures: Vec<render::Texture>,
    pub flip: bool,
    pub program: render::Program,
}

impl Mario {
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
            &CString::new(include_str!("assets/shaders/mario.vert")).unwrap(),
        ).unwrap();

        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/mario.frag")).unwrap(),
        ).unwrap();

        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        let move_vel_x = 0;
        let state = 0;
        let is_falling = true;
        let is_dead = false;
        let is_moving = 0;
        let is_turn = false;
        let is_crouch = false;
        let is_underground = false;
        let delay = 0;
        let move_acc_y = 0.0;
        let flip = false;
         
        Self{x, y, h, w, state, is_falling, is_dead, is_moving, is_turn, is_crouch, is_underground, delay, move_vel_x, move_acc_y, obj, textures, flip, program}
    }

    pub fn check_hitbox(&self, obj: &game::Block) -> &str {
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