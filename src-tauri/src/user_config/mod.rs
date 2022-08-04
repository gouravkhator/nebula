mod editor_settings;
mod keybindings;
mod tooling_prefs;

use editor_settings::EditorSettings;
use keybindings::KeyBindings;
use tooling_prefs::ToolingPrefs;

pub struct Config {
    keybindings: KeyBindings,
    tooling_prefs: ToolingPrefs,
    editor_settings: EditorSettings,
}
