use smithay::{
    backend::{
        renderer::{
            Frame, Renderer, damage::OutputDamageTracker, element::surface::WaylandSurfaceRenderElement, gles::GlesRenderer, utils::on_commit_buffer_handler
        }, winit::{self, WinitEvent}
    }, output::{
        Mode,
        Output,
        PhysicalProperties,
        Subpixel
    }, reexports::{
        calloop::EventLoop,
        wayland_server::Display,
    }, utils::{
        Rectangle, Scale, Transform
    }
};
use std::time::Duration;

use crate::Juno;

pub fn init_winit(event_loop: &mut EventLoop<Juno>, state: &mut Juno) -> Result<(), Box<dyn std::error::Error>> {
    let loop_handle = event_loop.handle();
    let _loop_signal  = event_loop.get_signal();
    let (mut backend, winit_source) = winit::init::<GlesRenderer>()?;

    backend.bind().expect("Failed to bind backend");
    let size = backend.window_size();
    let mode = Mode {
        size: size,
        refresh: 60_000,
    };

    let output = Output::new("winit".to_string(), 
        PhysicalProperties {
            size: (0, 0).into(),
            subpixel: Subpixel::Unknown,
            make: "Juno".into(),
            model: "winit".into(),
            serial_number: "Unknown".into()
        });

    let _global = output.create_global::<Juno>(&state.display_handle);
    output.change_current_state(
        Some(mode), 
        Some(Transform::Flipped180), 
        None,
        Some((0,0).into())
        );
    output.set_preferred(mode);

    let mut damage_tracker = OutputDamageTracker::from_output(&output);

    // AI! What is this "move"? What is it used for?
    loop_handle.insert_source(winit_source, move |event, _metadata, state| {
        match event {
            WinitEvent::CloseRequested => {
                tracing::info!("Host clicked the 'X' button. Exiting juno...");
                state.loop_signal.stop();
            }
            WinitEvent::Resized { size, scale_factor } => {
                tracing::info!("Juno was resized to {}x{}", size.w, size.h);
            }
            WinitEvent::Redraw => {
                let size = backend.window_size();
                let damage = Rectangle::from_size(size);

                // AI! I have no idea what this part of the code is doing. Please explain it to me
                // in good detail.
                {
                    let (renderer, mut framebuffer) = backend.bind().unwrap();
                    smithay::desktop::space::render_output::<
                        _,
                        WaylandSurfaceRenderElement<GlesRenderer>,
                        _,
                        _,
                    >(
                        &output,
                        renderer,
                        &mut framebuffer,
                        1.0,
                        0,
                        [&state.space],
                        &[],
                        &mut damage_tracker,
                        [0.1, 0.1, 0.1, 1.0],
                    )
                    .unwrap();
                }

                backend.submit(Some(&[damage])).unwrap();
            }
            _ => {}
        }
    }).map_err(|_| "Failed to insert winit event into calloop")?;

    Ok(())
}
