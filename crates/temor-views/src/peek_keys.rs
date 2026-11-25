//! What a key event means for the peek card (PLAN §10): a modifier going down or up moves ⌘, any
//! other key pressed is typing. The platform tells a modifier key's own press without its bit
//! and its release with it, so the bit is applied here.

use dioxus::html::{Key, Modifiers as HeldMods, NamedKey};
use ui_components::edit::raw_key::{KeyPhase, RawKey};

use temor_hub::peek_step::Command;
use temor_pane_view::input::vt_mods;

/// What a key event says.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum KeySays {
    /// ⌘ is now held or up; look at the pointer again.
    Look(Command),
    /// A key was typed.
    Typed,
}

/// The modifier bit a modifier key stands for.
fn bit_of(key: &Key) -> Option<HeldMods> {
    Some(match key {
        Key::Named(NamedKey::Control) => HeldMods::CONTROL,
        Key::Named(NamedKey::Shift) => HeldMods::SHIFT,
        Key::Named(NamedKey::Alt) => HeldMods::ALT,
        Key::Named(NamedKey::AltGraph) => HeldMods::ALT_GRAPH,
        Key::Named(NamedKey::Meta) => HeldMods::META,
        Key::Named(NamedKey::CapsLock) => HeldMods::CAPS_LOCK,
        Key::Named(NamedKey::NumLock) => HeldMods::NUM_LOCK,
        Key::Named(NamedKey::ScrollLock) => HeldMods::SCROLL_LOCK,
        Key::Named(NamedKey::Fn) => HeldMods::FN,
        Key::Named(NamedKey::FnLock) => HeldMods::FN_LOCK,
        Key::Named(NamedKey::Symbol) => HeldMods::SYMBOL,
        Key::Named(NamedKey::SymbolLock) => HeldMods::SYMBOL_LOCK,
        _ => return None,
    })
}

/// What `raw` means for the peek.
pub fn says(raw: &RawKey) -> KeySays {
    match (bit_of(&raw.key), raw.phase) {
        (Some(bit), KeyPhase::Release) => KeySays::Look(Command::of(vt_mods(raw.modifiers - bit))),
        (Some(bit), KeyPhase::Press | KeyPhase::Repeat) => {
            KeySays::Look(Command::of(vt_mods(raw.modifiers | bit)))
        }
        (None, KeyPhase::Press | KeyPhase::Repeat) => KeySays::Typed,
        (None, KeyPhase::Release) => KeySays::Look(Command::of(vt_mods(raw.modifiers))),
    }
}

#[cfg(test)]
mod tests {
    use dioxus::html::{Code, Location};

    use super::*;

    fn key(phase: KeyPhase, key: Key, held: HeldMods) -> RawKey {
        RawKey {
            phase,
            key,
            code: Code::Unidentified,
            modifiers: held,
            location: Location::Standard,
            text: None,
        }
    }

    #[test]
    fn modifiers_move_command_and_other_keys_are_typing() {
        use Command::{Held, Up};
        use KeyPhase::{Press, Release, Repeat};
        let ctrl = HeldMods::CONTROL;
        let both = ctrl | HeldMods::SHIFT;
        let c = || Key::Character("c".to_owned());
        // (event, what it says). A modifier's own press comes without its bit, its release with it.
        let table = [
            (
                key(Press, Key::Named(NamedKey::Shift), ctrl),
                KeySays::Look(Held),
            ),
            (
                key(Press, Key::Named(NamedKey::Control), HeldMods::empty()),
                KeySays::Look(Up),
            ),
            (
                key(Repeat, Key::Named(NamedKey::Shift), ctrl),
                KeySays::Look(Held),
            ),
            (
                key(Release, Key::Named(NamedKey::Shift), both),
                KeySays::Look(Up),
            ),
            (
                key(Release, Key::Named(NamedKey::Control), both),
                KeySays::Look(Up),
            ),
            (
                key(Release, Key::Named(NamedKey::Alt), both | HeldMods::ALT),
                KeySays::Look(Held),
            ),
            (key(Press, c(), both), KeySays::Typed),
            (key(Repeat, c(), HeldMods::empty()), KeySays::Typed),
            (key(Release, c(), both), KeySays::Look(Held)),
            (key(Release, c(), ctrl), KeySays::Look(Up)),
        ];
        for (raw, want) in table {
            assert_eq!(says(&raw), want, "{raw:?}");
        }
    }
}
