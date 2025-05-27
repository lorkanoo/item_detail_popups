use crate::addon::Addon;
use crate::api::gw2_wiki::{prepare_href_popup, prepare_item_popup};
use crate::cache::{is_cache_expired, Cacheable};
use crate::config::{fonts_dir, textures_dir};

use crate::context::Font;
use crate::render::popup_data::{COPPER_COIN_HREF, GOLD_COIN_HREF, SILVER_COIN_HREF};
use log::{debug, error, info};
use nexus::font::{add_font_from_file, RawFontReceive};
use nexus::font_receive;
use nexus::imgui::sys;
use nexus::imgui::sys::ImFontConfig;
use std::fs::File;
use std::io::Write;
use std::time::Duration;
use std::{fs, thread};

const MAIN_THREAD_SLEEP_DURATION_MS: u64 = 500;
const GC_THREAD_SLEEP_DURATION_SEC: u64 = 120;

pub fn open_link_thread(href: String, title: String) {
    debug!(
        "[open_link_thread] Opening link with href: {} and title: {}",
        href, title
    );
    Addon::lock_threads().push(thread::spawn(move || {
        Addon::write_context().ui.loading_progress = Some(1);
        Addon::write_context().ui.hovered_popup = Some(prepare_href_popup(&href, title));
        Addon::write_context().ui.loading_progress = None;
    }));
}

pub fn main_background_thread() {
    Addon::lock_threads().push(thread::spawn(|| loop {
        if !Addon::read_context().run_background_thread {
            break;
        }
        clean_finished_threads();
        thread::sleep(Duration::from_millis(MAIN_THREAD_SLEEP_DURATION_MS));
    }));
}

pub fn preloader_thread() {
    Addon::lock_threads().push(thread::spawn(|| {
        load_fonts();
        Addon::write_context()
            .cache
            .textures
            .retrieve(GOLD_COIN_HREF.to_string());
        Addon::write_context()
            .cache
            .textures
            .retrieve(SILVER_COIN_HREF.to_string());
        Addon::write_context()
            .cache
            .textures
            .retrieve(COPPER_COIN_HREF.to_string());
        thread::sleep(Duration::from_millis(2000));
        preselect_fonts();
    }));
}

pub fn gc_thread() {
    Addon::lock_threads().push(thread::spawn(|| loop {
        let mut slept_for = 0;
        while slept_for < GC_THREAD_SLEEP_DURATION_SEC {
            slept_for += 1;
            thread::sleep(Duration::from_secs(1));
            if !Addon::read_context().run_background_thread {
                return;
            }
        }
        clean_expired_cache();
        clean_expired_textures();
    }));
}

pub fn refresh_popup_thread(id: u64, name: String, pos: Option<[f32; 2]>) {
    Addon::lock_threads().push(thread::spawn(move || {
        Addon::write_context().ui.loading_progress = Some(1);
        let mut refreshed_popup = prepare_item_popup(name.as_str());
        refreshed_popup.id = id;
        refreshed_popup.opened = true;
        refreshed_popup.pinned = true;
        refreshed_popup.pos = pos;
        Addon::write_context()
            .ui
            .pinned_popups
            .push(refreshed_popup);
        Addon::write_context().ui.loading_progress = None;
    }));
}

fn clean_finished_threads() {
    Addon::lock_threads().retain(|handle| !handle.is_finished());
}

fn clean_expired_cache() {
    let cache = &mut Addon::write_context().cache;
    let popup_data_cache_expiration_duration =
        Addon::read_config().max_popup_data_cache_expiration_duration;
    cache.popup_data_map.retain(|_, popup_data| {
        !is_cache_expired(popup_data_cache_expiration_duration, popup_data.cached_date)
    });
}

fn clean_expired_textures() {
    let texture_expiration_duration = Addon::read_config().max_texture_expiration_duration;
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
    info!(
        "[clean_expired_textures] Removed {} textures",
        removed_count
    );
}

pub fn load_fonts() {
    let bold_font_bytes = include_bytes!("../fonts/default_bold.ttf");
    let mut bold_font_path = fonts_dir();
    let _ = fs::create_dir(&bold_font_path);
    bold_font_path.push("../fonts/default_bold.ttf");
    if !bold_font_path.exists() {
        let mut bold_font_file =
            File::create(&bold_font_path).expect("Couldn't create a bold font file.");
        bold_font_file
            .write_all(bold_font_bytes)
            .expect("Couldn't write to bold font file.");
    }
    let entries = fs::read_dir(fonts_dir());
    if entries.is_err() {
        error!("[load_fonts] Couldn't load fonts");
        return;
    }

    let mut loaded_count = 0;
    let font_size = unsafe { sys::igGetFontSize() };

    for entry in entries.unwrap() {
        if entry.is_err() {
            error!("[load_fonts] Couldn't process entry");
            continue;
        }
        let entry = entry.unwrap();
        let path = entry.path();
        if path.is_dir() {
            continue;
        }
        if let (Some(extension), Some(filestem)) = (path.extension(), path.file_stem()) {
            if extension == "ttf" {
                let font_receive: RawFontReceive = font_receive!(|id, _font| {
                    debug!("Font loaded: {id}");
                });
                let config: &mut ImFontConfig = unsafe { &mut *std::ptr::null_mut() };
                let filename = filestem.to_string_lossy();
                add_font_from_file(
                    format!("IDP_{filename}"),
                    path,
                    font_size,
                    config,
                    font_receive,
                )
                .revert_on_unload();
                loaded_count += 1;
            }
        }
    }
    info!("[load_fonts] Loaded {} fonts", loaded_count);
}

fn preselect_fonts() {
    for font in unsafe { Font::get_all() } {
        unsafe {
            if let (Ok(font_name), Some(selected_bold_font_name)) = (
                font.name_raw().to_str(),
                Addon::read_config().selected_bold_font_name.clone(),
            ) {
                if font_name == selected_bold_font_name {
                    Addon::write_context().bold_font = Some(font);
                }
            }
        }
    }
}
