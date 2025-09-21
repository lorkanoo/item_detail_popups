mod render;

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
        pub mod font;
        pub mod popup;
        pub mod link;
        pub mod daemon;
        pub mod cache;
    }
    pub mod cache {
        pub mod item_name;
        pub mod price;
        pub mod texture;
        pub mod caching_status;
        pub mod cached_data;
        pub mod cache;
    }
    pub mod keybinds;
    pub mod clipboard;
    pub mod context;
    pub mod ui_context;
    pub mod font;
    pub mod links;
    pub mod popup;
    pub mod mumble;
}

mod api {
    pub mod gw2_api;
    pub mod gw2_tp;
    pub mod gw2_wiki;
}

pub mod core {
    pub mod addon;
    pub(crate) mod threads;
    pub mod http_client;

    pub mod utils {
        pub mod item_name;
        pub mod key_combination;
        pub mod serde;
        pub mod ui;
    }
    pub mod vo {}
}

use nexus::{AddonFlags, UpdateProvider};
use self::core::addon::load;
use self::core::addon::unload;


nexus::export! {
    name: "Item detail popups",
    signature: -0xc347f84,
    flags: AddonFlags::None,
    load: load,
    unload: unload,
    provider: UpdateProvider::GitHub,
    update_link: env!("CARGO_PKG_REPOSITORY"),
}
