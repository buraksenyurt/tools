use std::env;
use sysinfo::System;

use crate::{models::SystemData, view::Display};

mod models;
mod utility;
mod view;

fn main() {
    utility::clear_screen();

    let args: Vec<String> = env::args().collect();
    if args.len() > 1 && args[1] == "--help" {
        utility::show_help();
        return;
    }

    let mut sys = System::new_all();
    sys.refresh_all();

    utility::show_loading_message();

    let system_data = SystemData::new(&mut sys);
    utility::clear_screen();
    system_data.display_all();
}
