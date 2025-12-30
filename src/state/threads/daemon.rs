use crate::state::cache::{is_cache_expired, Persist, StoreInCache};

use crate::configuration::read_config;
use crate::configuration::textures_dir;
use crate::render::popup_data::price::{COPPER_COIN_HREF, GOLD_COIN_HREF, SILVER_COIN_HREF};
use crate::state::context::{read_context, save_cache, write_context};
use crate::state::threads::cache::clean_expired_cache;
use crate::state::threads::font::{load_fonts, preselect_fonts};
use crate::threads::lock_threads;
use chrono::Local;
use log::debug;
use log::error;
use std::time::Duration;
use std::{fs, thread};

const DAEMON_THREAD_SLEEP_DURATION_MS: u64 = 50;
const GC_INTERVAL_SEC: u64 = 120;
const CONFIG_SAVE_INTERVAL_SEC: u64 = 5;
const CACHE_SAVE_INTERVAL_SEC: u64 = 60;

pub fn daemon_thread() {
    lock_threads().push(thread::spawn(|| loop {
        if !read_context().run_background_thread {
            break;
        }
        clean_finished_threads();

        let now = Local::now();
        if now
            > read_context().last_config_save_date + Duration::from_secs(CONFIG_SAVE_INTERVAL_SEC)
        {
            read_config().save();
            write_context().last_config_save_date = now;
        }

        if now > read_context().last_cache_save_date + Duration::from_secs(CACHE_SAVE_INTERVAL_SEC)
        {
            save_cache();
            write_context().last_cache_save_date = now;
        }

        if now > read_context().last_gc_date + Duration::from_secs(GC_INTERVAL_SEC) {
            clean_expired_cache();
            clean_expired_textures();
            write_context().last_gc_date = now;
        }

        thread::sleep(Duration::from_millis(DAEMON_THREAD_SLEEP_DURATION_MS));
    }));
}

pub fn preloader_thread() {
    lock_threads().push(thread::spawn(|| {
        load_fonts();
        write_context()
            .cache
            .textures
            .retrieve(GOLD_COIN_HREF.to_string());
        write_context()
            .cache
            .textures
            .retrieve(SILVER_COIN_HREF.to_string());
        write_context()
            .cache
            .textures
            .retrieve(COPPER_COIN_HREF.to_string());
        thread::sleep(Duration::from_millis(4000));
        preselect_fonts();
    }));
}

fn clean_expired_textures() {
    let texture_expiration_duration = read_config().max_texture_expiration_duration;
    let entries = fs::read_dir(textures_dir());
    if entries.is_err() {
        error!("[clean_expired_textures] Couldn't clean expired textures");
        return;
    }

    let mut removed_count = 0;
    for entry in entries.unwrap() {
        if entry.is_err() {
            error!("[clean_expired_textures] Couldn't process entry");
            continue;
        }
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        let metadata = fs::metadata(&path);
        if metadata.is_err() {
            error!("[clean_expired_textures] Couldn't extract metadata");
            continue;
        }
        let metadata = metadata.unwrap();
        if let Ok(created) = metadata.created() {
            if is_cache_expired(texture_expiration_duration, created.into()) {
                let _ = fs::remove_file(path);
                removed_count += 1;
            }
        }
    }
    debug!(
        "[clean_expired_textures] Removed {} textures",
        removed_count
    );
}

fn clean_finished_threads() {
    lock_threads().retain(|handle| !handle.is_finished());
}
