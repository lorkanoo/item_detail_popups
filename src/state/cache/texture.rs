use crate::api::gw2_wiki::download_wiki_image;
use crate::configuration::read_config;
use crate::configuration::textures_dir;
use crate::state::cache::cached_data::CachedData;
use crate::state::cache::caching_status::CachingStatus;
use crate::state::cache::is_cache_expired;
use crate::state::cache::StoreInCache;
use crate::state::context::write_context;
use crate::threads::lock_threads;
use chrono::Local;
use log::{debug, error};
use nexus::texture::{load_texture_from_file, RawTextureReceiveCallback, Texture};
use nexus::texture_receive;
use std::collections::HashMap;
use std::thread;

pub const RECEIVE_TEXTURE: RawTextureReceiveCallback = texture_receive!(receive_texture);
pub const TEXTURE_PREFIX: &str = "ITEM_DETAIL_POPUPS_URL_";

pub type TextureCache = HashMap<String, CachedData<Texture>>;

impl<'a> StoreInCache<'a, TextureCache, CachedData<Texture>, String> for TextureCache {
    fn retrieve(&'a mut self, texture_id: String) -> Option<CachedData<Texture>> {
        let texture_id_with_prefix = format!("{}{}", TEXTURE_PREFIX, texture_id);
        let mut should_start_caching = false;
        let result = match self.get(&texture_id_with_prefix) {
            Some(texture_cached_data) => {
                let cache_expiration_duration = read_config().max_texture_expiration_duration;
                let mut result = texture_cached_data.clone();
                if is_cache_expired(cache_expiration_duration, texture_cached_data.date)
                    && !matches!(&texture_cached_data.caching_status, CachingStatus::Caching)
                {
                    result = CachedData::new(Local::now());
                    self.insert(texture_id_with_prefix, result.clone());
                    should_start_caching = true;
                }
                Some(result)
            }
            None => {
                should_start_caching = true;
                let result = CachedData::new(Local::now());
                self.insert(texture_id_with_prefix, result.clone());
                Some(result)
            }
        };

        if should_start_caching {
            fetch_texture_thread(texture_id);
        }

        result
    }
}

pub fn fetch_texture_thread(texture_id: String) {
    lock_threads().push(thread::spawn(move || {
        debug!("[fetch_texture_thread] started for {}", texture_id);
        let mut path = textures_dir();
        path.push(identifier_to_filename(&texture_id));
        if !path.exists() {
            debug!(
                "[fetch_texture_thread] File does not exist, downloading: {}",
                path.display()
            );
            if let Err(e) = download_wiki_image(&texture_id) {
                error!("[fetch_texture_thread] failed to download image: {}", e);
                return;
            }
        }
        let texture_id_with_prefix = format!("{}{}", TEXTURE_PREFIX, texture_id);
        load_texture_from_file(texture_id_with_prefix, path, Some(RECEIVE_TEXTURE));
    }));
}

pub fn identifier_to_filename(identifier: &str) -> String {
    identifier.replace("/", "_").replace("\\", "_")
}

pub fn receive_texture(id: &str, texture: Option<&Texture>) {
    if texture.is_none() {
        return;
    }
    write_context().cache.textures.insert(
        id.to_string(),
        CachedData::new_with_value(Local::now(), texture.unwrap().clone())
            .with_caching_status(CachingStatus::Cached),
    );
}
