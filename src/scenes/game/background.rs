use std::path::Path;
use gl;

use crate::render;
use std::ffi::{CString, c_void};

pub struct Background {
    obj: render::Object,
    pub background_prog: render::Program,
    texture: render::Texture,
}

impl Background {
    pub fn init(x: f32, y: f32, h: f32, w: f32, which_bg: u32) -> Self {
        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("assets/shaders/background.vert")).unwrap(),
        ).unwrap();

        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/background.frag")).unwrap(),
        ).unwrap();

        let background_prog = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

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

        let texture: render::Texture;
        if which_bg == 1 {
            texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/background.png"));
        }else if which_bg == 2 {
            texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/background2.png"));
        }else {
            texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/background3.png"));
        }

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
        

        Self{obj, background_prog, texture}
    }

    pub unsafe fn draw(&self) {
        gl::BindTexture(gl::TEXTURE_2D, self.texture.texture);
        gl::BindVertexArray(self.obj.vao);
        gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
    }
}