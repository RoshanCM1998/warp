use settings::macros::define_settings_group;
use settings::{RespectUserSyncSetting, SupportedPlatforms, SyncToCloud};

define_settings_group!(CodeSettings, settings: [
    code_as_default_editor: CodeAsDefaultEditor {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Never,
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.editor.use_warp_as_default_editor",
        description: "Whether Warp is used as the default code editor.",
    }
    codebase_context_enabled: CodebaseContextEnabled {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        storage_key: "AgentModeCodebaseContext",
        toml_path: "code.indexing.agent_mode_codebase_context",
        description: "Whether codebase context is provided to the AI agent.",
    },
    auto_indexing_enabled: AutoIndexingEnabled {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        storage_key: "AgentModeCodebaseContextAutoIndexing",
        toml_path: "code.indexing.agent_mode_codebase_context_auto_indexing",
        description: "Whether automatic codebase indexing is enabled.",
    },
    // Whether or not the user has manually dismissed the code toolbelt new feature popup.
    dismissed_code_toolbelt_new_feature_popup: DismissedCodeToolbeltNewFeaturePopup {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: true,
    },
    // Controls whether the project explorer / file tree appears in the tools panel.
    show_project_explorer: ShowProjectExplorer {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.editor.show_project_explorer",
        description: "Whether the project explorer is shown in the tools panel.",
    },
    // Controls whether global file search appears in the tools panel.
    show_global_search: ShowGlobalSearch {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.editor.show_global_search",
        description: "Whether global file search is shown in the tools panel.",
    },
    // Controls whether hidden files (dotfiles) are shown in the project explorer.
    show_hidden_files: ShowHiddenFiles {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.editor.show_hidden_files",
        description: "Whether hidden files (dotfiles) are shown in the project explorer.",
    },
    // When the terminal's working directory is not itself a git repository,
    // scan its direct subfolders (one level) for git repos and list them in
    // the Code Review repo dropdown. Off by default.
    scan_child_repos: ScanChildRepos {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.review.scan_child_repos",
        description: "When the terminal folder isn't a git repository, scan its direct subfolders for repositories and list them in Code Review.",
    },
    diff_layout: DiffLayoutSetting {
        type: crate::code::diff_layout::DiffLayout,
        default: crate::code::diff_layout::DiffLayout::Inline,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.editor.diff_layout",
        description: "Layout for Code Review diff views: 'inline' or 'side_by_side'.",
    },
    // Enables the git staging area (staged/unstaged sections) in Code Review.
    // Off by default; only takes effect when the `code_review_staging` feature flag is enabled.
    staging_area: StagingArea {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::DESKTOP,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.review.staging_area",
        description: "Show a git staging area with separate staged and unstaged sections in Code Review.",
    },
    // Controls whether the language server reformats the file on save.
    format_on_save: FormatOnSave {
        type: bool,
        default: true,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.editor.format_on_save",
        description: "Whether the language server automatically formats the file on save. Other LSP features (hover, go-to-definition, references, diagnostics) are unaffected.",
    },
    // Controls whether the Warp text editor automatically saves file changes as the
    // user types (debounced) and when the editor loses focus. Only applies to the
    // Warp text editor, not the command line or AI input.
    auto_save: AutoSave {
        type: bool,
        default: false,
        supported_platforms: SupportedPlatforms::ALL,
        sync_to_cloud: SyncToCloud::Globally(RespectUserSyncSetting::Yes),
        surface: settings::SettingSurfaces::GUI,
        private: false,
        toml_path: "code.editor.auto_save",
        description: "Whether the Warp text editor automatically saves changes as you type and when the editor loses focus.",
    },
]);

#[cfg(test)]
mod format_on_save_tests {
    use settings::Setting;
    use warpui::{App, SingletonEntity};

    use super::*;
    use crate::test_util::settings::initialize_settings_for_tests;

    #[test]
    fn format_on_save_defaults_to_true() {
        App::test((), |mut app| async move {
            initialize_settings_for_tests(&mut app);

            CodeSettings::handle(&app).read(&app, |settings, _ctx| {
                assert!(*settings.format_on_save);
            });
        });
    }

    #[test]
    fn format_on_save_uses_code_editor_toml_path() {
        assert_eq!(
            FormatOnSave::toml_path(),
            Some("code.editor.format_on_save")
        );
        assert_eq!(FormatOnSave::hierarchy(), Some("code.editor"));
        assert_eq!(FormatOnSave::toml_key(), "format_on_save");
    }
}
