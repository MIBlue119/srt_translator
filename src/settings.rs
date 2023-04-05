use async_openai::API_BASE;
/// `static` indicate that constant is stored in the binary
static DEFAULT_TEXT_ENGINE: &str = "gpt-3.5-turbo";

pub struct OpenAISettings {
    pub api_base: Option<String>,
    pub api_key: Option<String>,
    pub text_engine: Option<String>,
    pub retries: Option<u16>,
}

impl OpenAISettings {
    pub fn new() -> OpenAISettings {
        let mut openai_api_key = String::new();
        if let Ok(api_key) = std::env::var("OPENAI_API_KEY") {
            if !api_key.is_empty() {
                print!("OPENAI_API_KEY is set, using it to authenticate with OpenAI.\n");
                openai_api_key = api_key;
            }
        }
        OpenAISettings {
            api_base: Some(API_BASE.to_string()),
            api_key: Some(openai_api_key),
            text_engine: Some(DEFAULT_TEXT_ENGINE.to_string()),
            retries: None,
        }
    }
}
