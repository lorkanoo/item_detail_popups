mod addon;
mod api;
pub mod config;
pub mod context;
mod render;
mod thread;
pub mod util;
pub mod cache;

use crate::addon::Addon;
use nexus::{AddonFlags, UpdateProvider};

nexus::export! {
    name: "Item detail popups",
    signature: -0xc347f84,
    flags: AddonFlags::None,
    load: Addon::load,
    unload: Addon::unload,
    provider: UpdateProvider::GitHub,
    update_link: env!("CARGO_PKG_REPOSITORY"),
}
