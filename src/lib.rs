use zed_extension_api as zed;

struct UonExtension;

impl zed::Extension for UonExtension {
    fn new() -> Self {
        Self
    }

    fn language_server_command(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        worktree: &zed::Worktree,
    ) -> zed::Result<zed::Command> {
        let path = worktree.which("uon-lsp").ok_or_else(|| {
            "uon-lsp must be installed in PATH (hint: cargo install --path uon-lsp)".to_string()
        })?;
        Ok(zed::Command {
            command: path,
            args: vec![],
            env: vec![],
        })
    }

    fn language_server_initialization_options(
        &mut self,
        _language_server_id: &zed::LanguageServerId,
        _worktree: &zed::Worktree,
    ) -> zed::Result<Option<zed::serde_json::Value>> {
        Ok(None)
    }
}

zed::register_extension!(UonExtension);
