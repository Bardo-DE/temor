//! A chord the person pressed, written for `keys.conf`: with `Primary` where the platform's
//! primary modifiers were held, so the line means the same on every machine the file reaches.

use keymap::{Chord, Context, DefaultChord, Platform};

/// `chord`, as pressed in a terminal window of `platform`, in the form `keys.conf` holds
/// (`Primary+Alt+N`). It uses `Primary` only when that resolves back to exactly `chord`;
/// otherwise the chord is written as pressed.
pub fn portable_chord(platform: Platform, chord: Chord) -> String {
    let primary = platform.primary(Context::Terminal);
    if !primary.is_empty() && chord.modifiers().contains(primary) {
        let base = Chord::new(chord.modifiers().difference(primary), chord.key());
        let held = DefaultChord::new(true, base);
        if held.resolve(platform, Context::Terminal) == Some(chord) {
            return held.canonical();
        }
    }
    DefaultChord::from(chord).canonical()
}

#[cfg(test)]
mod tests {
    use keymap::keyboard_types::Modifiers;
    use keymap::{Chord, Key, Platform};

    use super::portable_chord;

    fn chord(mods: Modifiers, key: char) -> Chord {
        Chord::new(mods, Key::Char(key))
    }

    #[test]
    fn the_primary_modifiers_are_written_as_primary() {
        let rows = [
            (
                Platform::Windows,
                chord(Modifiers::CONTROL | Modifiers::SHIFT | Modifiers::ALT, 'n'),
                "Primary+Alt+N",
            ),
            (Platform::Windows, chord(Modifiers::ALT, 'n'), "Alt+N"),
        ];
        for (platform, pressed, want) in rows {
            assert_eq!(portable_chord(platform, pressed), want, "{pressed:?}");
        }
    }
}
