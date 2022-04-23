extern crate argparse;

use argparse::{ArgumentParser, StoreTrue, Store};
use std::process;
use gitclone::action_init::do_init_action;

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

    if action == "" {
        println!("No action given. Pass '-h' for help.");
        process::exit(1);
    }
    match action.as_str() {
        "init" => {
            if verbose { println!("Performing init action")}
            do_init_action(verbose);
        },
        _ => {
            if verbose { println!("Interpreting '{}' as relative repository path", action)}
        }
    }
}
