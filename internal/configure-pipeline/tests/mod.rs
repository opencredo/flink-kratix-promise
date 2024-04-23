use std::{env};


fn sqrt(number: f64) -> Result<f64, String> {
    if number >= 0.0 {
        Ok(number.powf(0.5))
    } else {
        Err("negative floats don't have square roots".to_owned())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_resource_request() -> Result<(), String> {
        let x = 4.0;
        let args: Vec<String> = env::args().collect();
        //kratix_utils::run_pipeline(args);
        assert_eq!(sqrt(x)?.powf(2.0), x);
        Ok(())
    }
}