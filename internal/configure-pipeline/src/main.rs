
use kratix_utils::run_pipeline;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    match args[1].as_str() {
        "pipeline" => {
            let _result = run_pipeline();
        }
        _ => {
            log::warn!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}
