use gl;
use image;
use image::EncodableLayout;
use std;
use std::ffi::{CString, CStr, c_void};
use std::path::Path;

pub struct Texture {
    pub texture: gl::types::GLuint,
}

impl Texture {
    pub fn create_new_texture_from_file(file_path: &Path) -> Texture {
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
        
        return Texture{texture}
    }
}

pub struct Object {
    pub vao: gl::types::GLuint,
    pub vbo: gl::types::GLuint,
}

impl Object {
    pub fn create_triangles_with_points(points: Vec<f32>) -> Self {
        let mut vao: gl::types::GLuint = 0;
        unsafe {
            gl::GenVertexArrays(1, &mut vao);
        }
        unsafe {
            gl::BindVertexArray(vao);
        }
        
        let mut vbo: gl::types::GLuint = 0;
        unsafe {
            gl::GenBuffers(1, &mut vbo);
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
            gl::BindBuffer(gl::ARRAY_BUFFER, 0);
            gl::BindVertexArray(0);
        }
        Self{vao, vbo}
    }

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
        Self{vao, vbo}
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