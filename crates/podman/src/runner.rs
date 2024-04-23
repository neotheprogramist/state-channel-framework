use thiserror::Error;

#[derive(Error, Debug)]
pub enum RunnerError {
    #[error("unknown data store error")]
    Unknown,
}

trait Runner {
    async fn run(&self, input: &str) -> Result<String, RunnerError>;
}

pub struct PodmanRunner(String);

impl PodmanRunner {
    pub fn new(image_name: &str) -> Self {
        PodmanRunner(image_name.to_string())
    }
}

impl Runner for PodmanRunner {
    async fn run(&self, input: &str) -> Result<String, RunnerError> {
        Ok(String::new())
    }
}
