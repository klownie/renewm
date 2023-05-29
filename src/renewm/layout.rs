use super::config::load_config;
use std::path::PathBuf;

pub struct Layout {
    config_file: Option<PathBuf>,
    debug: bool,
    key_processor: KeyProcessor,
    auth_backend: AuthBackend,
    panel_launcher: PanelsLauncher,
    dbus_endpoint: DBusEndpoint,
    gesture_providers: Vec<GestureProvider>,
    workspaces: Vec<Workspace>,
    state: LayoutState,
    overlay: Option<Overlay>,
    backgrounds: Vec<Background>,
    top_bars: Vec<TopBar>,
    bottom_bars: Vec<BottomBar>,
    corners: Vec<Vec<Corner>>,
    focus_borders: FocusBorders,
    thread: LayoutThread,
    animations: Vec<Animation>,
    idle_inhibit_user: bool,
    active_workspace: (Workspace, Option<Workspace>),
}

impl Layout {
    pub fn new(debug: bool, config_file: Option<PathBuf>) -> Self {
        load_config(config_file.clone());

        Layout {
            config_file,
            debug,
            key_processor: KeyProcessor::new(),
            auth_backend: AuthBackend::new(),
            panel_launcher: PanelsLauncher::new(),
            dbus_endpoint: DBusEndpoint::new(),
            gesture_providers: Vec::new(),
            workspaces: vec![Workspace::new(
                PyWMOutput::new("dummy", -1, 1.0, 1280, 720, (0, 0)),
                0,
                0,
                1280,
                720,
            )],
            state: LayoutState::new(),
            overlay: None,
            backgrounds: Vec::new(),
            top_bars: Vec::new(),
            bottom_bars: Vec::new(),
            corners: Vec::new(),
            focus_borders: FocusBorders::new(),
            thread: LayoutThread::new(),
            animations: Vec::new(),
            idle_inhibit_user: false,
            active_workspace: (workspaces[0].clone(), None),
        }
    }
}
