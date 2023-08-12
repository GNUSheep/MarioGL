use crate::render;
use crate::scenes::game;
use std::path::Path;
use std::ffi::{CString, c_void};
use std::rc::Rc;
use std::cell::RefCell;

pub struct Collision_rect {
    pub x: f32,
    pub y: f32,
    pub w: f32,
    pub h: f32,
}

impl Collision_rect {
    fn get_intersection(&self, b: Collision_rect) -> (f32, f32, bool) {
        let x_distance = self.x - b.x;
        let y_distance = self.y - b.y;
        let min_distance_x = self.w + b.w;
        let min_distance_y = self.h + b.h;
        if f32::abs(x_distance) >= min_distance_x || f32::abs(y_distance) >= min_distance_x
        {
            return (0.0, 0.0, false);
        }
        let depth_x = if x_distance > 0.0 { min_distance_x - x_distance } else { -min_distance_x - x_distance };
        let depth_y = if y_distance > 0.0 { min_distance_y - y_distance } else { -min_distance_y - y_distance };
        return (depth_x, depth_y, true);
    }

    pub fn check_hitbox(&self, b: Collision_rect) -> Vec<char> {
        let intersection = self.get_intersection(b);
        if intersection.2 == false || (intersection.0 == 0.0 && intersection.1 == 0.0) {
            return vec!['0'];
        }
        let mut collisions: Vec<char> = vec![];
        if intersection.1.abs() - 0.01 <= intersection.0.abs() {
            if intersection.1 > 0.0 {collisions.push('b')}
            else if intersection.1 < 0.0 {collisions.push('t')}
        }else if intersection.1.abs() > intersection.0.abs() {
            if intersection.0 > 0.0 {collisions.push('l')}
            else if intersection.0 < 0.0 {collisions.push('r')}
        }
        return collisions;
    }
}

pub trait Collisioner {
    fn get_collision_rect(&self) -> Collision_rect;
    fn get_type(&self) -> &String;
    fn handle_collision(&mut self, collision_object: &String, collision_rect: Collision_rect, collisions: Vec<char>);
    fn get_collision(&mut self); 
    fn set_default_behavior(&mut self);
    fn run_default_behavior(&mut self, deltatime: u32);
}

pub trait Drawer {
    unsafe fn get_program(&self) -> &render::Program;
    unsafe fn set_uniforms(&self, view_x: f32, view_y: f32);
    unsafe fn draw(&self);
}


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
        let mut obj = game::Block::create(0.0, 0.0, 24.0/240.0, 16.0/256.0, false, "src/scenes/game/assets/images/troopa1.png", "troopa", "troopa");

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
        self.obj = game::Block::create(0.0, 0.0, 16.0/240.0, 16.0/256.0, false, "src/scenes/game/assets/images/troopa_squash.png", "goomba", "goomba");
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
        let mut obj = game::Block::create(0.0, 0.0, 16.0/240.0, 16.0/256.0, false, "src/scenes/game/assets/images/goomba1.png", "goomba", "goomba");

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
        self.obj = game::Block::create(0.0, 0.0, 8.0/240.0, 16.0/256.0, false, "src/scenes/game/assets/images/goomba_squash.png", "goomba", "nill");
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

pub struct Flag {
    x: f32,
    y: f32,
    stone: game::Block,
    pub objects: Vec<render::Object>,
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

        let stone = game::Block::create(x, y, 16.0/240.0, 16.0/256.0, false, "src/scenes/game/assets/images/stone_up.png", "flag", "flag");
        
