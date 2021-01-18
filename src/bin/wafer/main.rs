use std::process;

use wafer::renderer::{RenderError, Renderer};

fn run_renderer() -> Result<(), RenderError> {
    let ttf_context = sdl2::ttf::init().unwrap();
    let mut renderer = Renderer::new(&ttf_context)?;
    'running: loop {
        if renderer.quit {
            break 'running;
        }
        renderer.update()?;
        renderer.run()?;
    }
    Ok(())
}

fn main() {
    let result = run_renderer();
    match result {
        Err(error) => {
            eprintln!("ERROR: {}", error);
        }
        Ok(()) => process::exit(0),
    }
}
