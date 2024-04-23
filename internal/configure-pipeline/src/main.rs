use std::{env,process};
use log::{LevelFilter};
use log4rs::append::console::ConsoleAppender;
use log4rs::config::{Appender, Root};
use log4rs::Config;



fn main() {
    

    let stdout = ConsoleAppender::builder().build();    
    let config = Config::builder()
        .appender(Appender::builder().build("stdout", Box::new(stdout)))
        .build(Root::builder().appender("stdout").build(LevelFilter::Trace))
        .unwrap(); 
       
    let _handle = log4rs::init_config(config).unwrap();

    let args: Vec<String> = env::args().collect();

    // TODO: pass function to do IOC transform
    match args[1].as_str() {
        "pipeline" => kratix_utils::run_pipeline(args),
        _ => {
            log::warn!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}

