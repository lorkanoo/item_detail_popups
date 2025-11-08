mod render;
mod service;
pub mod configuration {
    pub mod config;
    pub mod popup {
        pub mod rendering_params;
    }
    pub mod keyboard_layout;
}

pub mod state {
    pub mod vo {}
    pub mod threads {
        pub mod cache;
        pub mod daemon;
        pub mod font;
        pub mod link;
        pub mod popup;
    }
    pub mod cache {
        pub mod cache;
        pub mod cached_data;
        pub mod caching_status;
        pub mod item_name;
        pub mod price;
        pub mod texture;
        pub mod gw2_tp;
    }
    pub mod clipboard;
    pub mod context;
    pub mod font;
    pub mod keybinds;
    pub mod links;
    pub mod mumble;
    pub mod popup;
    pub mod ui_context;
}

mod api;

pub mod core {
    pub mod addon;
    pub mod http_client;
    pub(crate) mod threads;

    pub mod utils {
        pub mod item_tag_parser;
        pub mod key_combination;
        pub mod serde;
        pub mod ui;
    }
    pub mod vo {}
}

use self::core::addon::load;
use self::core::addon::unload;
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
