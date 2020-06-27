mod gla;

extern crate glfw;
extern crate gl;
extern crate nalgebra_glm as glm;

use std::ffi::c_void;
use glfw::{Action, Context, Key};

fn main() {
    glfw::WindowHint::DoubleBuffer(false);
    let mut glfw_instance = glfw::init(glfw::FAIL_ON_ERRORS).unwrap();

    // Create a windowed mode window and its OpenGL context
    let (mut window, events) = glfw_instance.create_window(800, 800, "test", glfw::WindowMode::Windowed)
        .expect("Failed to create GLFW window.");

    // Make the window's context current
    window.make_current();
    window.set_key_polling(true);

    gl::load_with(|s| window.get_proc_address(s));

     unsafe { gl::Enable(gl::DEPTH_TEST) };

    let shader = gla::Shader::new(concat!(env!("CARGO_MANIFEST_DIR"), "/res/teapot_v.glsl"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/res/teapot_f.glsl"));

    let mut camera = gla::Camera::new(70.0, 1.0, 0.1, 100.0);
    let mut triangle = gla::Model::new("res/teapot.obj", &shader);
    let light = gla::Light::new(
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(2.0, 3.0, 5.0)
    );

    camera.translate(0.0, -2.0, -8.0);
    // camera.rotate(glm::vec3(0.0, 1.0, 0.0), 180.0);

    println!("ready!");

    while !window.should_close() {
        glfw_instance.poll_events();
        for (_, event) in glfw::flush_messages(&events) {
            println!("{:?}", event);
            match event {
                glfw::WindowEvent::Key(Key::Escape, _, Action::Press, _) => {
                    window.set_should_close(true)
                },
                _ => {},
            }
        }

        unsafe {
            gl::ClearColor(0.7, 0.4, 0.6, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 800, 800);

            camera.update(shader.id);
            light.push_uniforms(&shader);

            triangle.rotate(glm::vec3(0.0, 1.0, 0.0), (glfw_instance.get_time() * 50.0) as f32);
            triangle.draw();

            glfw_instance.set_time(0.0);
        }

        window.swap_buffers();
    }
}