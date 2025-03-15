use zed_extension_api::{
    self as zed,
    SlashCommand,
    SlashCommandArgumentCompletion,
    SlashCommandOutput,
    SlashCommandOutputSection,
    Worktree,
};
use base64::Engine; // ensure the Engine trait is in scope

/// A Zed extension that provides two slash commands for Base64 encoding and decoding.
struct Base64SlashCommandExtension;

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
                // Decode Base64 input back to UTF-8 text.
                let decoded_bytes = base64::engine::general_purpose::STANDARD
                    .decode(&input)
                    .map_err(|e| format!("Decoding error: {}", e))?;
                let decoded = String::from_utf8(decoded_bytes)
                    .map_err(|e| format!("UTF-8 conversion error: {}", e))?;
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
