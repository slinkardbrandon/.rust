mod commands;
mod config;

use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    match args.get(1) {
        Some(v) => {
            match v.as_ref() {
                "sync" => commands::sync::exec(),
                "update" => commands::update::exec(),
                _ => commands::default::exec(),
            }
        }
        None => commands::default::exec(),
    }
}