        let h = 16.0/240.0;
        let w = 16.0/256.0;
        let mut offset = 1.0;
        for _i in 0..=8 {
            let points: Vec<f32> = vec![
                x+w, y+(offset*h), 0.0, 1.0, 0.0,
                x+w, y+((offset+2.0)*h), 0.0, 1.0, 1.0,
                x-w, y+((offset+2.0)*h), 0.0, 0.0, 1.0,
                x-w, y+(offset*h), 0.0, 0.0, 0.0
            ];

            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/flag_pipe.png"));
            let mut obj = render::Object::create_square_with_points(points, INDCIES);
            obj.set_cordinates(x, y+((offset+1.0)*h), 16.0/240.0, 16.0/256.0);
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
                obj.set_vertex_attrib_pointers();
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
    pub x: f32,
    pub y: f32,
    pub h: f32,
    pub w: f32,
    with_enter: bool,
    pub is_collision: bool,
    pipe_len: usize,
    pub objects: Vec<render::Object>,
    textures: Vec<render::Texture>,
    program: render::Program,
}

impl Pipe {
    pub fn create(x: f32, y: f32, h: f32, w: f32, pipe_len: usize, with_enter: bool, is_collision: bool) -> Self {
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

        if with_enter {
            let points: Vec<f32> = vec![
                x+w, y+h, 0.0, 1.0, 0.0,
                x+w, y-h, 0.0, 1.0, 1.0,
                x, y-h, 0.0, 0.0, 1.0,
                x, y+h, 0.0, 0.0, 0.0
            ];
    
            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_enter_right.png"));
            let mut obj = render::Object::create_square_with_points(points, INDCIES);
            obj.set_cordinates(x+w/2 as f32, y, 16.0/240.0, 16.0/256.0);
            textures.push(texture);
            objects.push(obj);
    
            let points: Vec<f32> = vec![
                x-w, y+h, 0.0, 1.0, 0.0,
                x-w, y-h, 0.0, 1.0, 1.0,
                x, y-h, 0.0, 0.0, 1.0,
                x, y+h, 0.0, 0.0, 0.0
            ];
    
            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_enter_left.png"));
            let mut obj = render::Object::create_square_with_points(points, INDCIES);
            obj.set_cordinates(x-w/2 as f32, y, 16.0/240.0, 16.0/256.0);
            textures.push(texture);
            objects.push(obj);
        }

        let mut offset = 1.0;
        for _i in 1..=pipe_len {    
            let points: Vec<f32> = vec![
                x+w, y-(offset*h), 0.0, 1.0, 0.0,
                x+w, y-((offset+2.0)*h), 0.0, 1.0, 1.0,
                x, y-((offset+2.0)*h), 0.0, 0.0, 1.0,
                x, y-(offset*h), 0.0, 0.0, 0.0
            ];

            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_right.png"));
            let mut obj = render::Object::create_square_with_points(points, INDCIES);
            obj.set_cordinates(x+w/2 as f32, y-(offset+1.0)*h, 16.0/240.0, 16.0/256.0);
            textures.push(texture);
            objects.push(obj);

            
            let points: Vec<f32> = vec![
                x-w, y-(offset*h), 0.0, 1.0, 0.0,
                x-w, y-((offset+2.0)*h), 0.0, 1.0, 1.0,
                x, y-((offset+2.0)*h), 0.0, 0.0, 1.0,
                x, y-(offset*h), 0.0, 0.0, 0.0
            ];

            let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_left.png"));
            let mut obj = render::Object::create_square_with_points(points, INDCIES);
            obj.set_cordinates(x-w/2 as f32, y-(offset+1.0)*h, 16.0/240.0, 16.0/256.0);
            textures.push(texture);
            objects.push(obj);
            
            offset += 2.0;
        }

        unsafe {
            for obj in objects.iter() {
                obj.set_vertex_attrib_pointers();
            }
        }
        
        Self{x, y, w, h, with_enter, is_collision, pipe_len, objects, textures, program} 
    }

    pub fn create_sidepipe(x: f32, y: f32, h: f32, w: f32, mut pipe_len: usize) -> Self {
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
            x+w, y+h+h, 0.0, 1.0, 0.0,
            x+w, y, 0.0, 1.0, 1.0,
            x-w, y, 0.0, 0.0, 1.0,
            x-w, y+h+h, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_side_enter_top.png"));
        let mut obj = render::Object::create_square_with_points(points, INDCIES);
        obj.set_cordinates(x, y+h, 16.0/240.0, 16.0/256.0);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x+w, y, 0.0, 1.0, 0.0,
            x+w, y-h-h, 0.0, 1.0, 1.0,
            x-w, y-h-h, 0.0, 0.0, 1.0,
            x-w, y, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_side_enter_bottom.png"));
        let mut obj = render::Object::create_square_with_points(points, INDCIES);
        obj.set_cordinates(x, y-h, 16.0/240.0, 16.0/256.0);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x+w*3 as f32, y+h+h, 0.0, 1.0, 0.0,
            x+w*3 as f32, y, 0.0, 1.0, 1.0,
            x+w, y, 0.0, 0.0, 1.0,
            x+w, y+h+h, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_side_top.png"));
        let mut obj = render::Object::create_square_with_points(points, INDCIES);
        obj.set_cordinates(x+w*2 as f32, y+h, 16.0/240.0, 16.0/256.0);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x+w*3 as f32, y, 0.0, 1.0, 0.0,
            x+w*3 as f32, y-h-h, 0.0, 1.0, 1.0,
            x+w, y-h-h, 0.0, 0.0, 1.0,
            x+w, y, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_side_bottom.png"));
        let mut obj = render::Object::create_square_with_points(points, INDCIES);
        obj.set_cordinates(x+w*2 as f32, y-h, 16.0/240.0, 16.0/256.0);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x+w*5 as f32, y+h+h, 0.0, 1.0, 0.0,
            x+w*5 as f32, y, 0.0, 1.0, 1.0,
            x+w*3 as f32, y, 0.0, 0.0, 1.0,
            x+w*3 as f32, y+h+h, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_side_connection_top.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x+w*5 as f32, y, 0.0, 1.0, 0.0,
            x+w*5 as f32, y-h-h, 0.0, 1.0, 1.0,
            x+w*3 as f32, y-h-h, 0.0, 0.0, 1.0,
            x+w*3 as f32, y, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_side_connection_bottom.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x+w*7 as f32, y+h+h, 0.0, 1.0, 0.0,
            x+w*7 as f32, y, 0.0, 1.0, 1.0,
            x+w*5 as f32, y, 0.0, 0.0, 1.0,
            x+w*5 as f32, y+h+h, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_right.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        let points: Vec<f32> = vec![
            x+w*7 as f32, y, 0.0, 1.0, 0.0,
            x+w*7 as f32, y-h-h, 0.0, 1.0, 1.0,
            x+w*5 as f32, y-h-h, 0.0, 0.0, 1.0,
            x+w*5 as f32, y, 0.0, 0.0, 0.0
        ];

        let texture = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/pipe_right.png"));
        let obj = render::Object::create_square_with_points(points, INDCIES);
        textures.push(texture);
        objects.push(obj);

        let pipe = Pipe::create(x+((16.0/256.0)*5 as f32), -1.0-((16.0/240.0)*(13-pipe_len) as f32), 16.0/240.0, 32.0/256.0, pipe_len, false, false);

        textures.extend(pipe.textures);
        objects.extend(pipe.objects);

        unsafe {
            for obj in objects.iter() {
                obj.set_vertex_attrib_pointers();
            }
        }

        pipe_len += 3;
        let with_enter = true;
        let is_collision = true;

        Self{x, y, w, h, with_enter, is_collision, pipe_len, objects, textures, program} 
    }

