use std::fs;

use zed_extension_api::{
    self as zed, settings::LspSettings, LanguageServerInstallationStatus as InstallationStatus,
    Result,
};

struct EncoreBinary {
    path: String,
    args: Vec<String>,
    env: zed::EnvVars,
}

struct EncoreExtension {
    cached_binary_path: Option<String>,
}

const ENCORE_LSP_BINARY: &str = "encore-lsp";
const ENCORE_RELEASE_REPOSITORY: &str = "encore-ecosystem/encore";

impl EncoreExtension {
    fn language_server_binary(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<EncoreBinary> {
        let settings =
            LspSettings::for_worktree(language_server_id.as_ref(), worktree).unwrap_or_default();
        let binary_settings = settings.binary;
        let mut command_env = worktree.shell_env();
        if let Some(overrides) = binary_settings
            .as_ref()
            .and_then(|binary| binary.env.clone())
        {
            for (name, value) in overrides {
                command_env.retain(|(existing, _)| existing != &name);
                command_env.push((name, value));
            }
        }
        let args = binary_settings
            .as_ref()
            .and_then(|binary| binary.arguments.clone())
            .unwrap_or_default();
        let configured_path = binary_settings.and_then(|binary| binary.path);
        let environment_path = environment_value(&command_env, "ENCORE_LSP_PATH")
            .filter(|path| fs::metadata(path).is_ok_and(|metadata| metadata.is_file()));
        let path = configured_path
            .or(environment_path)
            .or_else(|| worktree.which(ENCORE_LSP_BINARY));
        if let Some(path) = path {
            return Ok(EncoreBinary {
                path,
                args,
                env: command_env,
            });
        }

        match self.zed_managed_binary_path(language_server_id) {
            Ok(path) => Ok(EncoreBinary {
                path,
                args,
                env: command_env,
            }),
            Err(error) => {
                zed::set_language_server_installation_status(
                    language_server_id,
                    &InstallationStatus::Failed(error.clone()),
                );
                Err(error)
            }
        }
    }

    fn zed_managed_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
    ) -> Result<String> {
        if let Some(path) = &self.cached_binary_path {
            if fs::metadata(path).is_ok_and(|metadata| metadata.is_file()) {
                return Ok(path.clone());
            }
        }

        zed::set_language_server_installation_status(
            language_server_id,
            &InstallationStatus::CheckingForUpdate,
        );
        let release = zed::latest_github_release(
            ENCORE_RELEASE_REPOSITORY,
            zed::GithubReleaseOptions {
                require_assets: true,
                pre_release: false,
            },
        )?;
        let (os, architecture) = zed::current_platform();
        let arch = match architecture {
            zed::Architecture::Aarch64 => "aarch64",
            zed::Architecture::X8664 => "x86_64",
            zed::Architecture::X86 => {
                return Err("Encore does not publish a 32-bit Zed language server".into())
            }
        };
        let triple = match (os, architecture) {
            (zed::Os::Linux, _) => format!("{arch}-unknown-linux-gnu"),
            (zed::Os::Mac, _) => format!("{arch}-apple-darwin"),
            (zed::Os::Windows, zed::Architecture::X8664) => "x86_64-pc-windows-msvc".to_string(),
            (zed::Os::Windows, zed::Architecture::Aarch64) => "aarch64-w64-windows-gnu".to_string(),
            (zed::Os::Windows, zed::Architecture::X86) => unreachable!(),
        };
        let version = release.version.trim_start_matches('v');
        let archive_suffix = if os == zed::Os::Windows {
            "zip"
        } else {
            "tar.gz"
        };
        let package_name = format!("encore-{version}-{triple}");
        let asset_name = format!("{package_name}.{archive_suffix}");
        let asset = release
            .assets
            .iter()
            .find(|asset| asset.name == asset_name)
            .ok_or_else(|| format!("Encore release {} has no {asset_name}", release.version))?;
        let version_dir = format!("encore-lsp-{}-{triple}", release.version);
        let executable = if os == zed::Os::Windows {
            "encore-lsp.exe"
        } else {
            ENCORE_LSP_BINARY
        };
        let binary_path = format!("{version_dir}/{package_name}/bin/{executable}");

        if !fs::metadata(&binary_path).is_ok_and(|metadata| metadata.is_file()) {
            zed::set_language_server_installation_status(
                language_server_id,
                &InstallationStatus::Downloading,
            );
            let file_type = if os == zed::Os::Windows {
                zed::DownloadedFileType::Zip
            } else {
                zed::DownloadedFileType::GzipTar
            };
            zed::download_file(&asset.download_url, &version_dir, file_type)
                .map_err(|error| format!("failed to download Encore LSP: {error}"))?;
            if os != zed::Os::Windows {
                zed::make_file_executable(&binary_path)
                    .map_err(|error| format!("failed to make Encore LSP executable: {error}"))?;
            }
        }
        if !fs::metadata(&binary_path).is_ok_and(|metadata| metadata.is_file()) {
            return Err(format!(
                "Encore LSP is absent after extracting {asset_name}"
            ));
        }

        zed::set_language_server_installation_status(language_server_id, &InstallationStatus::None);
        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }
}

impl zed::Extension for EncoreExtension {
    fn new() -> Self
    where
        Self: Sized,
    {
        Self {
            cached_binary_path: None,
        }
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
        let binary = self.language_server_binary(language_server_id, worktree)?;
        Ok(zed::Command {
            command: binary.path,
            args: binary.args,
            env: binary.env,
        })
    }

    fn language_server_initialization_options(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(
            LspSettings::for_worktree(language_server_id.as_ref(), worktree)?
                .initialization_options,
        )
    }

    fn language_server_workspace_configuration(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<Option<zed::serde_json::Value>> {
        Ok(LspSettings::for_worktree(language_server_id.as_ref(), worktree)?.settings)
    }
}

fn environment_value(environment: &zed::EnvVars, name: &str) -> Option<String> {
    environment
        .iter()
        .find(|(candidate, _)| candidate == name)
        .map(|(_, value)| value.clone())
}

zed::register_extension!(EncoreExtension);
