//! The kit's Spaces handle as temor types it.

use temor_client::model::space::{Place, Project};
use ui_components::components::app::spaces::SpacesHandle;

/// The window's Spaces: temor's project in each, and the tabs it was left with as its place.
pub type SpacesOf = SpacesHandle<Project, Place>;
