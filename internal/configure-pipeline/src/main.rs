use kratix_utils::pipeline::PipelineConfig;
use kratix_utils::run_pipeline;
use kratix_utils::ResourceRequest;
use std::{env, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    let request = MyPromise {
        params: String::from("(default)"),
    };

    match args[1].as_str() {
        "pipeline" => {
            let _result = run_pipeline(Some(request));
        }
        _ => {
            log::warn!("Unknown command: {}", args[1]);
            process::exit(1);
        }
    }
}

#[derive(Clone)]
pub struct MyPromise {
    pub params: String,
}

impl ResourceRequest for MyPromise {
    fn transform(&self, _conf: &PipelineConfig) -> String {
        format!("{}", self.params)
    }
}
