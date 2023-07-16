use crate::data;
use crate::state;
use anyhow;
use smithay::{
    reexports::{calloop::EventLoop, wayland_server::Display},
    wayland::socket::ListeningSocketSource,
};
use std::sync::Arc;

pub fn run_winit() -> anyhow::Result<(), anyhow::Error> {
    let mut event_loop: EventLoop<data::Data> = EventLoop::try_new().unwrap();
    let mut display: Display<state::RenewmState> = Display::new().unwrap();

    let socket = ListeningSocketSource::new_auto()?;
    let socket_name = socket.socket_name().to_os_string();

    event_loop
        .handle()
        .insert_source(socket, |stream, _, data| {
            // Insert a new client into Display with data associated with that client.
            // This starts the management of the client, the communication is over the UnixStream.
            data.display
                .handle()
                .insert_client(stream, Arc::new(data::ClientData::default()))
                .unwrap();
        })?;

    todo!()
}
