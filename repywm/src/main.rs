mod wm_config {
    // wm_config.c

    pub struct WmConfig {
        //
    }

    impl WmConfig {
        pub fn default() -> Self {
            //
        }
    }

    impl WmConfig {
        pub fn init_default(&mut self) {
            //
        }
    }
}

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

fn main() {
    let mut config = wm_config::WmConfig::default();
    config.init_default();

    let wm = wm::Wm::new(config);
    wm.run();
    wm.destroy();
}
