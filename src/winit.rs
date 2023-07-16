use crate::data;
use crate::data::WinitData;
use crate::state;
use anyhow;
use smithay::reexports::calloop::generic::Generic;
use smithay::reexports::calloop::{Interest, Mode, PostAction};
use smithay::wayland::socket::ListeningSocketSource;
use smithay::{
    backend::{
        allocator::dmabuf::Dmabuf,
        egl::EGLDevice,
        renderer::{
            damage::{Error as OutputDamageTrackerError, OutputDamageTracker},
            element::AsRenderElements,
            gles::{GlesRenderer, GlesTexture},
            ImportDma, ImportMemWl,
        },
        winit::{self, WinitEvent, WinitGraphicsBackend},
        SwapBuffersError,
    },
    delegate_dmabuf,
    input::pointer::{CursorImageAttributes, CursorImageStatus},
    output::{Output, PhysicalProperties, Subpixel},
    reexports::{
        calloop::EventLoop,
        wayland_protocols::wp::presentation_time::server::wp_presentation_feedback,
        wayland_server::{protocol::wl_surface, Display},
    },
    utils::{IsAlive, Point, Scale, Transform},
    wayland::{
        compositor,
        dmabuf::{
            DmabufFeedback, DmabufFeedbackBuilder, DmabufGlobal, DmabufHandler, DmabufState,
            ImportError,
        },
        input_method::InputMethodSeat,
    },
};
use std::sync::Arc;

pub fn run_winit() -> anyhow::Result<(), anyhow::Error> {
    let mut event_loop: EventLoop<data::WinitData> = EventLoop::try_new().unwrap();
    let mut display: Display<state::RenewmState<WinitData>> = Display::new().unwrap();

    todo!()
}
