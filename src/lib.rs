pub mod addon;
mod api;
pub mod configuration;
mod render;
pub mod service;
pub mod state;
pub mod threads;
pub mod utils;

use addon::load;
use addon::unload;
use nexus::{AddonFlags, UpdateProvider};

nexus::export! {
    name: "Item detail popups",
    signature: -0xc347f84,
    flags: AddonFlags::None,
    load: load,
    unload: unload,
    provider: UpdateProvider::GitHub,
    update_link: env!("CARGO_PKG_REPOSITORY"),
}
