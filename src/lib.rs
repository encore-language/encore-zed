use std::env;
use std::path::PathBuf;

use zed_extension_api::{self as zed, Result};

struct EncoreExtension;

const ENCORE_LSP_BINARY: &str = "encore-lsp";
const LOCAL_LSP_BINARY: &str = "lsp";

impl zed::Extension for EncoreExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        if language_server_id.as_ref() != "encore-lsp" {
            return Err(format!(
                "unknown Encore language server: {language_server_id}"
            ));
        }

        if let Some(command) = env_lsp_path() {
            return Ok(command_with_args(command, Vec::new()));
        }

        if let Some(command) = local_checkout_lsp_path() {
            return Ok(command_with_args(command, Vec::new()));
        }

        if let Some(command) = installed_lsp_path() {
            return Ok(command_with_args(command, Vec::new()));
        }

        if let Some(command) = worktree.which(ENCORE_LSP_BINARY) {
            return Ok(command_with_args(command, Vec::new()));
        }

        Err(
            "Encore LSP launcher not found. Install it with \
             `encore install --path /path/to/encore/index/lsp --name encore-lsp --force --profile release` \
             or set `ENCORE_LSP_PATH=/absolute/path/to/encore/index/lsp/target/release/lsp` \
             and make sure `encore-lsp` is on PATH or present in ~/.encore/bin."
                .into(),
        )
    }
}

fn env_lsp_path() -> Option<String> {
    let path = env::var_os("ENCORE_LSP_PATH").map(PathBuf::from)?;
    path.is_file().then(|| path.to_string_lossy().into_owned())
}

fn installed_lsp_path() -> Option<String> {
    let install_root = env::var_os("ENCORE_INSTALL_ROOT")
        .map(PathBuf::from)
        .or_else(|| env::var_os("HOME").map(|home| PathBuf::from(home).join(".encore")))?;
    let binary_path = install_root.join("bin").join(ENCORE_LSP_BINARY);
    binary_path
        .is_file()
        .then(|| binary_path.to_string_lossy().into_owned())
}

fn local_checkout_lsp_path() -> Option<String> {
    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let candidates = [
        manifest_dir.parent().map(|dir| {
            dir.join("encore")
                .join("index")
                .join("lsp")
                .join("target")
                .join("release")
                .join(LOCAL_LSP_BINARY)
        }),
        manifest_dir.parent().map(|dir| {
            dir.join("encore")
                .join("index")
                .join("lsp")
                .join("target")
                .join("debug")
                .join(LOCAL_LSP_BINARY)
        }),
    ];

    candidates
        .into_iter()
        .flatten()
        .find(|path| path.is_file())
        .map(|path| path.to_string_lossy().into_owned())
}

fn command_with_args(command: String, args: Vec<String>) -> zed::Command {
    zed::Command {
        command,
        args,
        env: Default::default(),
    }
}

zed::register_extension!(EncoreExtension);
