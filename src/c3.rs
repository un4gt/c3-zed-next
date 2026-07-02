use std::{fs, path::Path};

use zed_extension_api::{
    self as zed, current_platform, download_file, github_release_by_tag_name,
    lsp::{CompletionKind, SymbolKind},
    make_file_executable, set_language_server_installation_status, Architecture,
    DownloadedFileType, LanguageServerInstallationStatus, Os, Result,
};

const C3LSP_REPOSITORY: &str = "pherrymason/c3-lsp";
const C3LSP_VERSION: &str = "v0.4.0";
const C3LSP_DEFAULT_DIAGNOSTICS_DELAY_MS: u64 = 250;
const C3LSP_DIAGNOSTICS_DELAY_ENV_VAR: &str = "C3_ZED_DIAGNOSTICS_DELAY_MS";

struct C3LspAsset {
    archive_name: &'static str,
    binary_path: &'static str,
    file_type: DownloadedFileType,
    make_executable: bool,
}

struct C3Extension {
    cached_binary_path: Option<String>,
}

impl C3Extension {
    fn asset_for_current_platform() -> Result<C3LspAsset> {
        let (os, architecture) = current_platform();

        match (os, architecture) {
            (Os::Windows, Architecture::X8664) => Ok(C3LspAsset {
                archive_name: "c3lsp-windows-amd64.zip",
                binary_path: "server/bin/release/c3lsp.exe",
                file_type: DownloadedFileType::Zip,
                make_executable: false,
            }),
            (Os::Linux, Architecture::X8664) => Ok(C3LspAsset {
                archive_name: "c3lsp-linux-amd64.tar.gz",
                binary_path: "server/bin/release/c3lsp",
                file_type: DownloadedFileType::GzipTar,
                make_executable: true,
            }),
            (Os::Mac, Architecture::Aarch64) => Ok(C3LspAsset {
                archive_name: "c3lsp-darwin-arm64.zip",
                binary_path: "server/bin/release/c3lsp",
                file_type: DownloadedFileType::Zip,
                make_executable: true,
            }),
            _ => Err(format!(
                "c3lsp {C3LSP_VERSION} does not provide a prebuilt binary for {os:?}/{architecture:?}; install c3lsp and add it to PATH"
            )),
        }
    }

    fn language_server_binary_path(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<String> {
        if let Some(path) = worktree.which("c3lsp") {
            return Ok(path);
        }

        if let Some(path) = &self.cached_binary_path {
            if Path::new(path).is_file() {
                return Ok(path.clone());
            }
        }

        self.install_c3lsp(language_server_id)
    }

    fn install_c3lsp(&mut self, language_server_id: &zed::LanguageServerId) -> Result<String> {
        let asset = Self::asset_for_current_platform().map_err(|error| {
            Self::installation_failed(
                language_server_id,
                format!("failed to select c3lsp asset: {error}"),
            )
        })?;

        let archive_stem = asset
            .archive_name
            .strip_suffix(".tar.gz")
            .or_else(|| asset.archive_name.strip_suffix(".zip"))
            .unwrap_or(asset.archive_name);
        let install_dir = format!("c3lsp/{C3LSP_VERSION}/{archive_stem}");
        let binary_path = format!("{install_dir}/{}", asset.binary_path);

        if Path::new(&binary_path).is_file() {
            self.cached_binary_path = Some(binary_path.clone());
            return Ok(binary_path);
        }

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::CheckingForUpdate,
        );

        let release =
            github_release_by_tag_name(C3LSP_REPOSITORY, C3LSP_VERSION).map_err(|error| {
                Self::installation_failed(
                    language_server_id,
                    format!("failed to fetch c3lsp {C3LSP_VERSION} release metadata: {error}"),
                )
            })?;

        let release_asset = release
            .assets
            .iter()
            .find(|release_asset| release_asset.name == asset.archive_name)
            .ok_or_else(|| {
                Self::installation_failed(
                    language_server_id,
                    format!(
                        "c3lsp {C3LSP_VERSION} release does not contain asset {}",
                        asset.archive_name
                    ),
                )
            })?;

        fs::create_dir_all(&install_dir).map_err(|error| {
            Self::installation_failed(
                language_server_id,
                format!("failed to create c3lsp install directory {install_dir}: {error}"),
            )
        })?;

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Downloading,
        );

        download_file(&release_asset.download_url, &install_dir, asset.file_type).map_err(
            |error| {
                Self::installation_failed(
                    language_server_id,
                    format!(
                        "failed to download c3lsp asset {} from {}: {error}",
                        asset.archive_name, release_asset.download_url
                    ),
                )
            },
        )?;

        if asset.make_executable {
            make_file_executable(&binary_path).map_err(|error| {
                Self::installation_failed(
                    language_server_id,
                    format!("failed to make c3lsp executable at {binary_path}: {error}"),
                )
            })?;
        }

