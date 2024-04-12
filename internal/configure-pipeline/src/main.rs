use std::{env, process};
use log;
use dotenv::dotenv;
use kratix_utils;


// Structure to hold potential errors
#[derive(Debug)]
struct EnvVarError {
    var_name: String,
}

// validation function
fn validate_env_vars() -> Result<(), Vec<EnvVarError>> {
    let required_vars = vec![
        "KRATIX_WORKFLOW_TYPE",
        "BASE_INSTANCE",
        "DEPENDENCIES_DIR",
        "RESOURCES_DIR",
        "KRATIX_INPUT",
        "KRATIX_OUTPUT",
    ];

    let mut errors = Vec::new();

    for var_name in required_vars {
        if env::var(var_name).is_err() {
            errors.push(EnvVarError {
                var_name: var_name.to_string(),
            });
        }
    }

    if errors.is_empty() {
        Ok(())
    } else {
        Err(errors)
    }
}


fn main() {
    dotenv().ok();
    // Validate environment variables up front
    match validate_env_vars() {
        Ok(()) => (), // Everything is good, proceed
        Err(errors) => {
            eprintln!("Error: Missing environment variables:");
            for error in errors {
                log::warn!(" - {}", error.var_name);
            }
            process::exit(1);
        }
    }

    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        log::warn!("Usage: <command> [build, pipeline, load, push, rmi, pull]");
        process::exit(1);
    }
    
    // Extract validated environment variables
    let workflow_type = env::var("KRATIX_WORKFLOW_TYPE").unwrap();
    let base_instance = env::var("BASE_INSTANCE").unwrap();
    let dep_dir = env::var("DEPENDENCIES_DIR").unwrap();
    let res_dir = env::var("RESOURCES_DIR").unwrap();
    let kratix_input_dir = env::var("KRATIX_INPUT").unwrap();
    let kratix_output_dir = env::var("KRATIX_OUTPUT").unwrap();

    // TODO hhill: pass function to do transform
    match args[1].as_str() {
        "pipeline" => kratix_utils::run_pipeline(
            &base_instance,
            &res_dir,
            &dep_dir,
            &kratix_output_dir,
            &kratix_input_dir,
            &workflow_type),
        _ => {
            log::warn!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}

