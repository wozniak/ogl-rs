mod gla;

extern crate sdl2;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::ffi::c_void;

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

    let vertices: [f32; 18] = [
        // positions     // colors
        0.5, -0.5, 0.0,  1.0, 0.0, 0.0,   // bottom right
       -0.5, -0.5, 0.0,  0.0, 1.0, 0.0,   // bottom left
        0.0,  0.5, 0.0,  0.0, 0.0, 1.0,   // top
    ];

    let indices: [u32; 3] = [
        0, 1, 2
    ];

    let shader = gla::Shader::new("vertex.glsl", "fragment.glsl");
    let mut triangle = gla::Model::new(&vertices, &indices, &shader);

    println!("ready!");

    unsafe { triangle.translate(0.0, 1.0, 0.0) };

    'main: loop {
        for event in event_pump.poll_iter() {
            match event {
                sdl2::event::Event::Quit {..} => break 'main,
                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(0.4, 0.7, 0.7, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 800, 800);

            triangle.draw();
        }

        window.gl_swap_window();
    }
}