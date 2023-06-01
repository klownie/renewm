use renewm_core::renewm;
use std::env;
use std::path::{Path, PathBuf};

fn main() {
    let mut debug = false;
    let mut profile = false;
    let mut config_file: Option<PathBuf> = None;

    let args: Vec<String> = env::args().skip(1).collect();

    println!("renewm: LIFT OFF!");
    println!("renewm: start-renewm args received: {:?}", args);

    if args.is_empty() {
        println!("renewm: [WARN] no arguments provided ☜(ˆ▿ˆc) ");
    } else {
        let mut index = 0;
        while index < args.len() {
            match args[index].as_str() {
                "-d" | "--debug" => {
                    debug = true;
                }
                "-p" | "--profile" => {
                    profile = true;
                }
                "-c" | "--config" => {
                    index += 1;
                    if let Some(path) = args.get(index) {
                        let path_buf = PathBuf::from(path);
                        if path_buf.exists() {
                            config_file = Some(path_buf);
                        } else {
                            panic!("The specified config path does not exist: {}", path);
                        }
                    } else {
                        panic!("You forgot to provide a config path with --config");
                    }
                }
                _ => {}
            }
            index += 1;
        }
    }

    renewm::run::run(debug, profile, config_file);
}
