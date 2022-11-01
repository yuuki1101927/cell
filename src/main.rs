mod vertex;
mod vector3d;
mod vector2d;
mod matrix4d;
mod cell;
mod field;

#[macro_use]
extern crate glium;

use std::f32::consts::PI;
use std::time::Instant;
use vertex::Vertex;
use glium::{glutin, Surface};
use glium::uniforms::{EmptyUniforms, UniformsStorage};
use crate::cell::{State, VERTEX1, VERTEX2, VERTEX3, VERTEX4};
use crate::field::Field;
use crate::glutin::event::VirtualKeyCode;
use crate::glutin::event_loop::ControlFlow;
use crate::matrix4d::Matrix4d;
use crate::vector2d::Vector2d;
use crate::vector3d::Vector3d;

static VERTEX_SHADER_SRC: &str = r#"
    #version 140

    in vec2 position;

    uniform vec2 movement;

    uniform mat4 projection;
    uniform mat4 view;
    uniform mat4 model;

    void main() {
        vec2 pos = position;
        pos.x += movement.x;
        pos.y += movement.y;
        mat4 mvp = projection * view * model;
        gl_Position = mvp * vec4(pos, 0.0, 1.0);
    }
"#;

static FRAGMENT_SHADER_SRC: &str = r#"
    #version 140

    in vec2 tex_coords;

    uniform vec3 in_color;

    out vec4 color;

    void main() {
        color = vec4(in_color, 1.0);
    }
"#;

const TARGET_FPS: u64 = 60;