    pub unsafe fn draw(&self) {
        let mut add = 0;
        if self.with_enter {
            add = 2;
        }
        for i in 0..self.pipe_len*2+add {
            gl::BindTexture(gl::TEXTURE_2D, self.textures[i].texture);
            gl::BindVertexArray(self.objects[i].vao);
            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }
    }
}

pub struct QuestionMarkBlock {
    pub x: f32,
    pub y: f32,
    pub h: f32,
    pub w: f32,
    pub object_type: String,
    pub move_block: bool,
    pub collision_event: bool,
    pub collision_name: String,
    pub state: usize,
    pub delay: i32,
    pub is_hit: bool,
    pub obj: render::Object,
    textures: Vec<render::Texture>,
    program: render::Program,
}

impl QuestionMarkBlock {
    pub fn create(
        x: f32,
        y: f32, 
        h: f32, 
        w: f32, 
        collision_event: bool, 
        collision_name: String
    ) -> Self {
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

        let mut obj = render::Object::create_square_with_points(points, INDCIES);
        obj.set_cordinates(x, y, h, w);

        let mut textures: Vec<render::Texture> = vec![];
    
        let texture1 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/question_mark_block1.png"));
        textures.push(texture1);
        let texture2 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/question_mark_block2.png"));
        textures.push(texture2);
        let texture3 = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/question_mark_block3.png"));
        textures.push(texture3);

        let vert_shader = render::Shader::vertex_from_src(
            &CString::new(include_str!("assets/shaders/block.vert")).unwrap(),
        ).unwrap();

        let frag_shader = render::Shader::fragment_from_src(
            &CString::new(include_str!("assets/shaders/block.frag")).unwrap(),
        ).unwrap();

        let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

        unsafe {
            obj.set_vertex_attrib_pointers();
        }

        let state = 0;
        let delay = 0;
        let is_hit = false;
        let move_block = false;

        Self{x, y, w, h, object_type: "?block".to_string(), move_block, collision_event, collision_name, state, delay, is_hit, obj, textures, program} 
    }

    pub fn attach_to_main_loop(
        self,
        mut collisions_objects: &mut Vec<Rc<RefCell<dyn Collisioner>>>,
        mut objects_draw: &mut Vec<Rc<RefCell<dyn Drawer>>>,
    ){
        let question_mark_block_rc: Rc<RefCell<QuestionMarkBlock>> = Rc::new(RefCell::new(self));

        let question_mark_block_drawer: Rc<RefCell<dyn Drawer>> = question_mark_block_rc.clone();
        objects_draw.push(question_mark_block_drawer);

        let question_mark_block_collisioner: Rc<RefCell<dyn Collisioner>> = question_mark_block_rc.clone();
        collisions_objects.push(question_mark_block_collisioner);
    }

