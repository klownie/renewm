pub struct Layout {
    config_file: Option<String>,
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
    pub fn new(debug: bool, config_file: Option<String>) -> Self {
        let config_file_copy = config_file.clone();
        load_config(&config_file_copy);

        let key_processor = KeyProcessor::new();
        let auth_backend = AuthBackend::new();
        let panel_launcher = PanelsLauncher::new();
        let dbus_endpoint = DBusEndpoint::new();

        let gesture_providers: Vec<GestureProvider> = vec![];

        let workspaces: Vec<Workspace> = vec![Workspace::new(
            PyWMOutput::new("dummy", -1, 1.0, 1280, 720, (0, 0)),
            0,
            0,
            1280,
            720,
        )];

        let state = LayoutState::new();

        let overlay = None;

        let backgrounds: Vec<Background> = vec![];
        let top_bars: Vec<TopBar> = vec![];
        let bottom_bars: Vec<BottomBar> = vec![];
        let corners: Vec<Vec<Corner>> = vec![];
        let focus_borders = FocusBorders::new();

        let thread = LayoutThread::new();

        let animations: Vec<Animation> = vec![];

        let idle_inhibit_user = false;

        let active_workspace = (workspaces[0].clone(), None);

        Layout {
            config_file,
            debug,
            key_processor,
            auth_backend,
            panel_launcher,
            dbus_endpoint,
            gesture_providers,
            workspaces,
            state,
            overlay,
            backgrounds,
            top_bars,
            bottom_bars,
            corners,
            focus_borders,
            thread,
            animations,
            idle_inhibit_user,
            active_workspace,
        }
    }
}
