use std::ffi::c_void;
use std::fs::File;
use std::io::prelude::*;
use std::ffi::{CString, CStr};

pub struct Shader {
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
            let shader_program: u32 = gl::CreateProgram();
            gl::AttachShader(shader_program, vertex_shader);
            gl::AttachShader(shader_program, fragement_shader);
            gl::LinkProgram(shader_program);
            Shader { id: shader_program }
        }
    }

    pub unsafe fn use_program(&self) {
        gl::UseProgram(self.id);
    }

    pub unsafe fn get_uniform(&self, name: &str) -> i32 {
        self.use_program();
        gl::GetUniformLocation(self.id,
            CString::new(name).unwrap().as_ptr())
    }
}

pub struct Model<'a> {
    pub shader: &'a Shader,
    pub vertices: &'a [f32],
    pub indices: &'a [u32],
    pub transform: glm::TMat4<f32>,
    transform_id: i32,
    vao: u32,
}

impl<'a> Model<'_> {
    pub fn new(vertices: &'a [f32], indices: &'a [u32], shader: &'a Shader) -> Model<'a> {
        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut ibo: u32 = 0;

        let transform_id = unsafe { shader.get_uniform("transform") };

        let mut transform: glm::TMat4<f32> = glm::TMat4::identity();

        unsafe {
            const FLOAT_SIZE: isize = std::mem::size_of::<f32>() as isize;
            const UNSIGNED_SIZE: isize = std::mem::size_of::<u32>() as isize;

            gl::GenVertexArrays(1, &mut vao);
            gl::GenBuffers(1, &mut vbo);
            gl::GenBuffers(1, &mut ibo);

            gl::BindVertexArray(vao);

            gl::BindBuffer(gl::ARRAY_BUFFER, vbo);
            gl::BufferData(gl::ARRAY_BUFFER, vertices.len() as isize * FLOAT_SIZE,
                           vertices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, indices.len() as isize * UNSIGNED_SIZE,
                           indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 6 * FLOAT_SIZE as i32, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
                                    6 * FLOAT_SIZE as i32, (3 * FLOAT_SIZE) as *const c_void);

            gl::EnableVertexAttribArray(1);

            shader.use_program();

            gl::UniformMatrix4fv(transform_id, 1, gl::FALSE, glm::value_ptr(&transform).as_ptr());
        };
        Model {
            shader,
            vertices,
            indices,
            transform,
            transform_id,
            vao,
        }
    }

    unsafe fn push_transformation(&self) {
        gl::UniformMatrix4fv(self.transform_id, 1, gl::FALSE, glm::value_ptr(&self.transform).as_ptr());
    }

    pub unsafe fn draw(&self) {
        self.shader.use_program();
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32,
                         gl::UNSIGNED_INT, *self.indices.as_ptr() as *const c_void)
    }

    pub unsafe fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform = glm::translate(&self.transform, &glm::vec3(x, y, z));
        println!("{}", self.transform_id);
        println!("{:?}", self.transform);
        self.push_transformation();
    }

    pub unsafe fn rotate(&mut self, axis: glm::Vec3, degrees: f32) {
        self.transform = glm::rotate(&self.transform, glm::radians(&glm::vec1(degrees))[0], &axis);
        self.push_transformation();
    }
}
