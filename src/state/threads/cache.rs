use crate::configuration::read_config;
use crate::state::cache::is_cache_expired;
use crate::state::context::write_context;

pub(crate) fn clean_expired_cache() {
    let cache = &mut write_context().cache;
    let popup_data_expiration_duration = read_config().max_popup_data_expiration_duration;
    cache.popup_data_map.retain(|_, popup_data| {
        !is_cache_expired(popup_data_expiration_duration, popup_data.cached_date)
    });
}
