use std::{env, process};
mod kratix;

fn main() {
    
    
    println!("UID: {}", std::process::id());
    println!("GID: {}", std::env::var("GID").unwrap_or_default()); // Assuming you might have a GID environment variable
    // Process command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <command> [build, pipeline, load, push, rmi, pull]");
        process::exit(1);
    }

    let workflow_type = std::env::var("KRATIX_WORKFLOW_TYPE").unwrap_or_else(|_| {
        eprintln!("Error: KRATIX_WORKFLOW_TYPE environment variable not set.");
        process::exit(1);
    });

    let base_instance = std::env::var("BASE_INSTANCE").unwrap_or_else(|_| {
        eprintln!("Error: SOURCE_DIR environment variable not set.");
        process::exit(1);
    });
    // Get source and destination from environment variables
    let source_dir = std::env::var("KRATIX_INPUT").unwrap_or_else(|_| {
        eprintln!("Error: SOURCE_DIR environment variable not set.");
        process::exit(1);
    });

    let destination_dir = std::env::var("KRATIX_OUTPUT").unwrap_or_else(|_| {
        eprintln!("Error: DESTINATION_DIR environment variable not set.");
        process::exit(1);
    });

    // Handle commands
    match args[1].as_str() {
        "pipeline" => kratix::run_pipeline(&base_instance,&source_dir,&destination_dir,&workflow_type),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}

