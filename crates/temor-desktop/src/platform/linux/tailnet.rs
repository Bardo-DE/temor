//! The person's other Tailscale computers, read from the accounts service
//! (`org.bardo.Tailnet1`, PLAN §9b, D22). temor never talks to tailscaled. With no session bus,
//! no account service, no Tailscale account or a stopped Tailscale the list is empty.

use super::bus::session_builder;
use crate::tailnet::{TailnetChanges, Ticks};
use crate::{DesktopEnv, DesktopError};
use account_client::{Changes, LinkError};
use account_client_dbus::{Bus, Tailnet};
use account_tailscale::{Machine, MachineOwner};
use std::future::Future;
use std::pin::{Pin, pin};
use std::task::{Context, Poll};
use temor_ssh::{NodeId, TailnetHost, TailnetOwner, UnixSeconds};

/// The tailnet list over the session bus.
#[derive(Debug)]
pub struct SystemTailnet {
    bus: Option<Bus>,
}

impl SystemTailnet {
    /// A reader on the session bus of `env`; without a bus it finds no computers.
    pub async fn connect(env: &DesktopEnv) -> Result<Self, DesktopError> {
        let builder = match session_builder(env) {
            Ok(builder) => builder,
            Err(DesktopError::NoBus) => return Ok(Self { bus: None }),
            Err(error) => return Err(error),
        };
        let connection = builder
            .build()
            .await
            .map_err(|error| DesktopError::bus("connect to the session bus", error))?;
        Ok(Self {
            bus: Some(Bus::over(connection)),
        })
    }

    /// The other computers on the person's tailnet, in the account service's order; this one is
    /// left out. Empty when Tailscale is absent.
    pub async fn hosts(&self) -> Result<Vec<TailnetHost>, DesktopError> {
        let Some(tailnet) = self.tailnet() else {
            return Ok(Vec::new());
        };
        match tailnet.machines().await {
            Ok(machines) => Ok(machines.iter().filter_map(host_of).collect()),
            Err(error) => absent_or_failed(error).map(|()| Vec::new()),
        }
    }

    /// A stream that ticks each time the list changed (read it again). Subscribe before the
    /// first [`SystemTailnet::hosts`] so no change is missed. Already ended when Tailscale is
    /// absent.
    pub async fn changes(&self) -> Result<TailnetChanges, DesktopError> {
        let Some(tailnet) = self.tailnet() else {
            return Ok(TailnetChanges::ended());
        };
        match tailnet.watch().await {
            Ok(changes) => Ok(TailnetChanges::over(Watch(changes))),
            Err(error) => absent_or_failed(error).map(|()| TailnetChanges::ended()),
        }
    }

    fn tailnet(&self) -> Option<Tailnet> {
        self.bus.as_ref().map(Bus::tailnet)
    }
}

/// No account service is the same as no computers; anything else is a failure to report.
fn absent_or_failed(error: LinkError) -> Result<(), DesktopError> {
    match error {
        LinkError::Unreachable => Ok(()),
        other => Err(DesktopError::bus("read the Tailscale computers", other)),
    }
}

/// The accounts service's change signal as a source of ticks. A signal that does not parse still
/// says the list changed, so it ticks too: the list is read again either way.
struct Watch(Changes<()>);

impl Ticks for Watch {
    fn poll_tick(mut self: Pin<&mut Self>, cx: &mut Context<'_>) -> Poll<Option<()>> {
        pin!(self.0.next())
            .poll(cx)
            .map(|change| change.map(|_| ()))
    }
}

/// One machine as temor's plain host record; `None` for a machine whose node id temor can't use.
pub(super) fn host_of(machine: &Machine) -> Option<TailnetHost> {
    Some(TailnetHost {
        node: NodeId::new(machine.node.as_str())?,
        name: machine.name.clone(),
        dns: machine.dns.clone(),
        addresses: machine.addresses.clone(),
        online: machine.online,
        last_seen: machine
            .last_seen
            .and_then(|at| u64::try_from(at.0).ok())
            .map(UnixSeconds),
        tailscale_ssh: machine.ssh,
        owner: match machine.owner {
            MachineOwner::Mine => TailnetOwner::Mine,
            MachineOwner::Shared => TailnetOwner::Shared,
            MachineOwner::Tagged => TailnetOwner::Tagged,
        },
    })
}
