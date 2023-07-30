use gl;
use image;
use image::EncodableLayout;
use std;
use std::ffi::{CString, CStr, c_void};
use std::path::Path;
use std::collections::HashMap;
use crate::scenes::game;

pub struct Texts {
    objects: Vec<Object>,
    texture: Texture,
    program: Program,
    chars: Vec<char>,
    positions: Vec<[f32; 8]>,
}

impl Texts {
    pub fn init() -> Self {
        let chars: Vec<char> = vec!['0','1','2','3','4','5','6','7','8','9','a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o','p', 'q', 'r', 's', 't', 'u', 'v', 'w', 'x', 'y', 'z'];
        let mut positions = vec![[0.0; 8]; 36];
        
        let mut y_bitmap = 0;
        let mut i = 0;
        for mut j in 0..=35 {
            if i == 16 {
                y_bitmap += 1;
                i = 0;
            }

            positions[i+y_bitmap*16][0] = (8.0+9.0*i as f32)/143.0;
            positions[i+y_bitmap*16][1] = (0.0+9.0*y_bitmap as f32)/26.0;
            positions[i+y_bitmap*16][2] = (8.0+9.0*i as f32)/143.0;
            positions[i+y_bitmap*16][3] = (8.0+9.0*y_bitmap as f32)/26.0;
            positions[i+y_bitmap*16][4] = (0.0+9.0*i as f32)/143.0;
            positions[i+y_bitmap*16][5] = (8.0+9.0*y_bitmap as f32)/26.0;
            positions[i+y_bitmap*16][6] = (0.0+9.0*i as f32)/143.0;
            positions[i+y_bitmap*16][7] = (0.0+9.0*y_bitmap as f32)/26.0;

            i += 1;
        }

        let bitmap: HashMap<_, _> = chars.iter().zip(positions.iter()).collect();
        let texture = Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/font_bitmap.png"));
        let mut objects: Vec<Object> = vec![];
        
        let vert_shader = Shader::vertex_from_src(
            &CString::new(include_str!("scenes/game/assets/shaders/text.vert")).unwrap(),
        ).unwrap();

        let frag_shader = Shader::fragment_from_src(
            &CString::new(include_str!("scenes/game/assets/shaders/text.frag")).unwrap(),
        ).unwrap();
        
        let program = Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        Self{objects, texture, program, chars, positions}
    }

    pub fn add_text(&mut self, text: String, x: f32, y: f32) {
        let bitmap: HashMap<_, _> = self.chars.iter().zip(self.positions.iter()).collect();

        let text_vec_char: Vec<_> = text.to_lowercase().chars().collect();
        const INDCIES: [i32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];
    
        let mut index = 0;
        for c in text_vec_char.iter() {
            let pos = bitmap.get(c).unwrap();
            let points: Vec<f32> = vec![
                (x+(8.0/256.0)*index as f32)+8.0/256.0, y+8.0/240.0, 0.0, pos[0], pos[1],
                (x+(8.0/256.0)*index as f32)+8.0/256.0, y-8.0/240.0, 0.0, pos[2], pos[3],
                (x+(8.0/256.0)*index as f32)-8.0/256.0, y-8.0/240.0, 0.0, pos[4], pos[5],
                (x+(8.0/256.0)*index as f32)-8.0/256.0, y+8.0/240.0, 0.0, pos[6], pos[7]
            ];

            index += 2;
            let obj = Object::create_square_with_points(points, INDCIES);
            self.objects.push(obj);
        }

        unsafe {
            for obj in self.objects.iter() {
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
        }
    }

    pub unsafe fn draw(&self) {
        self.program.set_active();
        gl::BindTexture(gl::TEXTURE_2D, self.texture.texture);
        for obj in self.objects.iter() {
            gl::BindVertexArray(obj.vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

pub struct Texture {
    pub texture: gl::types::GLuint,
}

impl Texture {
    pub fn create_new_texture_from_file(file_path: &Path) -> Self {
        let mut texture: gl::types::GLuint = 0;
        unsafe {
            gl::GenTextures(1, &mut texture);
        }
        let img_data = image::open(file_path).unwrap().into_rgba8();

        unsafe {
            gl::BindTexture(gl::TEXTURE_2D, texture);
            gl::TexImage2D(
                gl::TEXTURE_2D,
                0,
                gl::RGBA as i32,
                img_data.width() as i32,
                img_data.height() as i32,
                0,
                gl::RGBA,
                gl::UNSIGNED_BYTE,
                img_data.as_bytes().as_ptr() as *const _,
            );
            gl::GenerateMipmap(gl::TEXTURE_2D);

            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_S, gl::CLAMP_TO_EDGE as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_WRAP_T, gl::CLAMP_TO_EDGE  as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER, gl::LINEAR as gl::types::GLint);
            gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER, gl::LINEAR as gl::types::GLint);
        
            
            gl::BlendFunc(gl::SRC_ALPHA, gl::ONE_MINUS_SRC_ALPHA);
            gl::Enable(gl::BLEND);
        }
        
        Self{texture}
    }
}

pub struct Object {
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
    pub x: f32,
    pub y: f32,
    pub h: f32,
    pub w: f32,
}

impl Object {
    //pub fn create_triangles_with_points(points: Vec<f32>) -> Self {
    //    let mut vao: gl::types::GLuint = 0;
    //    unsafe {
    //        gl::GenVertexArrays(1, &mut vao);
    //    }
    //    unsafe {
    //        gl::BindVertexArray(vao);
    //    }
    //    
    //    let mut vbo: gl::types::GLuint = 0;
    //    unsafe {
    //        gl::GenBuffers(1, &mut vbo);
    //    }
    //    unsafe {
    //        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
    //        gl::BufferData (
    //            gl::ARRAY_BUFFER,
    //            (points.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
    //            points.as_ptr() as *const gl::types::GLvoid,
    //            gl::STATIC_DRAW,
    //        );
    //    }
    //    unsafe {
    //        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
    //        gl::BindVertexArray(0);
    //    }
    //    Self{vao, vbo}
    //}

    pub fn create_square_with_points(points: Vec<f32>, indices: [i32; 6]) -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
        }

        let mut ebo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut ebo);
        }
        
