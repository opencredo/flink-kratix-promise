mod pipeline;
mod promise;


pub fn run_pipeline(_base_instance: &str,_source_dir: &str,_destination_dir: &str,_workflow_type: &str) {
   
    //STEP 1. 
    if _workflow_type == "promise" { // Check if workflow_type is "promise"
        if let Err(err) = pipeline::copy_files(_source_dir, _destination_dir) {
            println!("Error during file copy: {}", err);
        }
    } 

     
    let yaml_doc = 
    pipeline::load_file(_base_instance).
        expect("Error loading YAML file"); //STEP 2.
    
    promise::transform(yaml_doc); //STEP 3. 
    
    //pipeline::status();

}

