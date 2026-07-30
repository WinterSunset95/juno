mod state;
mod winit;
mod handlers;
// mod grabs;
// mod input;

use smithay::{
    reexports::{
        calloop::{ EventLoop },
        wayland_server::Display,
    },
};


use state::Juno;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    tracing_subscriber::fmt().init();
    tracing::info!("[-] Starting Juno...");

    let mut event_loop: EventLoop<Juno> = EventLoop::try_new()?;
    let display: Display<Juno> = Display::new()?;

    let mut state = Juno::new(&mut event_loop, display);

    winit::init_winit(&mut event_loop, &mut state)?;

    tracing::info!("[+] Juno initialized! Entering main loop!");

    event_loop.run(None, &mut state, move |_| {
        // juno is running
    })?;

    // while state.is_running {
    //     event_loop.dispatch(None, &mut state)?;
    // };

    tracing::info!("Juno Shutting Down");

    Ok(())
}
