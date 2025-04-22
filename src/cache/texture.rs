use crate::addon::Addon;
use crate::api::gw2_wiki::download_wiki_image;
use crate::cache::{CachedData, CachingStatus};
use crate::config::textures_dir;
use chrono::Local;
use log::{debug, error};
use nexus::texture::{load_texture_from_file, RawTextureReceiveCallback, Texture};
use nexus::texture_receive;
use std::thread;
use super::Cacheable;
use super::is_cache_expired;
use std::collections::HashMap;

pub const RECEIVE_TEXTURE: RawTextureReceiveCallback = texture_receive!(receive_texture);
pub const TEXTURE_PREFIX: &str = "ITEM_DETAIL_POPUPS_URL_";
pub type TextureCache = HashMap<String, CachedData<Texture>>;

impl<'a> Cacheable<'a, TextureCache, CachedData<Texture>, String> for TextureCache {
    fn retrieve(&'a mut self, texture_id: String) -> Option<CachedData<Texture>> {
        let texture_id_with_prefix = format!("{}{}", TEXTURE_PREFIX, texture_id);
        let mut should_start_caching = false;
        let result = match self.get(&texture_id_with_prefix) {
            Some(texture_cached_data) => {
                let cache_expiration_duration = Addon::lock_config().texture_expiration_duration.clone();
                let mut result = texture_cached_data.clone();
                if is_cache_expired(cache_expiration_duration, texture_cached_data.date) && !matches!(&texture_cached_data.caching_status, CachingStatus::InProgress) {
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
    Addon::lock_threads().push(thread::spawn(move || {
        debug!("[fetch_texture_thread] started for {}", texture_id);
        let mut path = textures_dir();
        path.push(identifier_to_filename(&texture_id));
        if !path.exists() {
            debug!(
                "[fetch_texture_thread] File does not exist, downloading: {}",
                path.display()
            );
            match download_wiki_image(&texture_id) {
                Err(e) => {
                    error!("[fetch_texture_thread] failed to download image: {}", e);
                    return;
                }
                _ => {}
            }
        }
        let texture_id_with_prefix = format!("{}{}", TEXTURE_PREFIX, texture_id);
        load_texture_from_file(texture_id_with_prefix, path, Some(RECEIVE_TEXTURE));
    }));
}

pub fn identifier_to_filename(identifier: &String) -> String {
    identifier.replace("/", "_").replace("\\", "_")
}

pub fn receive_texture(id: &str, texture: Option<&Texture>) {
    if texture.is_none() {
        return;
    }
    Addon::lock_cache().textures.insert(
        id.to_string(),
        CachedData::new_with_value(Local::now(), texture.unwrap().clone()).with_caching_status(CachingStatus::Cached),
    );  
}
