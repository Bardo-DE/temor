//! The Settings window's sheet: the sidebar of pages beside the page, which scrolls under it.
//! Each page is the toolkit's `Form`, which owns the page's margin and the gap between its sections.
//! No design values here: lengths are the toolkit's spacing tokens.

pub const SETTINGS_CSS: &str = "\
.temor-settings { display: grid; grid-template-columns: calc(var(--s-16) * 13) minmax(0, 1fr); width: 100%; height: 100%; min-height: 0; }\
.temor-settings-scroll { height: 100%; min-height: 0; overflow-y: auto; scrollbar-width: none; }\
.temor-settings-page { min-width: 0; }\
";
