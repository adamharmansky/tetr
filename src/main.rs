mod graphics;

mod game;
mod menu;
mod resource;
mod text;
mod util;

use game::Game;
use std::{cell::RefCell, rc::Rc};

use glutin::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

use util::*;

fn main() {
    let evloop = EventLoop::new();

    let builder = WindowBuilder::new()
        .with_title("Definitely not a tetr.io clone")
        .with_inner_size(glutin::dpi::PhysicalSize::new(1000, 600))
        .with_resizable(true);

    let context = glutin::ContextBuilder::new()
        .with_gl(glutin::GlRequest::Specific(glutin::Api::OpenGl, (3, 3)))
        .with_vsync(true)
        .with_multisampling(4)
        .build_windowed(builder, &evloop)
        .expect("Couldn't create context!");
    let context = unsafe { context.make_current().unwrap() };

    let gl = unsafe {
        graphics::GlFns::load_from(&|ptr| {
            context.get_proc_address(std::ffi::CStr::from_ptr(ptr as *const i8).to_str().unwrap())
        })
    }
    .expect("Couldn't load OpenGL!");

    unsafe {
        gl.Enable(gl33::GL_TEXTURE_2D);
        gl.Enable(gl33::GL_BLEND);
        gl.BlendFunc(gl33::GL_SRC_ALPHA, gl33::GL_ONE_MINUS_SRC_ALPHA);
    }

    let mut gh = graphics::GraphicsHandle::new(gl);
    let roman = resource::ResourceManager::new(String::from("resources")).unwrap();
    let tr = Rc::new(text::TextRenderer::new(&mut gh, &roman).unwrap());
    let mut screen: Box<dyn Playable> = Box::new(menu::Menu::new(&mut gh, &roman, tr.clone()));

    evloop.run(move |ev, _, control_flow| {
        *control_flow = ControlFlow::Wait;
        match ev {
            Event::LoopDestroyed => return,
            Event::WindowEvent { event, .. } => match event {
                WindowEvent::CloseRequested => *control_flow = ControlFlow::Exit,
                WindowEvent::KeyboardInput { input, .. } => screen.input(input),
                _ => (),
            },
            Event::MainEventsCleared => {
                screen.update();

                let winsize = context.window().inner_size();
                unsafe {
                    gh.gl
                        .Viewport(0, 0, winsize.width as _, winsize.height as _);
                    gh.gl.ClearColor(0.0, 0.0, 0.0, 1.0);
                    gh.gl
                        .Clear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
                }

                screen.draw(&mut gh, winsize.width as _, winsize.height as _);

                if let Some(x) = screen.next_screen() {
                    match x {
                        Screen::Menu => {
                            screen = Box::new(menu::Menu::new(&mut gh, &roman, tr.clone()))
                        }
                        Screen::SingleGame => {
                            screen = Box::new(Game::new(
                                &mut gh,
                                &roman,
                                tr.clone(),
                                game::GameMode::Single,
                            ))
                        }
                        Screen::DoubleGame => {
                            screen = Box::new(Game::new(
                                &mut gh,
                                &roman,
                                tr.clone(),
                                game::GameMode::Double,
                            ))
                        }
                        Screen::Exit => {
                            *control_flow = ControlFlow::Exit;
                            return;
                        }
                    };
                }
                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
