use smithay::reexports::calloop::{EventLoop, LoopSignal};
use smithay::reexports::wayland_server::Display;

pub struct JunoState {
    pub is_running: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> { // What is this return?
    tracing_subscriber::fmt().init(); // What does this do?
    tracing::info!("[-] Starting Juno...");

    let mut event_loop: EventLoop<JunoState> = EventLoop::try_new()?; // What's this question mark?

    let loop_handle = event_loop.handle();
    let loop_signal  = event_loop.get_signal();

    let mut display: Display<JunoState> = Display::new()?;

    let mut state = JunoState {
        is_running: true,
    };

    tracing::info!("[+] Juno initialized! Entering main loop!");

    while state.is_running {
        event_loop.dispatch(None, &mut state)?; // What does the & sign mean?
    };

    Ok(())
}