        if !Path::new(&binary_path).is_file() {
            return Err(Self::installation_failed(
                language_server_id,
                format!(
                    "downloaded c3lsp asset {}, but expected binary was not found at {binary_path}",
                    asset.archive_name
                ),
            ));
        }

        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::None,
        );
        self.cached_binary_path = Some(binary_path.clone());
        Ok(binary_path)
    }

    fn installation_failed(language_server_id: &zed::LanguageServerId, message: String) -> String {
        set_language_server_installation_status(
            language_server_id,
            &LanguageServerInstallationStatus::Failed(message.clone()),
        );
        message
    }

    fn label_for_named_item(name: &str, prefix: &str, suffix: &str) -> Option<zed::CodeLabel> {
        if name.is_empty() {
            return None;
        }

        let code = format!("{prefix}{name}{suffix}");
        let name_start = prefix.len();

        Some(zed::CodeLabel {
            code,
            spans: vec![zed::CodeLabelSpan::code_range(
                name_start..name_start + name.len(),
            )],
            filter_range: (0..name.len()).into(),
        })
    }

    fn diagnostics_delay_ms(env: &[(String, String)]) -> String {
        env.iter()
            .find_map(|(key, value)| {
                if key.eq_ignore_ascii_case(C3LSP_DIAGNOSTICS_DELAY_ENV_VAR) {
                    value.parse::<u64>().ok().filter(|ms| *ms <= 60_000)
                } else {
                    None
                }
            })
            .unwrap_or(C3LSP_DEFAULT_DIAGNOSTICS_DELAY_MS)
            .to_string()
    }
}

impl zed::Extension for C3Extension {
    fn new() -> Self {
        Self {
            cached_binary_path: None,
        }
    }

    fn language_server_command(
        &mut self,
        language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> Result<zed::Command> {
        let env = worktree.shell_env();
        let mut args = vec![
            "-diagnostics-delay".to_string(),
            Self::diagnostics_delay_ms(&env),
        ];

        if let Some(path) = worktree.which("c3c") {
            args.push("-c3c-path".to_string());
            args.push(path);
        }

        Ok(zed::Command {
            command: self.language_server_binary_path(language_server_id, worktree)?,
            args,
            env,
        })
    }

    fn label_for_completion(
        &self,
        _language_server_id: &zed::LanguageServerId,
        completion: zed::lsp::Completion,
    ) -> Option<zed::CodeLabel> {
        // Parse small C3 snippets so Zed can syntax-highlight completion labels.
        let precolon = "int x = s(a";
        let colon = ": ";
        let postcolon = " 5);\n";
        let colon_prelude = format!("{precolon}{colon}{postcolon}\n");

        let colon_span = zed::CodeLabelSpan::code_range({
            let start = precolon.len();
            start..start + colon.len()
        });

        let name = &completion.label;
        let detail = &completion.detail?;

        let (code, spans) = match completion.kind? {
            kind @ (CompletionKind::Variable
            | CompletionKind::Field
            | CompletionKind::Method
            | CompletionKind::Function
            | CompletionKind::Constructor) => {
                let mut type_prelude = "def Mt = ";
                let ty = detail;
                let mut type_postlude = ";\nint a = ";
                let mut call = "";

                if matches!(
                    kind,
                    CompletionKind::Function | CompletionKind::Method | CompletionKind::Constructor
                ) {
                    call = "()";
                    if ty.starts_with("macro") {
                        type_prelude = "";
                        type_postlude = "{}\nint a = ";
                    }
                }

                let code = format!("{colon_prelude}{type_prelude}{ty}{type_postlude}{name}{call};");

                (
                    code,
                    vec![
                        zed::CodeLabelSpan::code_range({
                            let start = colon_prelude.len()
                                + type_prelude.len()
                                + ty.len()
                                + type_postlude.len();
                            start..start + name.len()
                        }),
                        colon_span,
                        zed::CodeLabelSpan::code_range({
                            let start = colon_prelude.len() + type_prelude.len();
                            start..start + ty.len()
                        }),
                    ],
                )
            }
            _ => {
                let detail_prelude = "; ])>)";
                let name_prelude = "int a = ";
                let code = format!("{colon_prelude}{name_prelude}{name}{detail_prelude}{detail}");

                (
                    code,
                    vec![
                        zed::CodeLabelSpan::code_range({
                            let start = colon_prelude.len() + name_prelude.len();
                            start..start + name.len()
                        }),
                        colon_span,
                        zed::CodeLabelSpan::code_range({
                            let start = colon_prelude.len()
                                + name_prelude.len()
                                + name.len()
                                + detail_prelude.len();
                            start..start + detail.len()
                        }),
                    ],
                )
            }
        };

        Some(zed::CodeLabel {
            spans,
            filter_range: (0..name.len()).into(),
            code,
        })
    }

    fn label_for_symbol(
        &self,
        _language_server_id: &zed::LanguageServerId,
        symbol: zed::lsp::Symbol,
    ) -> Option<zed::CodeLabel> {
        match symbol.kind {
            SymbolKind::Module | SymbolKind::Namespace | SymbolKind::Package => {
                Self::label_for_named_item(&symbol.name, "module ", ";\n")
            }
            SymbolKind::Class | SymbolKind::Struct => {
                Self::label_for_named_item(&symbol.name, "struct ", " {}\n")
            }
            SymbolKind::Enum => Self::label_for_named_item(&symbol.name, "enum ", " {}\n"),
            SymbolKind::Interface => {
                Self::label_for_named_item(&symbol.name, "interface ", " {}\n")
            }
            SymbolKind::Function | SymbolKind::Method | SymbolKind::Constructor => {
                Self::label_for_named_item(&symbol.name, "fn void ", "();\n")
            }
            SymbolKind::Constant | SymbolKind::EnumMember => {
                Self::label_for_named_item(&symbol.name, "const int ", " = 0;\n")
            }
            SymbolKind::Field | SymbolKind::Property | SymbolKind::Variable => {
                Self::label_for_named_item(&symbol.name, "int ", ";\n")
            }
            SymbolKind::TypeParameter => {
                Self::label_for_named_item(&symbol.name, "typedef int ", ";\n")
            }
            _ => Self::label_for_named_item(&symbol.name, "int ", ";\n"),
        }
    }
}

zed::register_extension!(C3Extension);
