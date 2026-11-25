//! A pane action's chord as the toolkit draws it (a [`Shortcut`]), for menus, tooltips and the ⌘T bar
//! rows. The chord is the live one the keymap holds for this platform, so a person's own binding
//! shows too. Pure.

use keymap::{DefaultChord, Keymap};
use temor_panes::pane::keys::Action;
use ui_core::vocab::Shortcut;

/// What a person presses in a terminal window to run `action` (`Ctrl+Shift+D` on Windows or KDE,
/// where plain Ctrl is the program's). `None` when the action has no chord, or one the toolkit has no
/// key cap for.
pub fn terminal_shortcut_of(keymap: &Keymap, action: Action) -> Option<Shortcut> {
    let named = action.to_keymap()?;
    let chord = keymap
        .chords_in(&named, keymap::Context::Terminal)
        .into_iter()
        .next()?;
    Shortcut::from_default_chord(DefaultChord::new(false, chord))
}
