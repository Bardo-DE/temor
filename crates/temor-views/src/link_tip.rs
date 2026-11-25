//! The tooltip for a hovered link (PLAN §6): its target, by the pointer, in the toolkit's tooltip.

use dioxus::prelude::*;
use ui_components::components::overlays::tooltip::Tooltip;

use temor_pane_view::session::LinkTipSpot;

/// A tooltip standing under the pointer's spot on the grid. It takes no pointer events.
#[component]
#[allow(non_snake_case)]
pub fn LinkTip(spot: LinkTipSpot) -> Element {
    let place = format!("left:{:.1}px;top:{:.1}px", spot.x, spot.y);
    rsx! {
        div { class: "temor-link-tip", style: place,
            Tooltip { text: spot.target, shown: Some(true),
                span { class: "temor-link-tip-spot" }
            }
        }
    }
}
