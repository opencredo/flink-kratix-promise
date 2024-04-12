mod pipeline;
mod promise;
use log;

pub fn run_pipeline(
    _base_instance: &str,
    _res_dir: &str,
    _dep_dir: &str,
    _kratix_output_dir: &str,
    _kratix_input_dir: &str,
    _workflow_type: &str,
) {
    
    log::info!("<- Start Pipeline ({}) ->", _workflow_type);

    
    match _workflow_type {
        "promise" => {
            // Fullful promise.yaml
            if let Err(err) = 
                // tmp/transfer/dependecies -> /kratix/output
                pipeline::copy_files(_dep_dir, _kratix_output_dir) {
                log::warn!("Error during file copy: {}", err);
            }
        }
        _ => { 
            log::info!("  1. transform request");
            // Fullfil resource_request.yaml
            promise::transform(_base_instance,
                               _kratix_output_dir,
                               _kratix_input_dir); 
        }
    }

    
    pipeline::status();

    pipeline::list_files_recursively(_kratix_output_dir);

    log::info!("<- End Pipeline ->");
}
