use misc::context::{Context, Err};

mod cpu;
mod dos;
mod fpu;
mod hardware;
mod ints;
mod misc;
mod shell;

fn main() -> Result<(), Err>{
    env_logger::init();

    let mut context = Context::new().expect("Cannot create dosbox context.");
    context.parse_args();

    Ok(())
}
