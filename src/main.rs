extern crate sdl2;

mod scenes;
mod render;

use sdl2::video::GLProfile;
use sdl2::keyboard::Keycode;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let timer_subsystem = sdl_context.timer().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    //let timer = timer_subsystem.add_timer();

    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(GLProfile::Core);
    gl_attr.set_context_version(4, 6);

    let window = video_subsystem.window("Runner", 1280, 720).opengl().build().unwrap();
    
    let _ctx = window.gl_create_context().unwrap();
    gl::load_with(|name| video_subsystem.gl_get_proc_address(name) as *const _);

    let mut event_pump = sdl_context.event_pump().unwrap();
    let mut game = scenes::game::Game::init();
    let mut up = false;
    let mut down = false;
    let mut right = false;
    let mut left = false;

    let mut sec = 0;
    'main: loop {
        let last_ticks = timer_subsystem.ticks();
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                sdl2::event::Event::KeyDown {keycode: Some(keycode), ..} => { 
                    match keycode {
                        Keycode::D => right = true,
                        Keycode::A => left = true,
                        Keycode::W => up = true,
                        Keycode::S => down = true,
                        _ => {},
                    }
                },
                sdl2::event::Event::KeyUp {keycode: Some(keycode), ..} => { 
                    match keycode {
                        Keycode::D => right = false,
                        Keycode::A => left = false,
                        Keycode::W => up = false,
                        Keycode::S => down = false,
                        _ => {},
                    }
                },
                _ => {},
            }
        }
        //if !game.spirit.is_dead {
            if left {game.move_x("left")};
            if right {game.move_x("right")};
            if up {game.jump()};
            if down {game.crouch()};
        //}

        unsafe{game.draw()};
        window.gl_swap_window();

        let deltatime = timer_subsystem.ticks()-last_ticks;

        sec += deltatime;
        if sec > 1000 {
            sec = 0;
            game.time -= 1;
        }

        if game.is_endlvl {
            game.endLevel(deltatime);
        }else {
            game.handle(deltatime);
        }
    }
}