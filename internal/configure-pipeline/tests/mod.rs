use kratix_utils::pipeline::PipelineConfig;
use kratix_utils::ResourceRequest;
use std::{env, fs, path::Path};

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

    const WORKSPACE: &str = "WORKSPACE";
    const KRATIX_WORKFLOW_TYPE: &str = "KRATIX_WORKFLOW_TYPE";

    #[test]
    fn test_promise_request() -> Result<(), String> {
        init_logger();

        let current_dir = env::current_dir().unwrap();
        env::set_var(WORKSPACE, current_dir);
        env::set_var(KRATIX_WORKFLOW_TYPE, "promise");

        let request = MyPromise {
            params: String::from("(custom)"),
        };

        let _result = kratix_utils::run_custom_pipeline(Some(request));

        let new_kout_path = format!("{}/cluster-operator.yml", _result.kratix_output_dir());
        assert_eq!(Path::new(&new_kout_path).exists(), true);

        let _ = teardown(_result.kratix_output_dir());

        Ok(())
    }

    #[test]
    fn test_resource_request() -> Result<(), String> {
        init_logger();

        let current_dir = env::current_dir().unwrap();
        env::set_var(WORKSPACE, current_dir);
        env::set_var(KRATIX_WORKFLOW_TYPE, "resource");

        let request = MyPromise {
            params: String::from("(custom)"),
        };

        let _result = kratix_utils::run_custom_pipeline(Some(request));

        let new_kout_path = format!("{}/flink-instance.yaml", _result.kratix_output_dir());
        assert_eq!(Path::new(&new_kout_path).exists(), true);

        let _ = teardown(_result.kratix_output_dir());

        Ok(())
    }

    fn init_logger() {
        let _ = env_logger::builder()
            // Include all events in tests
            .filter_level(log::LevelFilter::max())
            // Ensure events are captured by `cargo test`
            .is_test(true)
            // Ignore errors initializing the logger if tests race to configure it
            .try_init();
    }

    fn teardown(path: &str) {
        fs::remove_dir_all(path).unwrap();
        fs::create_dir(path).unwrap();
        env::remove_var(KRATIX_WORKFLOW_TYPE);
    }
}
