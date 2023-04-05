// Define the translation prompter

pub struct TranslatorPrompter {
    pub language: String,
}

impl TranslatorPrompter {
    pub fn new(language: String) -> TranslatorPrompter {
        TranslatorPrompter { language }
    }

    pub fn get_translation_prompt(&self, chunk: &String) -> String {
        // Create the prompt
        let chunk_prompt = format!(
            "Translate [{}] to lang:{}, Don't explain and only return translated text.",
            &chunk, self.language
        );
        // Return the prompt
        chunk_prompt
    }
}
