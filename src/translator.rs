use anyhow::Result;
use std::fs::File;
use std::io::{BufWriter, Write};
// import the loader module
use crate::loader::srt_loader;
use crate::openai::OpenAIClient;
use crate::prompter::TranslatorPrompter;
use crate::settings::OpenAISettings;

pub struct Translator {
    loader: srt_loader::SrtLoader,
    pub loaded_lines: Vec<String>,
    openai_client: OpenAIClient,
    prompter: TranslatorPrompter,
}

impl Translator {
    pub fn new(language: String) -> Result<Translator, anyhow::Error> {
        let settings = OpenAISettings::new();
        let language = language.to_lowercase();
        Ok(Translator {
            loader: srt_loader::SrtLoader::new(),
            loaded_lines: Vec::new(),
            openai_client: OpenAIClient::new(settings).unwrap(),
            prompter: TranslatorPrompter::new(language),
        })
    }
    // A method to load the srt file with loader and return a vector of strings at self.loaded_lines
    pub fn load_data(&mut self, file_path: String) {
        self.loaded_lines = self.loader.load_srt_file(file_path);
    }

    // A method to translate one line of text
    pub async fn translate_one_line(&self, one_line: String) -> Result<String, anyhow::Error> {
        let prompt = self.prompter.get_translation_prompt(&one_line);
        let translated = self.openai_client.completions(&prompt).await?;
        Ok(translated)
    }

    // A method to translate all the lines in self.loaded_lines
    pub async fn translate_to_file(&self, output_path: &String) -> Result<(), anyhow::Error> {
        let file = File::create(output_path)?;
        let mut writer = BufWriter::new(file);

        let mut last_line = String::new();
        // let translated_lines: Vec<String> = Vec::new();
        for line in &self.loaded_lines {
            if last_line.contains("-->") && !line.is_empty() {
                let translated_line = self.translate_one_line(line.clone()).await?;
                writeln!(&mut writer, "{}", translated_line)?;
            } else {
                last_line = line.to_owned();
                writeln!(&mut writer, "{}", line)?;
            }
        }
        writer.flush()?;
        Ok(())
    }
}
