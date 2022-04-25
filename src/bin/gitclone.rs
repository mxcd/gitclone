extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::process;
use simple_logger;
use log;
use log::{info};

use gitclone::action_init::do_init_action;
use gitclone::action_repository::clone;

fn main() {
    let mut verbose = false;
    let mut action = String::new();
    {
        let mut argument_parser = ArgumentParser::new();
        argument_parser.set_description("gitclone cli");
        argument_parser.refer(&mut verbose).add_option(&["-v", "--verbose"], StoreTrue, "Verbose output");
        argument_parser.refer(&mut action).add_argument("action", Store, "action or relative path to clone");
        argument_parser.parse_args_or_exit();
    }

    if verbose {
        simple_logger::init_with_level(log::Level::Debug).expect("Error initializing logger");
    }
    else {
        simple_logger::init_with_level(log::Level::Warn).expect("Error initializing logger");
    }

    if action == "" {
        println!("No action given. Pass '-h' for help.");
        process::exit(1);
    }
    match action.as_str() {
        "init" => {
            info!("Performing init action");
            do_init_action();
        },
        _ => {
            info!("Interpreting '{}' as relative repository path", action);
            clone(action);
        }
    }
}