        unsafe {
            gl::BindVertexArray(vao);
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData (
                gl::ARRAY_BUFFER,
                (points.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                points.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        unsafe {
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ebo);
            gl::BufferData(
                gl::ELEMENT_ARRAY_BUFFER, 
                (indices.len() * std::mem::size_of::<f32>()) as gl::types::GLsizeiptr,
                indices.as_ptr() as *const gl::types::GLvoid,
                gl::STATIC_DRAW,
            );
        }

        unsafe {
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, 0);
        }

        // default options for objects that use collision by being Block class
        let x = -1.0;
        let y = -1.0;
        let h = 0.0;
        let w = 0.0;

        Self{vao, vbo, x, y, h, w}
    }

    pub fn set_cordinates(&mut self, x: f32, y: f32, h: f32, w: f32) {
        self.x = x;
        self.y = y;
        self.h = h;
        self.w = w;
    }

    pub unsafe fn set_vertex_attrib_pointer(
        &self,
        loc: gl::types::GLuint, 
        size: gl::types::GLint, 
        data_type: gl::types::GLenum,
        normalized: gl::types::GLboolean,
        stride: gl::types::GLsizei,
        pointer: *const c_void,
    ) {
        gl::BindVertexArray(self.vao);
        gl::BindBuffer(gl::ARRAY_BUFFER, self.vbo);
        gl::VertexAttribPointer(
            loc,
            size,
            data_type,
            normalized,
            stride,
            pointer,
        );
        gl::EnableVertexAttribArray(loc);
        gl::BindBuffer(gl::ARRAY_BUFFER, 0);
        gl::BindVertexArray(0);
    }
}


pub struct Program {
    pub program: gl::types::GLuint,
}

impl Program {
    pub fn create_with_shaders(shaders: &[Shader]) -> Result<Program, String> {
        let program = unsafe {gl::CreateProgram()};

        for shader in shaders {
            unsafe {gl::AttachShader(program, shader.id())};
        }

        unsafe {gl::LinkProgram(program)};

        for shader in shaders {
            unsafe { gl::DetachShader(program, shader.id()); }
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl::GetProgramiv(program, gl::LINK_STATUS, &mut success);
        }
        if success == 0 {
            let mut err_len: gl::types::GLint = 0;
            unsafe {
                gl::GetProgramiv(program, gl::INFO_LOG_LENGTH, &mut err_len);
            }

            let error_log = create_empty_log_buffer(err_len as usize);

            unsafe {
                gl::GetProgramInfoLog(program, err_len, std::ptr::null_mut(), error_log.as_ptr() as *mut gl::types::GLchar);
            }

            return Err(error_log.to_string_lossy().into_owned());
        }

        Ok(Program{program})
    }

    pub fn set_active(&self) {
        unsafe {
            gl::UseProgram(self.program);
        }
    }
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteProgram(self.program);
        }
    }
}

fn create_empty_log_buffer(len: usize) -> CString {
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    buffer.extend([b' '].iter().cycle().take(len));
    unsafe { CString::from_vec_unchecked(buffer)}
}

fn create_shader_from_src(src: &CStr, shader_type: gl::types::GLuint) -> Result<gl::types::GLuint, String> {
    let shader = unsafe {gl::CreateShader(shader_type)};

    unsafe {
        gl::ShaderSource(shader, 1, &src.as_ptr(), std::ptr::null());
        gl::CompileShader(shader);
    }

    let mut success: gl::types::GLint = 1;
    unsafe {
        gl::GetShaderiv(shader, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 {
        let mut err_len: gl::types::GLint = 0;
        unsafe {
            gl::GetShaderiv(shader, gl::INFO_LOG_LENGTH, &mut err_len);
        }

        let error_log = create_empty_log_buffer(err_len as usize);

        unsafe {
            gl::GetShaderInfoLog(shader, err_len, std::ptr::null_mut(), error_log.as_ptr() as *mut gl::types::GLchar);
        }

        return Err(error_log.to_string_lossy().into_owned());
    }

    Ok(shader)
}

pub struct Shader {
    id: gl::types::GLuint,
}

impl Shader {
    pub fn vertex_from_src(src: &CStr) -> Result<Shader, String> {
        let id = create_shader_from_src(src, gl::VERTEX_SHADER)?;
        Ok(Shader{id})
    }

    pub fn fragment_from_src(src: &CStr) -> Result<Shader, String> {
        let id = create_shader_from_src(src, gl::FRAGMENT_SHADER)?;
        Ok(Shader{id})
    }

    pub fn id(&self) -> gl::types::GLuint{
        self.id
    }
}

impl Drop for Shader {
    fn drop(&mut self) {
        unsafe {
            gl::DeleteShader(self.id);
        }
    }
}