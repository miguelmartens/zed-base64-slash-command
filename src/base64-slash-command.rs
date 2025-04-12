use base64::Engine;
use zed_extension_api::{
    self as zed, SlashCommand, SlashCommandArgumentCompletion, SlashCommandOutput,
    SlashCommandOutputSection, Worktree,
}; // ensure the Engine trait is in scope

/// A Zed extension that provides two slash commands for Base64 encoding and decoding.
struct Base64SlashCommandExtension;

// Helper function to check if a string is likely Base64-encoded
fn is_likely_base64(input: &str) -> bool {
    // Base64 length should be a multiple of 4 (or have appropriate padding)
    let valid_length = input.len() % 4 == 0 || (input.len() % 4 > 0 && input.ends_with('='));

    // Base64 should only contain A-Z, a-z, 0-9, +, /, and = for padding
    let valid_chars = input
        .chars()
        .all(|c| c.is_ascii_alphanumeric() || c == '+' || c == '/' || c == '=');

    valid_length && valid_chars
}

impl zed::Extension for Base64SlashCommandExtension {
    fn new() -> Self {
        Base64SlashCommandExtension
    }

    fn complete_slash_command_argument(
        &self,
        _command: SlashCommand,
        _args: Vec<String>,
    ) -> Result<Vec<SlashCommandArgumentCompletion>, String> {
        // No auto-completion for these commands.
        Ok(vec![])
    }

    fn run_slash_command(
        &self,
        command: SlashCommand,
        args: Vec<String>,
        _worktree: Option<&Worktree>,
    ) -> Result<SlashCommandOutput, String> {
        // Ensure arguments were provided.
        if args.is_empty() {
            return Err("No input provided".to_string());
        }
        let input = args.join(" ");
        match command.name.as_str() {
            "encode" => {
                // Encode input to Base64.
                let encoded = base64::engine::general_purpose::STANDARD.encode(input.as_bytes());
                Ok(SlashCommandOutput {
                    text: encoded.clone(),
                    sections: vec![SlashCommandOutputSection {
                        range: (0..encoded.len()).into(),
                        label: "Encoded Output".into(),
                    }],
                })
            }
            "decode" => {
                // Validate the input looks like Base64
                if !is_likely_base64(&input) {
                    return Err(
                        "The input doesn't appear to be Base64-encoded. Use /encode to encode text.".to_string()
                    );
                }

                // Decode Base64 input back to UTF-8 text.
                let decoded_bytes = match base64::engine::general_purpose::STANDARD.decode(&input) {
                    Ok(bytes) => bytes,
                    Err(e) => {
                        return Err(format!(
                            "Decoding error: {}. Please ensure your input is valid Base64.",
                            e
                        ));
                    }
                };

                let decoded = match String::from_utf8(decoded_bytes) {
                    Ok(text) => text,
                    Err(_) => return Err("The decoded data is not valid UTF-8 text.".into()),
                };

                Ok(SlashCommandOutput {
                    text: decoded.clone(),
                    sections: vec![SlashCommandOutputSection {
                        range: (0..decoded.len()).into(),
                        label: "Decoded Output".into(),
                    }],
                })
            }
            _ => Err(format!("Unknown slash command: {}", command.name)),
        }
    }
}

zed::register_extension!(Base64SlashCommandExtension);
