use crate::api::gw2_wiki::prepare_href_popup;

use crate::core::threads::lock_threads;
use crate::state::context::write_context;
use log::debug;
use std::thread;

pub fn open_link_thread(href: String, title: String) {
    debug!(
        "[open_link_thread] Opening link with href: {} and title: {}",
        href, title
    );
    lock_threads().push(thread::spawn(move || {
        write_context().ui.loading_progress = Some(1);
        write_context().ui.hovered_popup = Some(prepare_href_popup(&href, title));
        write_context().ui.loading_progress = None;
    }));
}
