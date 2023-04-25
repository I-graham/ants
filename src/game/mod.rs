mod fsm;
mod messenger;
mod state;
mod utils;
mod world;

use crate::window::{External, Input, Instance};
pub use fsm::*;
pub use messenger::*;
pub use utils::*;

use winit::event_loop::EventLoop;
use world::World;

pub trait GameObject {
    fn plan(
        &self,
        _world: &World,
        _external: &External,
        _input: &Input,
        _messenger: &mut Messenger,
    ) {
    }

    fn update(&mut self, _external: &External, _messenger: &Messenger) -> Option<Action> {
        None
    }

    //If object renders a single instance, this can be implemented instea
    //of GameObject::render
    fn instance(&self, _external: &External) -> Option<Instance> {
        None
    }

    fn render(&self, external: &External, out: &mut Vec<Instance>) {
        if let Some(inst) = self.instance(external) {
            external.clip(out, inst);
        }
    }
}

#[derive(Clone, PartialEq)]
pub enum Action {
    Die,
}

pub fn play() -> ! {
    use winit::event::{Event, WindowEvent};

    let event_loop = EventLoop::new();
    let mut game = state::GameState::new(&event_loop);

    let mut prev = std::time::Instant::now();
    let mut frame_counter = 0;

    event_loop.run(move |event, _, flow| {
        flow.set_poll();
        match event {
            Event::WindowEvent { event, window_id } if window_id == game.api.id() => match event {
                WindowEvent::CloseRequested => {
                    flow.set_exit();
                }

                WindowEvent::Resized(dims) if dims.height != 0 && dims.width != 0 => {
                    game.api.resize(dims);
                }

                WindowEvent::KeyboardInput { input, .. } => game.api.input.capture_key(input),

                WindowEvent::MouseWheel { delta, .. } => {
                    use winit::dpi::PhysicalPosition;
                    use winit::event::MouseScrollDelta::*;
                    game.api.input.scroll = match delta {
                        LineDelta(_hor, ver) => ver,
                        PixelDelta(PhysicalPosition { y, .. }) => y as f32,
                    };
                }

                WindowEvent::CursorMoved { position, .. } => {
                    game.api
                        .input
                        .capture_mouse(&position, game.api.external.size);
                }

                WindowEvent::MouseInput { button, state, .. } => game
                    .api
                    .input
                    .mouse_button(&button, state == winit::event::ElementState::Pressed),

                _ => {}
            },

            Event::MainEventsCleared => {
                {
                    frame_counter += 1;
                    let now = game.api.external.now;
                    let time = now.duration_since(prev).as_secs_f64();
                    if time > 1. {
                        println!("fps: {}", frame_counter);
                        prev = now;
                        frame_counter = 0;
                    }
                }

                game.step();
                game.draw();
                game.api.submit();
            }

            _ => {}
        }
    })
}
