use fanshim_lib::config::FanshimConfig;
use log::{error, info};
use simple_signal::Signal;
use simplelog::{CombinedLogger, Config, LevelFilter, TermLogger, TerminalMode};
use std::env;
use std::path::Path;
use std::process::exit;

const SUCCESSFUL_EXIT: i32 = 0;
const UNKNOWN_ARGUMENT: i32 = 1;

const VERSION: &str = env!("CARGO_PKG_VERSION");
const AUTHOR: &str = env!("CARGO_PKG_AUTHORS");
const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
const CONFIG_FILE_LOCATION: &str = "/etc/fanshim-rs.toml";

fn main() -> fanshim_lib::Result<()> {
    CombinedLogger::init(vec![TermLogger::new(
        LevelFilter::Info,
        Config::default(),
        TerminalMode::Mixed,
    )])?;

    handle_arguments();

    let config = {
        match fanshim_lib::config::read_config_file(Path::new(CONFIG_FILE_LOCATION)) {
            Ok(c) => c,
            Err(_) => {
                error!("Error parsing config file. Using defaults.");
                FanshimConfig::default()
            }
        }
    };
    info!("Loaded with config: {:#?}", config);

    let mut s = fanshim_lib::RealShim::new();

    simple_signal::set_handler(&[Signal::Int, Signal::Term], |signals| {
        fanshim_lib::led::set_led_rgb_brightness(0, 0, 0, 0.0);
        fanshim_lib::fan::turn_fan_off();
        if signals.contains(&Signal::Int) {
            info!("Received interrupt signal. Exiting.");
        } else if signals.contains(&Signal::Term) {
            info!("Received termination signal. Exiting.");
        }
        exit(SUCCESSFUL_EXIT);
    });

    fanshim_lib::initial_setup(&config, &mut s);
    fanshim_lib::main_loop(&config, &mut s);
    Ok(())
}

fn handle_arguments() {
    let args: Vec<String> = std::env::args().collect();

    // Length of 1 means only one argument is passed; the name of the binary.
    if args.len() == 1 {
        return;
    }

    let show_version = args.contains(&"--version".to_owned()) || args.contains(&"-v".to_owned());
    if show_version {
        println!("{}", VERSION);

        exit(SUCCESSFUL_EXIT);
    }

    let show_help = args.contains(&"--help".to_owned()) || args.contains(&"-h".to_owned());
    if show_help {
        println!("Pimoroni Fanshim Rust Driver {}", VERSION);
        println!("{}", AUTHOR);
        println!("Controls the Pimoroni Fanshim Fan and LED.");
        println!();
        println!(
            "The configuration file is located at '{}'.",
            CONFIG_FILE_LOCATION
        );
        println!(
            "For more information see the man page at 'man {}'",
            PACKAGE_NAME
        );
        println!();
        println!("USAGE:");
        println!("\t{} [FLAGS]", PACKAGE_NAME);
        println!();
        println!("FLAGS:");
        println!("\t-h, --help\tPrints this message");
        println!("\t-v, --version\tPrints version information");
        println!();

        exit(SUCCESSFUL_EXIT);
    }

    println!("Unknown arguments passed to binary:");
    // We can slice the args because we checked for a length of 1 above.
    for i in &args[1..] {
        println!("\t'{}'", i);
    }
    println!();
    println!("For help use '--help' or '-h'.");

    exit(UNKNOWN_ARGUMENT);
}
