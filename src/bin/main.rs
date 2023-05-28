use renewm_core::renewm;
use std::env;

fn main() {
    let mut no_args = false;
    let mut debug = false;
    let mut profile = false;
    let mut config_file: Option<String> = None;

    let mut args: Vec<String> = env::args().skip(1).collect();

    println!("renewm: LIFT OFF!");
    println!("renewm: start-renewm args received: {:?}", args);

    if args.len() == 1 {
        println!("renewm: [WARN] no arguments provided ☜(ˆ▿ˆc) ");
        no_args = true;
    }

    if !no_args {
        if args.contains(&String::from("-d")) || args.contains(&String::from("--debug")) {
            debug = true;
            args.iter()
                .position(|x| x == "-d" || x == "--debug")
                .map(|index| args.remove(index));
        }

        if args.contains(&String::from("-p")) || args.contains(&String::from("--profile")) {
            profile = true;
            args.iter()
                .position(|x| x == "-p" || x == "--profile")
                .map(|index| args.remove(index));
        }

        if args.contains(&String::from("-c")) || args.contains(&String::from("--config")) {
            let index = args.iter().position(|x| x == "-c" || x == "--config");
            if let Some(index) = index {
                args.remove(index);
                match args[0] {
                    None => {
                        panic!("You forgot to provide a config path with --config ( ˘︹˘ )");
                    }
                    Some(path) => {
                        config_file = Some(path.clone());
                    }
                }
            }
        }
    }

    renewm::run::run(debug, profile, config_file.as_deref());
}
