use kratix_utils::pipeline::load_file;
use kratix_utils::pipeline::PipelineConfig;
use kratix_utils::ResourceRequest;
use serde_yaml::{to_string, Value};
use std::fs::read_to_string;
use std::path::Path;

#[derive(Clone)]
pub struct MyPromise {
    pub params: String,
}

impl ResourceRequest for MyPromise {
    fn transform(&self, _conf: &PipelineConfig) -> String {
        let new_kin_path = format!("{}/object.yaml", _conf.kratix_input_dir());

        format!("{} modify {:?}", self.params, new_kin_path)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_request() -> Result<(), String> {
        let request = MyPromise {
            params: String::from("(custom)"),
        };

        let _result = kratix_utils::run_pipeline(Some(request));

        let new_kout_path = format!("{}/flink-instance.yaml", _result.kratix_output_dir());
        assert_eq!(Path::new(&new_kout_path).exists(), true);
        Ok(())
    }
}
