trait Interpolation<T> {
    fn get(&self, at: f64) -> T;
}

struct LayoutDownstreamInterpolation<'a> {
    lock_perc: (f64, f64),
    layout: &'a Layout,
    state0: &'a PyWMDownstreamState,
    state1: &'a PyWMDownstreamState,
}

impl<'a> LayoutDownstreamInterpolation<'a> {
    fn new(layout: &'a Layout, state0: &'a PyWMDownstreamState, state1: &'a PyWMDownstreamState) -> Self {
        Self {
            lock_perc: (state0.lock_perc, state1.lock_perc),
            layout,
            state0,
            state1,
        }
    }
}

impl<'a> Interpolation<PyWMDownstreamState> for LayoutDownstreamInterpolation<'a> {
    fn get(&self, at: f64) -> PyWMDownstreamState {
        let at = at.min(1.0).max(0.0);
        let lock_perc = self.lock_perc.0 + at * (self.lock_perc.1 - self.lock_perc.0);
        let lock_perc = if lock_perc < 0.0001 { 0.0 } else { lock_perc };
        PyWMDownstreamState { lock_perc }
    }
}

struct ViewDownstreamInterpolation<'a> {
    z_index: (usize, usize),
    box_: (f64, f64, f64, f64),
    mask: (f64, f64, f64, f64),
    corner_radius: (f64, f64),
    accepts_input: bool,
    size: (f64, f64),
    opacity: (f64, f64),
    lock_enabled: bool,
    workspace: Option<(f64, f64, f64, f64)>,
    floating: (f64, f64),
    fixed_output: (f64, f64),
    anim: bool,
    _size_adjustment: f64,
    layout: &'a Layout,
    state0: &'a PyWMViewDownstreamState,
    state1: &'a PyWMViewDownstreamState,
}

impl<'a> ViewDownstreamInterpolation<'a> {
    fn new(
        layout: &'a Layout,
        state0: &'a PyWMViewDownstreamState,
        state1: &'a PyWMViewDownstreamState,
    ) -> Self {
        let mut workspace = None;
        if let (Some(workspace0), Some(workspace1)) = (state0.workspace, state1.workspace) {
            let (mut x, mut y, mut w, mut h) = state0.workspace.unwrap();
            if workspace1.0 < x {
                w += x - workspace1.0;
                x = workspace1.0;
            }
            if workspace1.1 < y {
                h += y - workspace1.1;
                y = workspace1.1;
            }
            if workspace1.0 + workspace1.2 > x + w {
                w = workspace1.0 + workspace1.2 - x;
            }
            if workspace1.1 + workspace1.3 > y + h {
                h = workspace1.1 + workspace1.3 - y;
            }
            workspace = Some((x, y, w, h));
        }

        let mut anim = true;
        if let Some(workspace) = workspace {
            for ws in layout.workspaces {
                if ws.prevent_anim {
                    continue;
                }
                if ws.pos_x <= workspace.0
                    && workspace.0 + workspace.2 <= ws.pos_x + ws.width
                    && ws.pos_y <= workspace.1
                    && workspace.1 + workspace.3 <= ws.pos_y + ws.height
                {
                    anim = false;
                    break;
                }
            }
        }

        let mut size_adjustment = conf_size_adjustment();
        if let Some(workspace0) = state0.workspace {
            let (x, y, w, h) = state0.box;
            let (ws_x, ws_y, ws_w, ws_h) = workspace0;
            if x + w - 1. < ws_x
                || y + h - 1. < ws_y
                || ws_x + ws_w - 1. < x
                || ws_y + ws_h - 1. < y
            {
                size_adjustment = 0.0;
            }
        }
        if let Some(workspace1) = state1.workspace {
            let (x, y, w, h) = state1.box;
            let (ws_x, ws_y, ws_w, ws_h) = workspace1;
            if x + w - 1. < ws_x
                || y + h - 1. < ws_y
                || ws_x + ws_w - 1. < x
                || ws_y + ws_h - 1. < y
            {
                size_adjustment = 0.99;
            }
        }

        Self {
            z_index: (state0.z_index, state1.z_index),
            box_: (state0.box.0, state0.box.1, state0.box.2, state0.box.3),
            mask: (state0.mask.0, state0.mask.1, state0.mask.2, state0.mask.3),
            corner_radius: (state0.corner_radius, state1.corner_radius),
            accepts_input: state1.accepts_input,
            size: (state0.size, state1.size),
            opacity: (state0.opacity, state1.opacity),
            lock_enabled: state0.lock_enabled,
            workspace,
            floating: (state0.floating, state1.floating),
            fixed_output: (state0.fixed_output, state1.fixed_output),
            anim,
            _size_adjustment: size_adjustment,
            layout,
            state0,
            state1,
        }
    }
}

impl<'a> Interpolation<PyWMViewDownstreamState> for ViewDownstreamInterpolation<'a> {
    fn get(&self, at: f64) -> PyWMViewDownstreamState {
        let at = if !self.anim { 1.0 } else { at };
        let at = at.min(1.0).max(0.0);
        let box_ = (
            self.box_.0 + (self.box_.1 - self.box_.0) * at,
            self.box_.1 + (self.box_.2 - self.box_.1) * at,
            self.box_.2 + (self.box_.3 - self.box_.2) * at,
            self.box_.3 + (self.box_.3 - self.box_.2) * at,
        );
        let mask = (
            self.mask.0 + (self.mask.1 - self.mask.0) * at,
            self.mask.1 + (self.mask.2 - self.mask.1) * at,
            self.mask.2 + (self.mask.3 - self.mask.2) * at,
            self.mask.3 + (self.mask.3 - self.mask.2) * at,
        );
        let mut res = PyWMViewDownstreamState {
            z_index: if at > 0.5 {
                self.z_index.1
            } else {
                self.z_index.0
            },
            box_,
            mask,
            corner_radius: self.corner_radius.0 + at * (self.corner_radius.1 - self.corner_radius.0),
            accepts_input: self.accepts_input,
            ..Default::default()
        };
        res.opacity = self.opacity.0 + at * (self.opacity.1 - self.opacity.0);
        res.size = if at > self._size_adjustment {
            self.size.1
        } else {
            self.size.0
        };
        res.floating = if at > self._size_adjustment {
            self.floating.1
        } else {
            self.floating.0
        };
        res.lock_enabled = self.lock_enabled;
        res.workspace = self.workspace;
        res.fixed_output = if at > self._size_adjustment {
            self.fixed_output.1
        } else {
            self.fixed_output.0
        };
        res
    }
}
