use std::fs::{self, File};
use std::io::{Read, Write};
use std::path::Path;
use yaml_rust::YamlLoader;
use std::{env, process};
use std::path::PathBuf;

fn main() {
    // Get environment variables
    let pipeline_name = env::var("PIPELINE_NAME").expect("$PIPELINE_NAME is not set");
    let pwd = env::current_dir().unwrap().parent().unwrap().to_path_buf();

    // Process command-line arguments
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: <command> [build, pipeline, load, push, rmi, pull]");
        process::exit(1);
    }

    // Handle commands
    match args[1].as_str() {
        "build" => build_image(&pwd, &pipeline_name),
        "pipeline" => run_pipeline(&pwd,&pipeline_name),
        _ => {
            eprintln!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}

// Implement the command handlers below
fn build_image(_pwd: &PathBuf, _pipeline_name: &str) { 
    // ... Use the 'process' module to execute Docker build commands
    eprintln!("Build docker image: {}", _pipeline_name);
}

// Implement the command handlers below
fn run_pipeline(_pwd: &PathBuf,_pipeline_name: &str) { 
    // ... Use the 'process' module to execute Docker build commands
    let workflow_type = env::var("KRATIX_WORKFLOW_TYPE");
    let input_yaml = "/kratix/input/object.yaml";

    // Read YAML data 
    // let mut input_file = File::open(input_yaml);
    // let mut input_contents = String::new();
    // input_file.read_to_string(&mut input_contents);

    // let yaml_docs = YamlLoader::load_from_str(&input_contents);
    // let doc = &yaml_docs[0];

    // let name = doc["metadata"]["name"].as_str().ok_or("Name not found in YAML");


    fs::copy("/tmp/transfer/*", "/kratix/output/");
    eprintln!("Run pipeline_name: {}", _pipeline_name);
}
