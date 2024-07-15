extern crate gl;
extern crate sdl2;

use sdl2::{video::Window, EventPump, Sdl, VideoSubsystem};

fn main() 
{
    let sdl: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl.video().unwrap();
    let window: Window = video_subsystem
        .window("Game", 900, 700)
        .opengl()
        .resizable()
        .build()
        .unwrap();

    let gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);
    let gl_context = window.gl_create_context().unwrap();

    unsafe 
    {
        gl::ClearColor(0.3, 0.3, 0.5, 1.0);
    }
    let mut event_pump: EventPump = sdl.event_pump().unwrap();
    'main: loop 
    {
        for event in event_pump.poll_iter() 
        {
            match event 
            {
                sdl2::event::Event::Quit { .. } => break 'main,
                _ => {}
            }
        }

        // render window contents here

        unsafe 
        {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }
        window.gl_swap_window();
    }
}