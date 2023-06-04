use super::config::load_config;
use std::path::PathBuf;

struct LayoutThread {
    layout: Layout,
    pending: Vec<NextItem>,
    current_ovr: Option<Overlay>,
    current_anim: Option<Animation>,
    running: bool,
}

impl LayoutThread {
    fn new(layout: Layout) -> Self {
        LayoutThread {
            layout,
            pending: Vec::new(),
            current_ovr: None,
            current_anim: None,
            running: true,
        }
    }

    fn stop(&mut self) {
        self.running = false;
    }

    fn push(&mut self, nxt: NextItem) {
        match nxt {
            NextItem::Overlay(ovr) => {
                if self.current_ovr.is_some()
                    || self
                        .pending
                        .iter()
                        .any(|item| matches!(item, NextItem::Overlay(_)))
                {
                    println!("Rejecting queued overlay");
                    return;
                } else {
                    println!("Queuing overlay");
                    self.pending.push(NextItem::Overlay(ovr));
                }
            }
            NextItem::Animation(anim) => {
                if anim.overlay_safe {
                    println!("Overlay-safe animation not queued");
                    self.pending.insert(0, NextItem::Animation(anim));
                } else {
                    println!("Queuing animation");
                    self.pending.push(NextItem::Animation(anim));
                }
            }
        }
    }

    fn on_overlay_destroyed(&mut self) {
        println!("Thread: Finishing overlay...");
        self.current_ovr = None;
        self.layout.exit_constant_damage();
    }

    fn run(&mut self) {
        while self.running {
            if let Some(item) = self.pending.get(0) {
                match item {
                    NextItem::Overlay(_) => {
                        if self.current_anim.is_none() && self.current_ovr.is_none() {
                            println!("Thread: Starting overlay...");
                            if let Some(NextItem::Overlay(ovr)) = self.pending.remove(0) {
                                self.current_ovr = Some(ovr);
                                self.layout
                                    .start_overlay(self.current_ovr.as_ref().unwrap());
                                self.layout.enter_constant_damage();
                            }
                        }
                    }
                    NextItem::Animation(anim) => {
                        if self.current_anim.is_none()
                            && (self.current_ovr.is_none() || anim.overlay_safe)
                        {
                            println!("Thread: Starting animation...");
                            if let Some(NextItem::Animation(anim)) = self.pending.remove(0) {
                                self.current_anim = Some(anim);
                                self.current_anim.as_mut().unwrap().start();
                                self.layout.enter_constant_damage();
                            }
                        }
                    }
                }
            }

            if let Some(anim) = &self.current_anim {
                if anim.check_finished() {
                    println!("Thread: Finishing animation...");
                    self.current_anim = None;
                    self.layout.exit_constant_damage();
                }
            }

            // conf_synchronous_update()();
            // Placeholder for conf_synchronous_update() function call

            thread::sleep(Duration::from_secs_f64(1.0 / 30.0));
        }
    }
}

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
