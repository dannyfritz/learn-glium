#[macro_use]
extern crate glium;
extern crate nalgebra;

use glium::{glutin, Surface};
use nalgebra::Matrix4;
use std::fs::File;
use std::io::prelude::*;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
}
implement_vertex!(Vertex, position);

fn main() {
    let mut events_loop = glutin::EventsLoop::new();
    let window = glutin::WindowBuilder::new();
    let context = glutin::ContextBuilder::new();
    let display = glium::Display::new(window, context, &events_loop).unwrap();

    let vertex1 = Vertex {
        position: [-0.5, -0.5],
    };
    let vertex2 = Vertex {
        position: [0.0, 0.5],
    };
    let vertex3 = Vertex {
        position: [0.5, -0.25],
    };
    let shape = vec![vertex1, vertex2, vertex3];

    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let mut projection = Matrix4::<f32>::new_orthographic(0.0, 1.0, 0.0, 1.0, -1.0, 1.0);
    let mut view = Matrix4::<f32>::identity();
    let mut model = Matrix4::<f32>::identity();

    let program = glium::Program::from_source(
        &display,
        get_file_string("./shaders/simple.vert").as_str(),
        get_file_string("./shaders/simple.frag").as_str(),
        None,
    ).unwrap();

    let mut closed = false;
    while !closed {
        let mut target = display.draw();
        target.clear_color(0.0, 0.0, 1.0, 1.0);
        let projection_slice: [[f32; 4]; 4] = projection.into();
        let view_slice: [[f32; 4]; 4] = view.into();
        let model_slice: [[f32; 4]; 4] = model.into();
        let uniforms = uniform! {
            projection: projection_slice,
            view: view_slice,
            model: model_slice,
        };
        target
            .draw(
                &vertex_buffer,
                &indices,
                &program,
                &uniforms,
                &Default::default(),
            )
            .unwrap();
        target.finish().unwrap();
        events_loop.poll_events(|event| match event {
            glutin::Event::WindowEvent { event, .. } => match event {
                glutin::WindowEvent::Closed => closed = true,
                _ => (),
            },
            _ => (),
        });
    }
}

fn get_file_string(file_name: &str) -> String {
    let mut file = File::open(file_name).unwrap();
    let mut file_src = String::new();
    file.read_to_string(&mut file_src).unwrap();
    file_src
}
