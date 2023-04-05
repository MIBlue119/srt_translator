use clap::Parser; // Use the clap crate to parse the arguments
/// Tell the compiler we are using the loader module
mod loader;
// use crate::loader::srt_loader;
mod openai;
mod prompter;
mod settings;
mod translator;
use crate::translator::Translator;

/// A simple program to translate a srt file
#[derive(Parser, Debug)]
#[command(author, version, about, long_about=None)]
struct Args {
    #[arg(short, long, default_value = "None")]
    file_path: String,
    #[arg(short, long, default_value = "gpt-3.5-turbo")]
    text_engine: String,
    #[arg(short, long, default_value = "japanese")]
    language: String,
    #[arg(short, long, default_value = "None")]
    output_path: String,
}
#[tokio::main]
async fn main() {
    // let args: Vec<String> = env::args().collect(); // Collect the arguments into a vector of strings
    // println!("{:?}", args); // Print the vector
    // println!("Hello, world!");
    let args = Args::parse();
    println!("{:?}", args);
    // According to the argument language, set the language

    let mut translator = Translator::new(args.language).unwrap();

    if args.file_path != "None" {
        // Load the srt file and print the lines
        // let lines = srt_loader.load_srt_file(args.file_path);
        let file_path = args.file_path.clone();
        translator.load_data(file_path);
        // println!("{:?}", translator.loaded_lines);
        //srt_loader.load_srt_file(args.file_path);
        // println!("{:?}", lines);
        // let test = translator
        //     .translate(translator.loaded_lines[2].clone())
        //     .await
        //     .unwrap();
        // print!("{:?}", test)
        // Extract the input file path from the arguments

        // Extract the output file path from the arguments
        if args.output_path != "None" {
            let output_path = &args.output_path;
            translator.translate_to_file(output_path).await.unwrap();
        } else {
            // Use the input filename and append _translated to it
            // Example: convert "test.srt" to "test_translated.srt"
            let output_path = args.file_path.replace(".srt", "_translated.srt");
            translator.translate_to_file(&output_path).await.unwrap();
        }
    } else {
        println!("No file path provided");
    }
}
