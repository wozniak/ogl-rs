extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::ffi::c_void;
use std::fs::File;
use std::io::prelude::*;
use std::ffi::{CString, CStr};

struct Shader {
    id: u32,
}

impl Shader {
    pub fn new(vert: &str, frag: &str) -> Shader {
        fn shader_from_source(
            source: &CStr,
            kind: gl::types::GLenum) -> gl::types::GLuint {
            let id = unsafe { gl::CreateShader(kind) };
        
            unsafe {
                gl::ShaderSource(id, 1, &source.as_ptr(), std::ptr::null());
                gl::CompileShader(id);
                id
            }
        }

        let mut file = File::open(vert).unwrap();
        let mut vertex_source = String::new();
        file.read_to_string(&mut vertex_source).unwrap();
        let vertex_source: &CStr = &CString::new(str::replace(&vertex_source, "\r", "")).unwrap();

        let mut file = File::open(frag).unwrap();
        let mut fragment_shader = String::new();
        file.read_to_string(&mut fragment_shader).unwrap();
        let fragment_source: &CStr = &CString::new(str::replace(&fragment_shader, "\r", "")).unwrap();

        let vertex_shader    = shader_from_source(vertex_source, gl::VERTEX_SHADER);
        let fragement_shader = shader_from_source(fragment_source, gl::FRAGMENT_SHADER);
    
        unsafe {
            let shaderprogram: u32 = gl::CreateProgram();
            gl::AttachShader(shaderprogram, vertex_shader);
            gl::AttachShader(shaderprogram, fragement_shader);
            gl::LinkProgram(shaderprogram);
            Shader { id: shaderprogram }
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_uniform(&self, name: &str) -> i32 {
        gl::GetUniformLocation(self.id,
            CString::new(name).unwrap().as_ptr())
    }
}

struct Model {
    shader: Shader,
    vertecies: Vec<f32>,
    indicies: Vec<u32>,
}

fn main() {
    let sdl = sdl2::init().unwrap();
    let video_subsystem = sdl.video().unwrap();

    let gl_attr = video_subsystem.gl_attr();

    gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
    gl_attr.set_context_version(4, 5);

    let window = video_subsystem
        .window("Game", 800, 800)
        .opengl()
        .build()
        .unwrap();
    
    let _gl_context = window.gl_create_context().unwrap();
    let _gl = gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as *const std::os::raw::c_void);

    let mut event_pump = sdl.event_pump().unwrap();

    let vertecies: [f32; 18] = [
        // positions     // colors
        0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
       -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,  0.0, 0.0, 1.0,   // top
    ];

    let indicies: [u32; 3] = [
        0, 1, 2
    ];

    let shader = Shader::new("vertex.glsl", "fragment.glsl");

    let mut vao: u32 = 0;
    let mut vbo: u32 = 0;
    let mut ibo: u32 = 0;

    let trans_unif = unsafe { shader.get_uniform("transform") };
    let mut vec = glm::vec3(1.0, 1.0, 0.0);

    let mut trans = glm::mat4(1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                              1.0, 1.0, 1.0, 1.0, 1.0, 1.0, 1.0,
                              1.0, 1.0, 1.0);

    trans = glm::translate(&trans, &vec);

    unsafe {
        const FLOAT_SIZE: usize = std::mem::size_of::<f32>();

        gl::GenVertexArrays(1, &mut vao);
        gl::GenBuffers(1, &mut vbo);
        gl::GenBuffers(1, &mut ibo);
    
        gl::BindVertexArray(vao);
    
        gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
        gl::BufferData(gl::ARRAY_BUFFER, 18 * FLOAT_SIZE as isize,
            &vertecies as *const f32 as *const c_void, gl::STATIC_DRAW);
        
        gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
        gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, 3 * std::mem::size_of::<u32>() as isize,
            &indicies as *const u32 as *const c_void, gl::STATIC_DRAW);
    
        gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * FLOAT_SIZE as i32, 0 as *const c_void);
        gl::EnableVertexAttribArray(0);
    
        gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
            6 * FLOAT_SIZE as i32, (3 * FLOAT_SIZE) as *const c_void);

        gl::EnableVertexAttribArray(1);

        shader.use_program();
        gl::UniformMatrix4fv(trans_unif, 1, gl::FALSE,
                             glm::value_ptr(&trans) as *const _ as *const f32);
    };

    println!("ready!");

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(0.4, 0.7, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);
            gl::Viewport(0, 0, 800, 800);
    
            shader.use_program();
            gl::BindVertexArray(vao);

            //elements.draw();

            gl::DrawElements(gl::TRIANGLES, 6, gl::UNSIGNED_INT, std::ptr::null());
        }

        window.gl_swap_window();
    }
}