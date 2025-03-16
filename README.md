# Zed Base64 Slash Command

This extension brings Base64 encoding and decoding capabilities to the [Zed Editor](https://zed.dev). It provides two slash commands to quickly transform text directly within the Assistant panel—one to encode text into Base64 and another to decode a Base64 string back into plain UTF‑8 text.

> **Note:** This extension is built in Rust and compiled to WebAssembly using the Zed Extension API and the [base64](https://docs.rs/base64) crate. It is intended for use in the Assistant panel, not the main editor.

## Features

- **Base64 Encoding:** Convert plain text into a Base64-encoded string using the `/encode` command.
- **Base64 Decoding:** Convert a Base64 string back to plain text with the `/decode` command.
- **Seamless Integration:** Inject transformed text directly into your LLM conversation in Zed.
- **Extensible:** Easily customize or expand the functionality to fit your workflow.

## Installation

### Prerequisites

- [Zed Editor](https://zed.dev) installed.
- [Rust](https://www.rust-lang.org/tools/install) installed via `rustup`.

### Steps

1. **Clone the Repository:**

   ```sh
   git clone https://github.com/your-username/zed-base64-slash-command.git
   cd zed-base64-slash-command
   ```

2. **Build the Extension:**

   Compile the extension in release mode:

   ```sh
   cargo build --release
   ```

   This generates the WebAssembly binary (typically `extension.wasm`) in your build output.

3. **Install in Zed:**

   Open Zed Editor and navigate to the Extensions page. Click **Install Dev Extension** and select the extension's directory. (If a published version exists, the dev extension will override it.)

## Usage

Open the Assistant panel and type:

- **`/encode <your text>`**
  Converts the provided text into a Base64 string.

- **`/decode <base64 string>`**
  Decodes the provided Base64 string back to plain text.

The extension will output the transformed text along with a label indicating the operation performed.

## How It Works

The extension registers two slash commands via the `extension.toml` file:

- **encode:** Joins the command arguments into a string and encodes it using the `base64::engine::general_purpose::STANDARD` engine.
- **decode:** Attempts to decode the input string from Base64 back to bytes and then converts those bytes into a UTF‑8 string.

Results are returned as a `SlashCommandOutput`, which includes both the text result and a section label for display in the Assistant panel.

# Releases

This repository uses a GitHub Action (configured in `release.yml`) based on the [zed-extension-action](https://github.com/huacnlee/zed-extension-action) to automate releases.

**How it works:**
- **Trigger:** When you push a new tag (e.g., `v1.2.3`), the action is triggered automatically.
- **Validation:** It first verifies that the tag follows a valid version format.
- **Pull Request:** The action then creates a pull request to update the extension version in the target repository.
- **Merge & Release:** Once the pull request is approved and merged, the new version of the extension is released.

**How to release:**
1. Create and push a new tag in the format `v*` (for example, `v1.2.3`).
2. The GitHub Action validates the tag, creates the pull request, and pushes the changes.
3. The target repository is the forked repo, [miguelmartens/extensions (Zed Extensions)](https://github.com/miguelmartens/extensions), which is the fork of the [zed-industries/extensions](https://github.com/zed-industries/extensions) repository.
4. After the PR is merged, the extension release process is completed automatically.

## License

This project is licensed under the MIT License.
