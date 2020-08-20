mod gla;

extern crate glfw;
extern crate gl;
extern crate nalgebra_glm as glm;

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

    let mut camera = gla::Camera::new(70.0, 1.0, 0.1, 100.0);
    let material = gla::Material::new(
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/teapot_v.glsl"),
        concat!(env!("CARGO_MANIFEST_DIR"), "/assets/teapot_f.glsl"),
        glm::vec3(0.1, 0.1, 0.1),
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(0.8, 0.8, 0.8),
        32,
    );

    let mut teapot = gla::Model::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/teapot.obj"), &material);
    let mut teapot2 = gla::Model::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/teapot.obj"), &material);
    let mut teapot3 = gla::Model::new(concat!(env!("CARGO_MANIFEST_DIR"), "/assets/teapot.obj"), &material);

    // let light = gla::point_light(
    //     glm::vec3(1.0, 1.0, 1.0),
    //     glm::vec3(5.0, 5.0, 5.0),
    //     0.007,
    //     0.0002,
    // );

    let light = gla::directional_light(
        glm::vec3(1.0, 1.0, 1.0),
        glm::vec3(0.5, -1.0, 0.0),
    );

    // let light = gla::spot_light(
    //     glm::vec3(0., 0., -5.),
    //     glm::vec3(1.0, 1.0, 1.0),
    //     glm::vec3(0.0, 0.0, 0.0),
    //     10.,
    //     0.007,
    //     0.0002,
    // );

    camera.translate(0., 0., -7.);
    camera.rotate(glm::vec3(1.0, 0.0, 0.0), 15.0);

    teapot.translate(-2., -1., -0.0);
    teapot2.translate(2., 1., -1.);
    teapot3.translate(2., -3., -1.);

    println!("ready!");
    while !window.should_close() {
        unsafe {
            gl::ClearColor(0.45, 0.55, 0.55, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT | gl::DEPTH_BUFFER_BIT);
            gl::Viewport(0, 0, 800, 800);

            material.push_uniforms(&camera, &light);

            teapot.rotate(glm::vec3(0.0, 1.0, 0.0), (glfw_instance.get_time() * 20.0) as f32);
            teapot.draw();

            teapot2.rotate(glm::vec3(0.0, 1.0, 0.0), (glfw_instance.get_time() * -15.0) as f32);
            teapot2.draw();

            teapot3.rotate(glm::vec3(0.0, 1.0, 0.0), (glfw_instance.get_time() * 25.0) as f32);
            teapot3.draw();

            glfw_instance.set_time(0.0);
        }

        window.swap_buffers();
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
    }
}