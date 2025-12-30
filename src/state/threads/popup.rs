use crate::api::gw2_wiki::prepare_item_popup_with_quantity;

use crate::state::context::write_context;
use crate::state::popup::popup_state::PopupState;
use crate::threads::lock_threads;
use std::thread;

pub fn refresh_popup_thread(popup_state: PopupState, popup_title: String) {
    lock_threads().push(thread::spawn(move || {
        write_context().ui.loading_progress = Some(1);
        let mut refreshed_popup =
            prepare_item_popup_with_quantity(popup_title.as_str(), &popup_state.item_quantity);
        refreshed_popup.state = popup_state;
        write_context().ui.pinned_popups.push(refreshed_popup);
        write_context().ui.loading_progress = None;
    }));
}
