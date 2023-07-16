use crate::state::RenewmState;
use smithay::{
    reexports::wayland_server::{backend, Display},
    wayland::compositor::CompositorClientState,
};

// Used for the calloop::EventLoop data
pub struct Data {
    pub display: Display<RenewmState>,
    pub state: RenewmState,
}

// Used to store client data associated with Wayland clients
#[derive(Default)]
pub struct ClientData {
    pub compositor_state: CompositorClientState,
}

impl backend::ClientData for ClientData {}
