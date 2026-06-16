use zed_extension_api::{self as zed, Result};

struct EncoreExtension;

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
            return Err(format!("unknown Encore language server: {language_server_id}"));
        }

        if let Some(command) = worktree.which("encore-lsp") {
            return Ok(command_with_args(command, Vec::new()));
        }

        if let Some(command) = worktree.which("encore_lsp") {
            return Ok(command_with_args(command, Vec::new()));
        }

        Err(
            "Encore LSP not found. Install it with \
             `encore install --path <encore>/lsp --name encore-lsp --force` or put \
             `encore-lsp` on PATH."
                .into(),
        )
    }
}

fn command_with_args(command: String, args: Vec<String>) -> zed::Command {
    zed::Command {
        command,
        args,
        env: Default::default(),
    }
}

zed::register_extension!(EncoreExtension);