    pub fn handler(&mut self, objects: &mut Vec<game::Block>) {
        if self.collision_event {
            if self.collision_name == "mushroom" {
                let mut obj = game::Block::create(0.0, 0.0, self.h, self.w, false, "src/scenes/game/assets/images/mushroom.png", "mushroom", "mushroom");
    
                let vert_shader = render::Shader::vertex_from_src(
                    &CString::new(include_str!("assets/shaders/mushroom.vert")).unwrap(),
                ).unwrap();
        
                let frag_shader = render::Shader::fragment_from_src(
                    &CString::new(include_str!("assets/shaders/mushroom.frag")).unwrap(),
                ).unwrap();
        
                let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();

                obj.x = self.x;
                obj.y = self.y+2.0*self.h;
    
                obj.program = program;
    
                obj.move_acc_x = 1.0;
                objects.push(obj);
            }else {
                let mut block = game::Block::create(self.x, self.y+2.0*self.h, self.h, 8.0/256.0, true, "src/scenes/game/assets/images/coin1.png", "coin", "coin");

                block.collision_name = "coin".to_string();
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin2.png")));
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin3.png")));
                block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin4.png")));

                let vert_shader = render::Shader::vertex_from_src(
                    &CString::new(include_str!("assets/shaders/coin.vert")).unwrap(),
                ).unwrap();
        
                let frag_shader = render::Shader::fragment_from_src(
                    &CString::new(include_str!("assets/shaders/coin.frag")).unwrap(),
                ).unwrap();
        
                let program = render::Program::create_with_shaders(&[vert_shader, frag_shader]).unwrap();
    
                block.program = program;

                block.x = 0.0;
                block.y = 0.0;

                block.move_acc_y = 2.5;
    
                objects.push(block);
            }
        }
        self.textures[0] = render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/brick-still.png"));
        self.state = 0;
        self.collision_event = false;
        self.is_hit = true;
    } 

}

impl Collisioner for QuestionMarkBlock {
    fn get_collision_rect(&self) -> Collision_rect {
        Collision_rect{
            x: self.x, 
            y: self.y, 
            w: self.w, 
            h: self.h,
        }
    }
    fn get_type(&self) -> &String {
        &self.object_type
    }
    fn handle_collision(&mut self, collision_object: &String, collision_rect: Collision_rect, collisions: Vec<char>){}
    fn get_collision(&mut self){}
    fn set_default_behavior(&mut self){}
    fn run_default_behavior(&mut self, deltatime: u32){}
}

impl Drawer for QuestionMarkBlock {
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

pub struct Objects {
    pub pipes: Vec<Pipe>,
    pub flag: Vec<Flag>,
    pub castle: Vec<game::Block>,
    pub coins: Vec<game::Block>,
}

impl Objects {
    pub fn init() -> Self {
        let pipes: Vec<Pipe> = vec![];
        let flag: Vec<Flag> = vec![];
        let castle: Vec<game::Block> = vec![];
        let coins: Vec<game::Block> = vec![];

        Self{pipes, flag, castle, coins}
    }

    pub fn create_castle(&mut self, x: f32, y: f32, size: &str) {
        let block: game::Block;
        if size == "small" {
            block = game::Block::create(x, y, 80.0/240.0, 80.0/256.0, false, "src/scenes/game/assets/images/castle_small.png", "castle", "nill");
        }else {
            block = game::Block::create(x, y, 80.0/240.0, 80.0/256.0, false, "src/scenes/game/assets/images/castle_large.png", "castle", "nill");
        }

        self.castle.push(block);
    }

    pub fn create_coin(&mut self, x: f32, y: f32) {
        let mut block = game::Block::create(x, y, 16.0/240.0, 16.0/256.0, true, "src/scenes/game/assets/images/coin_still1.png", "coin", "coin");

        block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin_still2.png")));
        block.textures.push(render::Texture::create_new_texture_from_file(&Path::new("src/scenes/game/assets/images/coin_still3.png")));

        self.coins.push(block);
    }
    
    pub fn create_flag(&mut self, x: f32, y: f32) {
        let block = Flag::create(x, y);

        self.flag.push(block);
    }

    pub fn create_pipe(&mut self, x: f32, y: f32, h: f32, w: f32, pipe_len: usize, with_enter: bool, is_collision: bool) {
        let block = Pipe::create(x, y, h, w, pipe_len, with_enter, is_collision);

        self.pipes.push(block);
    }

    pub unsafe fn draw(&self) {
        for coin in self.coins.iter() {
            coin.draw();
        }

        for castle in self.castle.iter() {
            castle.draw();
        }

        for flag in self.flag.iter() {
            flag.draw();
        }

        for pipe in self.pipes.iter() {
            pipe.draw();
        }
    }
}