## SRT Translator

## Usage

- build and run the code with example srt
  ```
  cargo run --release -- -f ./examples/small.srt -l German
  ```

## Convert a .srt/ .vtt to desired language

- Steps
  - Argument parser support
  - File loader
  - OpenAI API calling
  - Chunker
  - Prompt loader
