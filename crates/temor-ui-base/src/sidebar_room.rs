//! How much room the window has for the sidebar. Below [`NARROW_BELOW`] the sidebar starts
//! folded away, so the panes keep their width; the person can still show it at any width, and a
//! window that crosses the breakpoint starts over from what its width suggests.

use ui_components::prelude::Px;

/// The window width under which the sidebar starts hidden, in logical pixels. The sidebar is
/// about 240 wide and a pane wants room for a readable terminal (about 480 px) beside it.
pub const NARROW_BELOW: Px = Px(720.0);

/// The window's width, as far as the sidebar is concerned.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SidebarRoom {
    /// Wide enough to show the sidebar beside the panes.
    Wide,
    /// Too narrow: the panes get the whole width until the person asks for the sidebar.
    Narrow,
}

impl SidebarRoom {
    /// The room a window of `width` has.
    pub fn of(width: Px) -> Self {
        match width.0 < NARROW_BELOW.0 {
            true => Self::Narrow,
            false => Self::Wide,
        }
    }

    /// Whether the sidebar starts shown at this width.
    pub fn shown(self) -> bool {
        self == Self::Wide
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn the_sidebar_starts_hidden_under_the_breakpoint_and_shown_from_it() {
        // (width, sidebar)
        let cases = [
            (1600.0, true),
            (720.0, true),
            (719.5, false),
            (480.0, false),
            (0.0, false),
        ];
        for (width, want) in cases {
            assert_eq!(SidebarRoom::of(Px(width)).shown(), want, "{width}");
        }
    }
}