fn main() {
    println!("Hello, world!");

    let mut event_loop = glutin::event_loop::EventLoop::new();
    let wb = glutin::window::WindowBuilder::new();
    let cb = glutin::ContextBuilder::new();
    let display = glium::Display::new(wb, cb, &event_loop).unwrap();

    let shape = vec![VERTEX1,VERTEX2,VERTEX3,VERTEX4];
    let vertex_buffer = glium::VertexBuffer::new(&display, &shape).unwrap();

    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TriangleStrip);

    let program = glium::Program::from_source(
        &display,
        VERTEX_SHADER_SRC,
        FRAGMENT_SHADER_SRC,
        None).unwrap();

    let mut camera_position = Vector3d::new(15.0, -15.0, 30.0);

    let mut  horizontal_angle:f32 = PI;
    let mut  vertical_angle:f32 = 0.0;

    let mut w_pressed = false;
    let mut s_pressed = false;
    let mut a_pressed = false;
    let mut d_pressed = false;
    let mut space_pressed = false;
    let mut shift_pressed = false;
    let mut up_pressed = false;
    let mut down_pressed = false;
    let mut t_pressed = false;

    let mut time = Instant::now();

    let mut ticks:f32 = 0.0;

    let mut field = Field::new(200,200, State::S0);

    field.raw[6][6] = State::S1;
    field.raw[7][6] = State::S1;
    field.raw[8][6] = State::S1;
    field.raw[9][6] = State::S1;
    field.raw[10][6] = State::S1;
    field.raw[11][6] = State::S1;
    field.raw[6][7] = State::S1;
    field.raw[7][7] = State::S1;
    field.raw[8][7] = State::S1;
    field.raw[9][7] = State::S1;
    field.raw[10][7] = State::S1;
    field.raw[11][7] = State::S1;

    field.raw[6+3][6+7] = State::S1;
    field.raw[7+3][6+7] = State::S1;
    field.raw[8+3][6+7] = State::S1;
    field.raw[9+3][6+7] = State::S1;
    field.raw[10+3][6+7] = State::S1;
    field.raw[11+3][6+7] = State::S1;
    field.raw[6+3][7+7] = State::S1;
    field.raw[7+3][7+7] = State::S1;
    field.raw[8+3][7+7] = State::S1;
    field.raw[9+3][7+7] = State::S1;
    field.raw[10+3][7+7] = State::S1;
    field.raw[11+3][7+7] = State::S1;

    field.raw[13][6] = State::S1;
    field.raw[13][7] = State::S1;
    field.raw[13][8] = State::S1;
    field.raw[13][9] = State::S1;
    field.raw[13][10] = State::S1;
    field.raw[13][11] = State::S1;
    field.raw[14][6] = State::S1;
    field.raw[14][7] = State::S1;
    field.raw[14][8] = State::S1;
    field.raw[14][9] = State::S1;
    field.raw[14][10] = State::S1;
    field.raw[14][11] = State::S1;

    field.raw[14-8][6+3] = State::S1;
    field.raw[14-8][7+3] = State::S1;
    field.raw[14-8][8+3] = State::S1;
    field.raw[14-8][9+3] = State::S1;
    field.raw[14-8][10+3] = State::S1;
    field.raw[14-8][11+3] = State::S1;
    field.raw[15-8][6+3] = State::S1;
    field.raw[15-8][7+3] = State::S1;
    field.raw[15-8][8+3] = State::S1;
    field.raw[15-8][9+3] = State::S1;
    field.raw[15-8][10+3] = State::S1;
    field.raw[15-8][11+3] = State::S1;

    event_loop.run(move |event, _, control_flow| {
        let start_time = Instant::now();
        let delta_time = start_time.duration_since(time);
        match event {
            glutin::event::Event::DeviceEvent { event, .. } => match event {
                glutin::event::DeviceEvent::MouseMotion { delta } => {
                    horizontal_angle -= delta.0 as f32 / 500.0;
                    vertical_angle -= delta.1 as f32 / 500.0;
                    return;
                },
                _ => {return;},
            },
            glutin::event::Event::WindowEvent { event, .. } => match event {
                glutin::event::WindowEvent::CloseRequested => {
                    *control_flow = ControlFlow::Exit;
                    return;
                },
                glutin::event::WindowEvent::KeyboardInput { input, .. } => match input.virtual_keycode {
                    Some(VirtualKeyCode::Escape) => {
                        *control_flow = ControlFlow::Exit;
                        return;
                    },
                    Some(VirtualKeyCode::W) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            w_pressed = true;
                        } else {
                            w_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::S) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            s_pressed = true;
                        } else {
                            s_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::A) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            a_pressed = true;
                        } else {
                            a_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::D) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            d_pressed = true;
                        } else {
                            d_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::Space) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            space_pressed = true;
                        } else {
                            space_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::LControl) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            shift_pressed = true;
                        } else {
                            shift_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::Up) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            up_pressed = true;
                        } else {
                            up_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::Down) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            down_pressed = true;
                        } else {
                            down_pressed = false;
                        }
                    },
                    Some(VirtualKeyCode::T) => {
                        if input.state == glutin::event::ElementState::Pressed {
                            t_pressed = true;
                        } else {
                            t_pressed = false;
                        }
                    }
                    _ => {}
                },
                _ => return,
            },
            glutin::event::Event::NewEvents(cause) => match cause {
                glutin::event::StartCause::ResumeTimeReached { .. } => (),
                glutin::event::StartCause::Init => (),
                _ => return,
            },
            _ => return,
        }

        if up_pressed {
        }
        if down_pressed {
        }

        let mut camera_direction = Vector3d::new(vertical_angle.cos() * horizontal_angle.sin(),
                                                 vertical_angle.sin(),
                                                 vertical_angle.cos() *horizontal_angle.cos()
        );

        let right = Vector3d::new(0.0, 1.0, 0.0).cross(camera_direction).normalize();
        let up = camera_direction.cross(right).normalize();

        if w_pressed {
            camera_position += camera_direction.bird_view_z().scale(delta_time.as_secs_f32() * 5.0);
        }

        if s_pressed {
            camera_position -= camera_direction.bird_view_z().scale(delta_time.as_secs_f32() * 5.0);
        }

        if a_pressed {
            camera_position += right.bird_view_z().scale(delta_time.as_secs_f32() * 5.0);
        }

        if d_pressed {
            camera_position -= right.bird_view_z().scale(delta_time.as_secs_f32() * 5.0);
        }

        if space_pressed {
            camera_position.y += delta_time.as_secs_f32() * 5.0;
        }

        if shift_pressed {
            camera_position.y -= delta_time.as_secs_f32() * 5.01;
        }

        if t_pressed {
            field.tick_all();
        }


        let projection = Matrix4d::perspective(90.0 * PI / 180.0 , 1.0, 0.1, 100.0);
        let view = Matrix4d::look_at(
            camera_position,
            camera_position + camera_direction,
            up,
        );
        let model = Matrix4d::translate(0.0, 0.0, -1.0);

        let uniforms = uniform! {
            model: model.to_list(),
            view: view.to_list(),
            projection: projection.to_list(),
            ticks: ticks,
        };

        let mut target = display.draw();
        target.clear_color(0.5,0.5,0.5, 1.0);

        target.draw(&vertex_buffer, &indices, &program,
                    &uniforms.add("movement", [-2.2f32, -2.2f32])
                        .add("in_color", [0.2f32, 0.2f32, 1.0f32]),
                    &Default::default()).unwrap();

        for (y,xs) in (0_i32..).zip(field.raw.iter()) {
            for (x, state) in (0_i32..).zip(xs.iter()) {
                target.draw(&vertex_buffer, &indices, &program,
                            &uniforms.add("movement", [(x as f32) * 2.2, (y as f32) * -2.2])
                                .add("in_color", state.to_color().to_list()),
                            &Default::default()).unwrap();
            }
        }

        target.finish().unwrap();

        ticks += 0.05;

        let next_frame_time = start_time +
            std::time::Duration::from_nanos(16_666_667);
        *control_flow = glutin::event_loop::ControlFlow::WaitUntil(next_frame_time);
        time = Instant::now();
    });
}