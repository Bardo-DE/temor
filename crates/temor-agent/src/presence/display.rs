//! How an agent state is drawn: the toolkit's companion orb, so a waiting agent in temor looks like a
//! waiting companion anywhere else (§8.1), plus the row's unread weight.

use super::model::{AgentState, Seen};
use ui_core::vocab::CompanionPresence;

/// The orb for `state`. The companion has no finished look, so Done and Failed show a still orb
/// and say they are new through [`attention`].
pub fn companion_presence(state: AgentState) -> CompanionPresence {
    match state {
        AgentState::Idle | AgentState::Done(_) | AgentState::Failed(_) => CompanionPresence::Idle,
        AgentState::Working => CompanionPresence::Working,
        AgentState::Waiting(_) => CompanionPresence::Waiting,
    }
}

/// Whether the tab row is in strong weight (the unread dot): the agent waits or has an unseen result.
pub fn attention(state: AgentState) -> bool {
    match state {
        AgentState::Waiting(_)
        | AgentState::Done(Seen::Unseen)
        | AgentState::Failed(Seen::Unseen) => true,
        AgentState::Idle
        | AgentState::Working
        | AgentState::Done(Seen::Seen)
        | AgentState::Failed(Seen::Seen) => false,
    }
}
