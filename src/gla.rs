#![allow(dead_code)]
extern crate tobj;
extern crate nalgebra_glm;

use std::ffi::c_void;
use std::fs::File;
use std::io::prelude::*;
use std::ffi::{CString, CStr};

pub struct Material {
    shader_id: u32,
    ambient: glm::Vec3,
    diffuse: glm::Vec3,
    specular: glm::Vec3,
    gloss: u32,
}

impl Material {
    pub fn new(vert: &str, frag: &str, ambient: glm::Vec3, diffuse: glm::Vec3, specular: glm::Vec3, gloss: u32) -> Material {
        fn shader_from_source(
            source: &CStr,
            kind: gl::types::GLenum) -> u32 {
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
            Material {
                shader_id: shader_program,
                ambient,
                diffuse,
                specular,
                gloss,
            }
        }
    }

    pub fn push_uniforms(&self, camera: &Camera, light: &Light) {
        unsafe {
            self.uniform_vec3("material.diffuse", &self.diffuse);
            self.uniform_vec3("material.specular", &self.specular);
            self.uniform_vec3("material.ambient", &self.ambient);
            self.uniform_uint("material.gloss", self.gloss);

            match light {
                Light::Point(point) => {
                    self.uniform_uint("light.type", 0);
                    self.uniform_vec3("light.color", &point.color);
                    self.uniform_vec4("light.vector", &point.position);
                    self.uniform_float("light.range", point.range);
                }

                Light::Directional(directional) => {
                    self.uniform_float("light.strength", directional.strength);
                    self.uniform_vec3("light.color", &directional.color);
                    self.uniform_vec4("light.vector", &directional.direction);
                }
            }

            self.uniform_vec3("viewPos", &camera.position);
            self.uniform_mat4("view", &camera.view);
            self.uniform_mat4("projection", &camera.projection);
        }
    }

    unsafe fn uniform_mat4(&self, name: &str, matrix: &glm::TMat4<f32>) {
        self.use_program();
        gl::UniformMatrix4fv(self.get_uniform(name), 1, gl::FALSE, glm::value_ptr(matrix).as_ptr());
    }

    unsafe fn uniform_vec3(&self, name: &str, vec: &glm::Vec3) {
        self.use_program();
        gl::Uniform3f(self.get_uniform(name), vec.x, vec.y, vec.z);
    }

    unsafe fn uniform_vec4(&self, name: &str, vec: &glm::Vec4) {
        self.use_program();
        gl::Uniform4f(self.get_uniform(name), vec.x, vec.y, vec.z, vec.w);
    }

    unsafe fn uniform_float(&self, name: &str, float: f32) {
        self.use_program();
        gl::Uniform1f(self.get_uniform(name), float);
    }

    unsafe fn uniform_uint(&self, name: &str, uint: u32) {
        self.use_program();
        gl::Uniform1ui(self.get_uniform(name), uint);
    }

    unsafe fn uniform_int(&self, name: &str, int: i32) {
        self.use_program();
        gl::Uniform1i(self.get_uniform(name), int);
    }

    pub unsafe fn use_program(&self) { gl::UseProgram(self.shader_id); }

    pub unsafe fn get_uniform(&self, name: &str) -> i32 {
        self.use_program();
        gl::GetUniformLocation(self.shader_id,
            CString::new(name).unwrap().as_ptr())
    }
}

pub struct Model<'a> {
    material: &'a Material,
    pub transform: glm::TMat4<f32>,
    indices: Vec<u32>,
    transform_id: i32,
    vao: u32,
    pub vbo: u32,
}

impl Model<'_> {
    pub fn new<'a>(obj_file: &'a str, material: &'a Material) -> Model<'a> {
        let (mut models, _) = tobj::load_obj(&obj_file, true).unwrap();
        let mut mesh = models.swap_remove(0).mesh;

        let mut full_verts = Vec::new();
        let vert_count = mesh.positions.len();

        full_verts.append(&mut mesh.positions);
        full_verts.append(&mut mesh.normals);

        let mut vao: u32 = 0;
        let mut vbo: u32 = 0;
        let mut ibo: u32 = 0;

        let transform_id = unsafe { material.get_uniform("transform") };

        let transform: glm::TMat4<f32> = glm::TMat4::identity();

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

            gl::VertexAttribPointer(1, 3, gl::FLOAT, gl::FALSE, 3 * FLOAT_SIZE as i32, (vert_count as isize * FLOAT_SIZE) as *const c_void);
            gl::EnableVertexAttribArray(1);

        };
        Model {
            material,
            transform,
            transform_id,
            indices: mesh.indices,
            vao,
            vbo,
        }
    }

    pub unsafe fn draw(&self) {
        self.material.use_program();
        self.material.uniform_mat4("transform", &self.transform);
        gl::BindVertexArray(self.vao);
        gl::DrawElements(gl::TRIANGLES, self.indices.len() as i32,
                         gl::UNSIGNED_INT, *self.indices.as_ptr() as *const c_void)
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.transform = glm::translate(
            &self.transform,
            &glm::vec3(x, y, z));
    }

    pub fn scale(&mut self, x: f32, y: f32, z: f32) {
        self.transform = glm::scale(&self.transform,
            &glm::vec3(x, y, z)
        );
    }

    pub fn rotate(&mut self, axis: glm::Vec3, degrees: f32) {
        self.transform = glm::rotate(
            &self.transform,
            glm::radians(&glm::vec1(degrees))[0],
            &axis
        );
    }
}

pub struct Camera {
    fov: f32,
    view: glm::Mat4,
    projection: glm::Mat4,
    pub position: glm::Vec3,
}

impl Camera {
    pub fn new(fov: f32, aspect_ratio: f32, near: f32, far: f32) -> Camera {
        let projection = glm::perspective(aspect_ratio,
            glm::radians(&glm::vec1(fov))[0] as f32, near, far);

        let position = glm::vec3(0.0, 0.0, 0.0);

        let view: glm::TMat4<f32> = glm::TMat4::identity();

        Camera {
            fov,
            view,
            projection,
            position,
        }
    }

    pub fn translate(&mut self, x: f32, y: f32, z: f32) {
        self.view = glm::translate(&self.view, &glm::vec3(x, y, z));
        self.position = &self.position + glm::vec3(x, y, z);
    }

    pub fn rotate(&mut self, axis: glm::Vec3, degrees: f32) {
        self.view = glm::rotate(
            &self.view,
            glm::radians(&glm::vec1(degrees))[0] as f32,
            &axis
        );
    }
}

pub enum Light {
    Point(PointLight),
    Directional(DirectionalLight),
}

pub struct DirectionalLight {
    color: glm::Vec3,
    strength: f32,
    direction: glm::Vec4,
}

pub struct PointLight {
    color: glm::Vec3,
    position: glm::Vec4,
    range: f32,
}

impl PointLight {
    pub fn new(color: glm::Vec3, position: glm::Vec3, range: f32) -> PointLight {
        let position = glm::vec4(position[0], position[1], position[2], 1.0);
        PointLight {
            color,
            position,
            range,
        }
    }
}
