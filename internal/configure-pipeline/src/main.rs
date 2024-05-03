use kratix_utils::run_pipeline;

use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    init_logger();

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

fn init_logger() {
    let _ = env_logger::builder()
        // Include all events in tests
        .filter_level(log::LevelFilter::max())
        // Ensure events are captured by `cargo test`
        .is_test(false)
        // Ignore errors initializing the logger if tests race to configure it
        .try_init();
}
