use crate::data::Backend;
use smithay::{
    backend::renderer::element::{
        default_primary_scanout_output_compare, utils::select_dmabuf_feedback, RenderElementStates,
    },
    delegate_compositor, delegate_data_device, delegate_fractional_scale,
    delegate_input_method_manager, delegate_keyboard_shortcuts_inhibit, delegate_layer_shell,
    delegate_output, delegate_presentation, delegate_primary_selection, delegate_relative_pointer,
    delegate_seat, delegate_shm, delegate_tablet_manager, delegate_text_input_manager,
    delegate_viewporter, delegate_virtual_keyboard_manager, delegate_xdg_activation,
    delegate_xdg_decoration, delegate_xdg_shell,
    desktop::{
        utils::{
            surface_presentation_feedback_flags_from_states, surface_primary_scanout_output,
            update_surface_primary_scanout_output, OutputPresentationFeedback,
        },
        PopupManager, Space, Window,
    },
    input::{keyboard::XkbConfig, pointer::CursorImageStatus, Seat, SeatHandler, SeatState},
    output::Output,
    reexports::{
        calloop::{generic::Generic, Interest, LoopHandle, LoopSignal, Mode, PostAction},
        wayland_protocols::xdg::decoration::{
            self as xdg_decoration,
            zv1::server::zxdg_toplevel_decoration_v1::Mode as DecorationMode,
        },
        wayland_server::{
            backend::{ClientData, ClientId, DisconnectReason},
            protocol::{wl_data_source::WlDataSource, wl_surface::WlSurface},
            Display, DisplayHandle, Resource,
        },
    },
    utils::{Clock, Logical, Monotonic, Point, Rectangle},
    wayland::{
        compositor::{get_parent, with_states, CompositorClientState, CompositorState},
        data_device::{
            set_data_device_focus, ClientDndGrabHandler, DataDeviceHandler, DataDeviceState,
            ServerDndGrabHandler,
        },
        dmabuf::DmabufFeedback,
        fractional_scale::{
            with_fractional_scale, FractionalScaleHandler, FractionalScaleManagerState,
        },
        input_method::InputMethodManagerState,
        keyboard_shortcuts_inhibit::{
            KeyboardShortcutsInhibitHandler, KeyboardShortcutsInhibitState,
            KeyboardShortcutsInhibitor,
        },
        output::OutputManagerState,
        presentation::PresentationState,
        primary_selection::{set_primary_focus, PrimarySelectionHandler, PrimarySelectionState},
        relative_pointer::RelativePointerManagerState,
        seat::WaylandFocus,
        shell::{
            wlr_layer::WlrLayerShellState,
            xdg::{
                decoration::{XdgDecorationHandler, XdgDecorationState},
                ToplevelSurface, XdgShellState, XdgToplevelSurfaceData,
            },
        },
        shm::{ShmHandler, ShmState},
        socket::ListeningSocketSource,
        tablet_manager::TabletSeatTrait,
        text_input::TextInputManagerState,
        viewporter::ViewporterState,
        virtual_keyboard::VirtualKeyboardManagerState,
        xdg_activation::{
            XdgActivationHandler, XdgActivationState, XdgActivationToken, XdgActivationTokenData,
        },
    },
};
use std::ffi::OsString;
use std::sync::atomic::AtomicBool;
use std::sync::Arc;

pub struct CalloopData<BackendData: Backend + 'static> {
    pub state: RenewmState<BackendData>,
    pub display: Display<RenewmState<BackendData>>,
}

pub struct RenewmState<BackendData: Backend + 'static> {
    // Renewm State
    pub start_time: std::time::Instant,
    pub socket_name: OsString,
    pub display_handle: DisplayHandle,
    pub running: Arc<AtomicBool>,
    pub handle: LoopHandle<'static, CalloopData<BackendData>>,

    // Desktop
    pub space: Space<Window>,
    pub loop_signal: LoopSignal,
    pub grid: Rectangle<u32, Logical>,

    // Smithay State
    pub compositor_state: CompositorState,
    pub xdg_shell_state: XdgShellState,
    pub shm_state: ShmState,
    pub output_manager_state: OutputManagerState,
    pub seat_state: SeatState<Self>,
    pub data_device_state: DataDeviceState,
    pub layer_shell_state: WlrLayerShellState,
    pub primary_selection_state: PrimarySelectionState,
    pub keyboard_shortcuts_inhibit_state: KeyboardShortcutsInhibitState,
    pub viewporter_state: ViewporterState,
    pub xdg_activation_state: XdgActivationState,
    pub xdg_decoration_state: XdgDecorationState,
    pub presentation_state: PresentationState,
    pub fractional_scale_manager_state: FractionalScaleManagerState,

    pub dnd_icon: Option<WlSurface>,

    // input related state
    pub cursor_status: CursorImageStatus,
    pub pointer_location: Point<f64, Logical>,
    pub suppressed_keys: Vec<u32>,
    pub seat_name: String,
    pub seat: Seat<RenewmState<BackendData>>,
    pub clock: Clock<Monotonic>,
}

impl<BackendData: Backend> SeatHandler for RenewmState<BackendData> {
    type KeyboardFocus = WlSurface;
    type PointerFocus = WlSurface;

    fn seat_state(&mut self) -> &mut SeatState<Self> {
        &mut self.seat_state
    }

    fn cursor_image(
        &mut self,
        _: &smithay::input::Seat<Self>,
        image: smithay::input::pointer::CursorImageStatus,
    ) {
        self.cursor_status = image;
    }

    fn focus_changed(&mut self, _: &smithay::input::Seat<Self>, _: Option<&WlSurface>) {}
}
