//! AI Agent module for RAD IDE

//! AI Agent module for RAD IDE

use ollama_rs::Ollama;
use ollama_rs::generation::completion::request::GenerationRequest;

#[allow(dead_code)]
pub struct AiAgent {
    ollama: Ollama,
}

#[allow(dead_code)]
impl AiAgent {
    pub fn new() -> Self {
        Self {
            ollama: Ollama::default(),
        }
    }

    pub async fn ask(&self, prompt: &str) -> anyhow::Result<String> {
        let req = GenerationRequest::new("llama2".to_string(), prompt);
        let resp = self.ollama.generate(req).await?;
        Ok(resp.response)
    }
}
