extern crate gl;
extern crate sdl2;

use sdl2::{video::Window, EventPump, Sdl, VideoSubsystem};
use std::ffi::{CString, CStr};

fn main() 
{
    let sdl: Sdl = sdl2::init().unwrap();
    let video_subsystem: VideoSubsystem = sdl.video().unwrap();
    let gl_attr = video_subsystem.gl_attr();
    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);
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
        gl::Viewport(0, 0, 900, 700); // set viewport
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

struct Shader 
{
    id: gl::types::GLuint,
}

impl Shader 
{
    fn from_source(
        source: &CStr,
        kind: gl::types::GLenum
        ) -> Result<Shader, String> 
    {
        let id = shader_from_source(source, kind)?;
        Ok(Shader { id })
    }

    fn from_vert_source(source: &CStr) -> Result<Shader, String> 
    {
        Shader::from_source(source, gl::VERTEX_SHADER)
    }

    fn from_frag_source(source: &CStr) -> Result<Shader, String> 
    {
        Shader::from_source(source, gl::FRAGMENT_SHADER)
    }

    // continue here
}

impl Drop for Shader 
{
    fn drop(&mut self) 
    {
        unsafe 
        {
            gl::DeleteShader(self.id);
        }
    }
}

fn shader_from_source(
    source: &CStr, // modified
    kind: gl::types::GLuint
    ) -> Result<gl::types::GLuint, String> 
{
    let id = unsafe { gl::CreateShader(kind) };
    unsafe 
    {
        gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
        gl::CompileShader(id);
    }
    let mut success: gl::types::GLint = 1;
    unsafe 
    {
        gl::GetShaderiv(id, gl::COMPILE_STATUS, &mut success);
    }
    if success == 0 
    {
        let mut len: gl::types::GLint = 0;
        unsafe 
        {
            gl::GetShaderiv(id, gl::INFO_LOG_LENGTH, &mut len);
        }
        let error = create_whitespace_cstring_with_len(len as usize);
        unsafe 
        {
            gl::GetShaderInfoLog(
                id,
                len,
                std::ptr::null_mut(),
                error.as_ptr() as *mut gl::types::GLchar
            );
        }
        return Err(error.to_string_lossy().into_owned());

        // continue here
    }
    
    Ok(id)
    
    // continue here
}

fn create_whitespace_cstring_with_len(len: usize) -> CString 
{
    // allocate buffer of correct size
    let mut buffer: Vec<u8> = Vec::with_capacity(len + 1);
    // fill it with len spaces
    buffer.extend([b' '].iter().cycle().take(len));
    // convert buffer to CString
    unsafe { CString::from_vec_unchecked(buffer) }
}