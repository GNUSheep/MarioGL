use crate::render;
use crate::scenes::game;
use std::path::Path;
use std::ffi::{CString, c_void};

pub struct Flag {
    x: f32,
    y: f32,
    stone: game::Block,
    objects: Vec<render::Object>,
    textures: Vec<render::Texture>,
    program: render::Program,
}

impl Flag {
    fn create(x: f32, y: f32) -> Self {
        const INDCIES: [i32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];
        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("assets/shaders/block.vert")).unwrap(),
        ).unwrap();
        
        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/block.frag")).unwrap(),
        ).unwrap();
        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut textures: Vec<render::Texture> = vec![];
        let mut objects: Vec<render::Object> = vec![];

        let stone = game::Block::create(x, y, 16.0/208.0, 16.0/256.0, false, &Path::new("src/scenes/game/assets/images/stone_up.png"), "flag");
        
        let h = 16.0/208.0;
        let w = 16.0/256.0;
        let mut offset = 1.0;
        for i in (0..=8){
            let points: Vec<f32> = vec![
                x+w, y+(offset*h), 0.0, 1.0, 0.0,
                x+w, y+((offset+2.0)*h), 0.0, 1.0, 1.0,
                x-w, y+((offset+2.0)*h), 0.0, 0.0, 1.0,
                x-w, y+(offset*h), 0.0, 0.0, 0.0
            ];

            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/flag_pipe.png"));
            let obj = render::Object::create_square_with_points(points, INDCIES);
            textures.push(texture);
            objects.push(obj);

            offset += 2.0;
        }

        offset -= 2.0;

        let points: Vec<f32> = vec![
            x, y+(offset*h), 0.0, 1.0, 0.0,
            x, y+((offset+2.0)*h), 0.0, 1.0, 1.0,
            x-(2.0*w), y+((offset+2.0)*h), 0.0, 0.0, 1.0,
            x-(2.0*w), y+(offset*h), 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/flag.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        offset += 2.0;

        let points: Vec<f32> = vec![
            x+w, y+(offset*h), 0.0, 1.0, 0.0,
            x+w, y+((offset+2.0)*h), 0.0, 1.0, 1.0,
            x-w, y+((offset+2.0)*h), 0.0, 0.0, 1.0,
            x-w, y+(offset*h), 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/flag_ball.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        unsafe {
            for obj in objects.iter() {
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

        Self{x, y, stone, objects, textures, program}
    }

    pub unsafe fn draw(&self) {
        self.stone.draw();
        for i in 0..=self.objects.len()-1 {
            gl::BindTexture(gl::TEXTURE_2D, self.textures[i].texture);
            gl::BindVertexArray(self.objects[i].vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

pub struct Pipe {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
    pipe_len: usize,
    objects: Vec<render::Object>,
    textures: Vec<render::Texture>,
    program: render::Program,
}

impl Pipe {
    fn create(x: f32, y: f32, h: f32, w: f32, pipe_len: usize) -> Self {
        const INDCIES: [i32; 6] = [
            0, 1, 2,
            2, 3, 0
        ];
        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("assets/shaders/block.vert")).unwrap(),
        ).unwrap();
        
        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/block.frag")).unwrap(),
        ).unwrap();
        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        let mut textures: Vec<render::Texture> = vec![];
        let mut objects: Vec<render::Object> = vec![];

        let points: Vec<f32> = vec![
            x+w, y+h, 0.0, 1.0, 0.0,
            x+w, y-h, 0.0, 1.0, 1.0,
            x, y-h, 0.0, 0.0, 1.0,
            x, y+h, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_enter_right.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x-w, y+h, 0.0, 1.0, 0.0,
            x-w, y-h, 0.0, 1.0, 1.0,
            x, y-h, 0.0, 0.0, 1.0,
            x, y+h, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_enter_left.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        let mut offset = 1.0;
        for i in (1..=pipe_len) {    
            let points: Vec<f32> = vec![
                x+w, y-(offset*h), 0.0, 1.0, 0.0,
                x+w, y-((offset+2.0)*h), 0.0, 1.0, 1.0,
                x, y-((offset+2.0)*h), 0.0, 0.0, 1.0,
                x, y-(offset*h), 0.0, 0.0, 0.0
            ];

            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_right.png"));
            let obj = render::Object::create_square_with_points(points, INDCIES);
            textures.push(texture);
            objects.push(obj);

            
            let points: Vec<f32> = vec![
                x-w, y-(offset*h), 0.0, 1.0, 0.0,
                x-w, y-((offset+2.0)*h), 0.0, 1.0, 1.0,
                x, y-((offset+2.0)*h), 0.0, 0.0, 1.0,
                x, y-(offset*h), 0.0, 0.0, 0.0
            ];

            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_left.png"));
            let obj = render::Object::create_square_with_points(points, INDCIES);
            textures.push(texture);
            objects.push(obj);
            
            offset += 2.0;
        }

        unsafe {
            for obj in objects.iter() {
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
        
        Self{x, y, w, h, pipe_len, objects, textures, program} 
    }

    pub unsafe fn draw(&self) {
        for i in 0..self.pipe_len*2+2 {
            gl::BindTexture(gl::TEXTURE_2D, self.textures[i].texture);
            gl::BindVertexArray(self.objects[i].vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

pub struct QuestionMarkBlock {
    x: f32,
    y: f32,
    h: f32,
    w: f32,
    obj: render::Object,
    texture: render::Texture,
    program: render::Program,
}

impl QuestionMarkBlock {
    pub fn create(x: f32, y: f32, h: f32, w: f32) -> Self {
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

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/question_mark_block.png"));

        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("assets/shaders/block.vert")).unwrap(),
        ).unwrap();

        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/block.frag")).unwrap(),
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

pub struct Objects {
    pub question_mark_blocks: Vec<QuestionMarkBlock>,
    pub blocks: Vec<game::Block>,
    pub stones: Vec<game::Block>,
    pub pipes: Vec<Pipe>,
    pub flag: Vec<Flag>,
    pub castle: Vec<game::Block>,
}

impl Objects {
    pub fn init() -> Self {
        let question_mark_blocks: Vec<QuestionMarkBlock> = vec![];
        let blocks: Vec<game::Block> = vec![];
        let stones: Vec<game::Block> = vec![];
        let pipes: Vec<Pipe> = vec![];
        let flag: Vec<Flag> = vec![];
        let castle: Vec<game::Block> = vec![];

        Self{question_mark_blocks, blocks, stones, pipes, flag, castle}
    }

    pub fn create_castle(&mut self, x: f32, y: f32, size: &str) {
        let mut block: game::Block;
        if size == "small" {
            block = game::Block::create(x, y, 80.0/208.0, 80.0/256.0, false, &Path::new("src/scenes/game/assets/images/castle_small.png"), "castle");
        }else {
            block = game::Block::create(x, y, 80.0/208.0, 80.0/256.0, false, &Path::new("src/scenes/game/assets/images/castle_large.png"), "castle");
        }

        self.castle.push(block);
    }

    pub fn create_flag(&mut self, x: f32, y: f32) {
        let block = Flag::create(x, y);

        self.flag.push(block);
    }

    pub fn create_pipe(&mut self, x: f32, y: f32, h: f32, w: f32, pipe_len: usize) {
        let block = Pipe::create(x, y, h, w, pipe_len);

        self.pipes.push(block);
    }

    pub fn create_stone(&mut self, x: f32, y: f32, h: f32, w: f32) {
        let block = game::Block::create(x, y, h, w, false, &Path::new("src/scenes/game/assets/images/stone_up.png"), "stone");

        self.stones.push(block);
    }

    pub fn create_block(&mut self, x: f32, y: f32, h: f32, w: f32, collision_event: bool) {
        let block = game::Block::create(x, y, h, w, collision_event, &Path::new("src/scenes/game/assets/images/brick.png"), "block");

        self.blocks.push(block);
    }

    pub fn create_question_mark_block(&mut self, x: f32, y: f32, h: f32, w: f32) {
        let block = QuestionMarkBlock::create(x, y, h, w);

        self.question_mark_blocks.push(block);
    }

    pub unsafe fn draw(&self) {
        for castle in self.castle.iter() {
            castle.draw();
        }

        for flag in self.flag.iter() {
            flag.draw();
        }

        for pipe in self.pipes.iter() {
            pipe.draw();
        }

        for block in self.blocks.iter() {
            block.draw();
        }

        for stone in self.stones.iter() {
            stone.draw();
        }

        for question_mark_block in self.question_mark_blocks.iter() {
            question_mark_block.draw();
        }
    }
}