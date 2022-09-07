mod graphics;

mod game;
mod menu;
mod resource;
mod util;

use game::Game;
use std::rc::Rc;

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

    let gl = Rc::new(
        unsafe {
            graphics::GlFns::load_from(&|ptr| {
                context
                    .get_proc_address(std::ffi::CStr::from_ptr(ptr as *const i8).to_str().unwrap())
            })
        }
        .expect("Couldn't load OpenGL!"),
    );

    unsafe {
        gl.Enable(gl33::GL_TEXTURE_2D);
        gl.Enable(gl33::GL_BLEND);
        gl.BlendFunc(gl33::GL_SRC_ALPHA, gl33::GL_ONE_MINUS_SRC_ALPHA);
    }

    let roman = resource::ResourceManager::new(String::from("resources")).unwrap();
    let mut screen: Box<dyn Playable> = Box::new(menu::Menu::new(gl.clone(), &roman));

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
                    gl.Viewport(0, 0, winsize.width as _, winsize.height as _);
                    gl.ClearColor(0.0, 0.0, 0.0, 1.0);
                    gl.Clear(gl33::GL_COLOR_BUFFER_BIT | gl33::GL_DEPTH_BUFFER_BIT);
                }

                screen.draw(winsize.width as _, winsize.height as _);

                if let Some(x) = screen.next_screen() {
                    screen = match x {
                        Screen::Menu => Box::new(menu::Menu::new(gl.clone(), &roman)),
                        Screen::SingleGame => Box::new(Game::single(gl.clone(), &roman)),
                        Screen::DoubleGame => Box::new(Game::double(gl.clone(), &roman)),
                    };
                }

                context.swap_buffers().unwrap();
            }
            _ => (),
        }
    });
}
