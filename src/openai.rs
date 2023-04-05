use crate::settings::OpenAISettings;
use anyhow::{anyhow, bail, Ok, Result};
use async_openai::{
    types::{
        ChatCompletionRequestMessageArgs, CreateChatCompletionRequestArgs,
        CreateCompletionRequestArgs, Role,
    },
    Client,
};
use tiktoken_rs::{get_chat_completion_max_tokens, get_completion_max_tokens};

/// ref: https://github.dev/zurawiki/gptcommit
#[derive(Clone, Debug)]
pub struct OpenAIClient {
    text_engine: String,
    client: Client,
}

impl OpenAIClient {
    pub fn new(settings: OpenAISettings) -> Result<Self, anyhow::Error> {
        let api_key = settings.api_key.unwrap();
        if api_key.is_empty() {
            bail!("OPENAI_API_KEY is not set, please set it in your environment variables.");
        }
        let mut openai_client = Client::new().with_api_key(&api_key);
        Ok(Self {
            text_engine: settings.text_engine.unwrap(),
            client: openai_client,
        })
    }
    pub fn should_use_chat_completion(&self, text_engine: &str) -> bool {
        //`&str`: is a reference to a string. It is used to pass a string to a function without taking ownership of it.
        text_engine.to_lowercase().starts_with("gpt-4")
            || text_engine.to_lowercase().starts_with("gpt-3.5-turbo")
    }

    pub async fn get_chat_completions(&self, prompt: &str) -> Result<String> {
        let messages = [ChatCompletionRequestMessageArgs::default()
            .role(Role::User)
            .content(prompt)
            .build()?];
        let request = CreateChatCompletionRequestArgs::default()
            .model(&self.text_engine)
            .messages(messages)
            .build()?;
        let response = self.client.chat().create(request).await?;
        if let Some(choice) = response.choices.into_iter().next() {
            // debug!(
            //     "{}: Role: {} Content: {}",
            //     choice.index, choice.message.role, choice.message.content
            // );
            return Ok(choice.message.content);
        }
        bail!("No completion results returned from OpenAI.")
    }

    pub async fn get_completions(&self, prompt: &str) -> Result<String> {
        let prompt_token_limit = get_completion_max_tokens(&self.text_engine, prompt)?;
        // Create request using builder pattern
        let request = CreateCompletionRequestArgs::default()
            .model(&self.text_engine)
            .prompt(prompt)
            .max_tokens(prompt_token_limit as u16)
            .temperature(0.5)
            .top_p(1.)
            .frequency_penalty(0.)
            .presence_penalty(0.)
            .build()?;

        // debug!("Sending request to OpenAI:\n{:?}", request);

        let response = self
            .client
            .completions() // Get the API "group" (completions, images, etc.) from the client
            .create(request) // Make the API call in that "group"
            .await?;

        let completion = response
            .choices
            .first()
            .ok_or(anyhow!("No completion results returned from OpenAI."))
            .map(|c| c.text.clone());

        completion
    }

    pub async fn completions(&self, prompt: &str) -> Result<String, anyhow::Error> {
        let completion = if self.should_use_chat_completion(&self.text_engine) {
            self.get_chat_completions(prompt).await?
        } else {
            self.get_completions(prompt).await?
        };
        Ok(completion.trim().to_string())
    }
}
