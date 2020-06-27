#![allow(dead_code)]
extern crate tobj;

use std::ffi::c_void;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::ffi::{CString, CStr};

pub struct Shader {
    pub id: u32,
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

    pub unsafe fn uniform_mat4(&self, name: &str, matrix: &glm::TMat4<f32>) {
        self.use_program();
        gl::UniformMatrix4fv(self.get_uniform(name), 1, gl::FALSE, glm::value_ptr(matrix).as_ptr());
    }

    pub unsafe fn uniform_vec3(&self, name: &str, x: f32, y: f32, z: f32) {
        self.use_program();
        gl::Uniform3f(self.get_uniform(name), x, y, z);
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
    shader: &'a Shader,
    pub transform: glm::TMat4<f32>,
    indices: Vec<u32>,
    transform_id: i32,
    vao: u32,
    pub vbo: u32,
}

impl<'a> Model<'a> {
    pub fn new(obj_file: &'a str, shader: &'a Shader) -> Model<'a> {
        let (mut models, _) = tobj::load_obj(&obj_file, true).unwrap();
        let mut mesh = models.swap_remove(0).mesh;

        let mut full_verts = Vec::new();
        let vert_count = mesh.positions.len();

        full_verts.append(&mut mesh.positions);
        full_verts.append(&mut mesh.normals);

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
            gl::BufferData(gl::ARRAY_BUFFER, full_verts.len() as isize * FLOAT_SIZE,
                           full_verts.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::BindBuffer(gl::ELEMENT_ARRAY_BUFFER, ibo);
            gl::BufferData(gl::ELEMENT_ARRAY_BUFFER, mesh.indices.len() as isize * UNSIGNED_SIZE,
                           mesh.indices.as_ptr() as *const c_void, gl::STATIC_DRAW);

            gl::VertexAttribPointer(0, 3, gl::FLOAT, gl::FALSE, 3 * FLOAT_SIZE as i32, 0 as *const c_void);
            gl::EnableVertexAttribArray(0);

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE,
                                    3 * FLOAT_SIZE as i32, (vert_count as isize * FLOAT_SIZE) as *const c_void);

            gl::EnableVertexAttribArray(1);

            shader.use_program();
        };
        Model {
            shader,
            transform,
            transform_id,
            indices: mesh.indices,
            vao,
            vbo,
        }
    }

    pub unsafe fn draw(&self) {
        self.shader.use_program();
        self.shader.uniform_mat4("transform", &self.transform);
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32,
                         gl::UNSIGNED_INT, *self.indices.as_ptr() as *const c_void)
    }

    pub unsafe fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform = glm::translate(
            &self.transform,
            &glm::vec3(x, y, z));
    }

    pub unsafe fn rotate(&mut self, axis: glm::Vec3, degrees: f32) {
        self.transform = glm::rotate(
            &self.transform,
            glm::radians(&glm::vec1(degrees))[0],
            &axis
        );
    }
}

pub struct Camera {
    fov: f32,
    view: glm::TMat4<f32>,
    projection: glm::TMat4<f32>,
}

impl Camera {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Camera {
        let projection = glm::perspective(aspect_ratio,
            glm::radians(&glm::vec1(fov))[0] as f32, near, far);

        let view: glm::TMat4<f32> = glm::TMat4::identity();

        Camera {
            fov,
            view,
            projection,
        }
    }

    pub unsafe fn update(&self, shader_id: u32) {
        let s = Shader { id: shader_id };
        s.use_program();
        s.uniform_mat4("view", &self.view);
        s.uniform_mat4("projection", &self.projection);
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.view = glm::translate(&self.view, &glm::vec3(x, y, z));
    }

}

pub struct Light {
    color: glm::Vec3,
    position: glm::Vec3,
}

impl Light {
    pub fn new(color: glm::Vec3, position: glm::Vec3) -> Light { Light { color, position } }

    pub unsafe fn push_uniforms(&self, shader: &Shader) {
        shader.uniform_vec3("lightColor", self.color.x, self.color.y, self.color.z);
        shader.uniform_vec3("objectColor", 1.0, 1.0, 1.0);
        shader.uniform_vec3("lightPosition", self.position.x, self.position.y, self.position.z);
    }
}
