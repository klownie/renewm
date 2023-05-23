mod wm {
    // wm.c
    use super::wm_config::WmConfig;

    pub struct Wm {
        config: WmConfig,
    }

    impl Wm {
        pub fn new(config: WmConfig) -> Self {
            //
            Self { config }
        }

        pub fn run(&self) {
            //
        }

        pub fn destroy(&self) {
            //
        }
    }
}

mod wm_config {
    // wm_config.c

    use std::env;

    pub struct wm_config_output {
        name: String,
        scale: f64,
        width: i32,
        height: i32,
        mHz: i32,
        pos_x: i32,
        pos_y: i32,
        transform: wl_output_transform,
        link: wl_list_link,
    }
    pub struct WmConfig {
        //
        enable_xwayland: bool,
        callback_frequency: u32,
        xkb_model: String,
        xkb_layout: String,
        xkb_variant: String,
        xkb_options: String,
        texture_shaders: String,
        outputs: wl_list,
        xcursor_theme: Option<String>,
        xcursor_size: u32,
        natural_scroll: bool,
        tap_to_click: bool,
        focus_follows_mouse: bool,
        constrain_popups_to_toplevel: bool,
        encourage_csd: bool,
        debug: bool,
    }

    impl WmConfig {
        pub fn default() -> Self {
            //
            let mut config = WmConfig {
                enable_xwayland: false,
                callback_frequency: 10,
                xkb_model: String::new(),
                xkb_layout: String::new(),
                xkb_variant: String::new(),
                xkb_options: String::new(),
                texture_shaders: "basic".to_string(),
                outputs: wl_list::new(),
                xcursor_theme: env::var("XCURSOR_THEME").ok(),
                xcursor_size: env::var("XCURSOR_SIZE")
                    .ok()
                    .and_then(|env_size| env_size.parse::<u32>().ok())
                    .unwrap_or(24),
                natural_scroll: true,
                tap_to_click: true,
                focus_follows_mouse: true,
                constrain_popups_to_toplevel: false,
                encourage_csd: true,
                debug: false,
            };

            config
        }

        pub fn wm_config_reset_default(&mut self) {
            while let Some(output) = self.outputs.pop_front() {
                let _ = unsafe { Box::from_raw(output) };
            }
            super::wm_config::WmConfig::default();
        }

        pub fn wm_config_reconfigure(config: &mut self, server: &mut WmServer) {
            wm_seat_reconfigure(&server.wm_seat);
            wm_layout_reconfigure(&server.wm_layout);
            wm_server_reconfigure(server);

            xcursor_setenv(config);
            wm_renderer_select_texture_shaders(server.wm_renderer, &config.texture_shaders);
            wm_renderer_ensure_mode(server.wm_renderer, wm_config_get_renderer_mode(config));
        }

        fn xcursor_setenv(&self) {
            let cursor_size_fmt = format!("{}", self.xcursor_size);
            let cursor_size_fmt_c = CString::new(cursor_size_fmt).unwrap();
            unsafe {
                libc::setenv("XCURSOR_SIZE", cursor_size_fmt_c.as_ptr(), 1);
            }
            if let Some(xcursor_theme) = &self.xcursor_theme {
                let xcursor_theme_c = CString::new(xcursor_theme.as_str()).unwrap();
                unsafe {
                    libc::setenv("XCURSOR_THEME", xcursor_theme_c.as_ptr(), 1);
                }
            }
        }

        fn wm_config_get_renderer_mode(&self) -> wm_renderer_mode {
            if self.renderer_mode == "wlr" {
                WM_RENDERER_WLR
            } else if self.renderer_mode == "pywm" {
                WM_RENDERER_PYWM
            } else {
                WM_RENDERER_PYWM
            }
        }

        fn wm_config_set_xcursor_theme(&mut self, xcursor_theme: &str) {
            self.xcursor_theme = Some(xcursor_theme.to_string());
            self.xcursor_setenv();
        }

        fn wm_config_set_xcursor_size(&mut self, xcursor_size: u32) {
            self.xcursor_size = xcursor_size;
            self.xcursor_setenv();
        }

        fn wm_config_add_output(
            &mut self,
            name: &str,
            scale: f64,
            width: i32,
            height: i32,
            mHz: i32,
            pos_x: i32,
            pos_y: i32,
            transform: wl_output_transform,
        ) {
            if name.is_empty() {
                wlr_log(WLR_ERROR, "Cannot add output config without name");
                return;
            }

            let new = wm_config_output {
                name: name.to_string(),
                scale,
                width,
                height,
                mHz,
                pos_x,
                pos_y,
                transform,
                link: wl_list_link,
            };

            self.outputs.push(&new.link);
        }

        fn wm_config_find_output(&self, name: &str) -> Option<&wm_config_output> {
            if name.is_empty() {
                return None;
            }

            for output in self.outputs.iter() {
                if output.name == name {
                    return Some(output);
                }
            }

            None
        }

        fn wm_config_destroy(&mut self) {
            while let Some(output) = self.outputs.pop_front() {
                let _ = unsafe { Box::from_raw(output) };
            }
        }
    }
}

// main.c
fn main() {
    let mut config = wm_config::WmConfig::default();

    let wm = wm::Wm::new(config);
    wm.run();
    wm.destroy();
}
