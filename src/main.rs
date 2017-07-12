#![recursion_limit = "1024"]

extern crate gl;
extern crate glutin;
extern crate libc;
#[macro_use] extern crate error_chain;

mod errors {
    error_chain!{}
}

use glutin::GlContext;

use errors::*;

fn run() -> Result<()> {
    let mut event_loop = glutin::EventsLoop::new();
    let window = build_window(&event_loop)?;

    let mut running = true;
    while running {
        event_loop.poll_events(|event| {
            use glutin::{ Event, WindowEvent };

            match event {
                Event::WindowEvent { event, .. } => match event {
                    WindowEvent::Closed => running = false,
                    WindowEvent::Resized(w, h) => window.resize(w, h),
                    _ => (),
                },
                _ => (),
            }
        });

        unsafe {
            gl::Clear(gl::COLOR_BUFFER_BIT);
        }

        window.swap_buffers()
            .chain_err(|| "Unable to swap buffers")?;
    }

    Ok(())
}

fn build_window(event_loop: &glutin::EventsLoop) -> Result<glutin::GlWindow> {
    let window = glutin::WindowBuilder::new()
        .with_title("Hello Glutin !")
        .with_dimensions(1024, 768);
    let context = glutin::ContextBuilder::new().with_vsync(true);

    let gl_window = glutin::GlWindow::new(window, context, &event_loop)
        .chain_err(|| "Unable to build window")?;

    unsafe { gl_window.make_current() }
        .chain_err(|| "Unable to make window current")?;

    unsafe {
        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
    }

    Ok(gl_window)
}

fn main() {
    if let Err(ref e) = run() {
        use std::io::Write;
        let stderr = &mut ::std::io::stderr();
        let errmsg = "Error writing to stderr";

        writeln!(stderr, "error: {}", e).expect(errmsg);

        for e in e.iter().skip(1) {
            writeln!(stderr, "caused by: {}", e).expect(errmsg);
        }

        // The backtrace is not always generated. Try to run this example
        // with `RUST_BACKTRACE=1`.
        if let Some(backtrace) = e.backtrace() {
            writeln!(stderr, "backtrace: {:?}", backtrace).expect(errmsg);
        }

        ::std::process::exit(1);
    }
}